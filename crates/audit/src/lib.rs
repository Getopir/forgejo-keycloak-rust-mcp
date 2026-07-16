// SPDX-License-Identifier: AGPL-3.0-or-later

use policy::RiskClass;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub request_id: Uuid,
    pub issuer: String,
    pub subject: String,
    pub oauth_client: Option<String>,
    pub principal_type: PrincipalType,
    pub forgejo_user_id: Option<i64>,
    pub forgejo_login: Option<String>,
    pub tool: String,
    pub target: Option<String>,
    pub risk: RiskClass,
    pub decision: AuditDecision,
    pub approval_id: Option<String>,
    pub forgejo_status: Option<u16>,
    pub duration_ms: u64,
    pub response_bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrincipalType {
    Human,
    Agent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditDecision {
    Allow,
    Deny,
}

impl AuditEvent {
    pub fn contains_sensitive_material(&self) -> bool {
        let serialized = serde_json::to_string(self)
            .unwrap_or_default()
            .to_lowercase();
        [
            "authorization",
            "bearer ",
            "access_token",
            "refresh_token",
            "pat_",
        ]
        .iter()
        .any(|needle| serialized.contains(needle))
    }
}

#[derive(Debug)]
pub struct JsonlAuditSink {
    file: Mutex<File>,
}

impl JsonlAuditSink {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut options = OpenOptions::new();
        options.create(true).append(true);
        #[cfg(unix)]
        options.mode(0o600);
        let file = options.open(path)?;
        Ok(Self {
            file: Mutex::new(file),
        })
    }

    pub fn record(&self, event: &AuditEvent) -> io::Result<()> {
        let mut record = serde_json::to_vec(event).map_err(io::Error::other)?;
        record.push(b'\n');
        let mut file = self
            .file
            .lock()
            .map_err(|_| io::Error::other("audit sink lock poisoned"))?;
        file.write_all(&record)?;
        file.flush()?;
        file.sync_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn audit_event_does_not_report_sensitive_marker_for_normal_data() {
        let event = test_event();
        assert!(!event.contains_sensitive_material());
    }

    #[test]
    fn jsonl_sink_appends_complete_records() {
        let path = std::env::temp_dir().join(format!("audit-{}.jsonl", Uuid::now_v7()));
        let sink = JsonlAuditSink::open(&path).unwrap();
        sink.record(&test_event()).unwrap();
        sink.record(&test_event()).unwrap();

        let contents = fs::read_to_string(&path).unwrap();
        let records = contents
            .lines()
            .map(serde_json::from_str::<AuditEvent>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(records.len(), 2);
        fs::remove_file(path).unwrap();
    }

    fn test_event() -> AuditEvent {
        AuditEvent {
            request_id: Uuid::nil(),
            issuer: "https://sso.example/realms/company".to_string(),
            subject: "sub".to_string(),
            oauth_client: Some("agent-client".to_string()),
            principal_type: PrincipalType::Agent,
            forgejo_user_id: Some(42),
            forgejo_login: Some("bot-agent".to_string()),
            tool: "gateway_probe".to_string(),
            target: None,
            risk: RiskClass::ReadPrivate,
            decision: AuditDecision::Allow,
            approval_id: None,
            forgejo_status: None,
            duration_ms: 1,
            response_bytes: 100,
        }
    }
}
