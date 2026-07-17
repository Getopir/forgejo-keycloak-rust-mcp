// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct AgentRateLimiter {
    inner: Arc<Mutex<HashMap<AgentKey, Bucket>>>,
    capacity: u32,
    window: Duration,
    max_agents: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AgentKey {
    issuer: String,
    subject: String,
}

#[derive(Debug)]
struct Bucket {
    tokens: f64,
    updated_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitDecision {
    Allowed,
    Limited { retry_after_seconds: u64 },
}

impl AgentRateLimiter {
    pub fn new(capacity: u32, window: Duration, max_agents: usize) -> Self {
        assert!(capacity > 0, "rate-limit capacity must be positive");
        assert!(!window.is_zero(), "rate-limit window must be positive");
        assert!(max_agents > 0, "tracked-agent limit must be positive");
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            capacity,
            window,
            max_agents,
        }
    }

    pub fn check(&self, issuer: &str, subject: &str) -> RateLimitDecision {
        self.check_at(issuer, subject, Instant::now())
    }

    fn check_at(&self, issuer: &str, subject: &str, now: Instant) -> RateLimitDecision {
        let key = AgentKey {
            issuer: issuer.trim_end_matches('/').to_string(),
            subject: subject.to_string(),
        };
        let mut buckets = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if !buckets.contains_key(&key) && buckets.len() >= self.max_agents {
            let idle_limit = self.window.saturating_mul(2);
            buckets.retain(|_, bucket| now.duration_since(bucket.updated_at) < idle_limit);
            if buckets.len() >= self.max_agents {
                return RateLimitDecision::Limited {
                    retry_after_seconds: self.window.as_secs().max(1),
                };
            }
        }

        let bucket = buckets.entry(key).or_insert_with(|| Bucket {
            tokens: f64::from(self.capacity),
            updated_at: now,
        });
        let elapsed = now.duration_since(bucket.updated_at).as_secs_f64();
        let refill_per_second = f64::from(self.capacity) / self.window.as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * refill_per_second).min(f64::from(self.capacity));
        bucket.updated_at = now;

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            return RateLimitDecision::Allowed;
        }

        let retry_after_seconds =
            ((1.0 - bucket.tokens) / refill_per_second).ceil().max(1.0) as u64;
        RateLimitDecision::Limited {
            retry_after_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_each_agent_independently_and_refills() {
        let limiter = AgentRateLimiter::new(2, Duration::from_secs(60), 10);
        let start = Instant::now();

        assert_eq!(
            limiter.check_at("https://issuer/", "agent-a", start),
            RateLimitDecision::Allowed
        );
        assert_eq!(
            limiter.check_at("https://issuer", "agent-a", start),
            RateLimitDecision::Allowed
        );
        assert_eq!(
            limiter.check_at("https://issuer", "agent-a", start),
            RateLimitDecision::Limited {
                retry_after_seconds: 30
            }
        );
        assert_eq!(
            limiter.check_at("https://issuer", "agent-b", start),
            RateLimitDecision::Allowed
        );
        assert_eq!(
            limiter.check_at("https://issuer", "agent-a", start + Duration::from_secs(30)),
            RateLimitDecision::Allowed
        );
    }

    #[test]
    fn tracking_bound_fails_closed_until_idle_entries_expire() {
        let limiter = AgentRateLimiter::new(1, Duration::from_secs(10), 1);
        let start = Instant::now();

        assert_eq!(
            limiter.check_at("https://issuer", "agent-a", start),
            RateLimitDecision::Allowed
        );
        assert_eq!(
            limiter.check_at("https://issuer", "agent-b", start),
            RateLimitDecision::Limited {
                retry_after_seconds: 10
            }
        );
        assert_eq!(
            limiter.check_at("https://issuer", "agent-b", start + Duration::from_secs(20)),
            RateLimitDecision::Allowed
        );
    }
}
