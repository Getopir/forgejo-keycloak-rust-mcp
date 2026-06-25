use policy::RiskClass;
use serde::{Deserialize, Serialize};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_event_does_not_report_sensitive_marker_for_normal_data() {
        let event = AuditEvent {
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
        };
        assert!(!event.contains_sensitive_material());
    }
}
