// SPDX-License-Identifier: AGPL-3.0-or-later

use audit::PrincipalType;
use identity::Principal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct PrincipalMapFile {
    pub mappings: Vec<PrincipalMapping>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrincipalMapping {
    pub issuer: String,
    pub subject: String,
    pub forgejo_login: String,
    #[serde(default)]
    pub forgejo_user_id: Option<i64>,
    #[serde(default)]
    pub forgejo_email: Option<String>,
    #[serde(default)]
    pub forgejo_full_name: Option<String>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub principal_type: PrincipalKind,
    #[serde(default)]
    pub api_token_env: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrincipalKind {
    Human,
    Agent,
    #[default]
    Unknown,
}

impl From<PrincipalKind> for PrincipalType {
    fn from(value: PrincipalKind) -> Self {
        match value {
            PrincipalKind::Human => PrincipalType::Human,
            PrincipalKind::Agent => PrincipalType::Agent,
            PrincipalKind::Unknown => PrincipalType::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrincipalMapper {
    mappings: BTreeMap<(String, String), PrincipalMapping>,
}

#[derive(Debug, thiserror::Error)]
pub enum PrincipalMapError {
    #[error("principal map load failed: {0}")]
    Load(String),
    #[error("principal is not mapped to a Forgejo account")]
    NotMapped,
    #[error("principal mapping is disabled")]
    Disabled,
}

#[derive(Debug, Clone, Serialize)]
pub struct DelegatedHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct TrustedHeaderConfig {
    user_header: String,
    email_header: Option<String>,
    full_name_header: Option<String>,
}

fn default_enabled() -> bool {
    true
}

impl PrincipalMapper {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, PrincipalMapError> {
        let text = std::fs::read_to_string(path)
            .map_err(|err| PrincipalMapError::Load(err.to_string()))?;
        Self::from_json_str(&text)
    }

    pub fn from_json_str(text: &str) -> Result<Self, PrincipalMapError> {
        let file: PrincipalMapFile =
            serde_json::from_str(text).map_err(|err| PrincipalMapError::Load(err.to_string()))?;
        let mappings = file
            .mappings
            .into_iter()
            .map(|mapping| {
                (
                    (
                        mapping.issuer.trim_end_matches('/').to_string(),
                        mapping.subject.clone(),
                    ),
                    mapping,
                )
            })
            .collect();
        Ok(Self { mappings })
    }

    pub fn resolve(&self, principal: &Principal) -> Result<&PrincipalMapping, PrincipalMapError> {
        let key = (
            principal.issuer.trim_end_matches('/').to_string(),
            principal.subject.clone(),
        );
        let mapping = self
            .mappings
            .get(&key)
            .ok_or(PrincipalMapError::NotMapped)?;
        if !mapping.enabled {
            return Err(PrincipalMapError::Disabled);
        }
        Ok(mapping)
    }
}

impl TrustedHeaderConfig {
    pub fn new(
        user_header: impl Into<String>,
        email_header: Option<String>,
        full_name_header: Option<String>,
    ) -> Self {
        Self {
            user_header: user_header.into(),
            email_header,
            full_name_header,
        }
    }

    pub fn delegated_headers(&self, mapping: &PrincipalMapping) -> Vec<DelegatedHeader> {
        let mut headers = vec![DelegatedHeader {
            name: self.user_header.clone(),
            value: mapping.forgejo_login.clone(),
        }];
        if let (Some(header), Some(email)) = (&self.email_header, &mapping.forgejo_email) {
            headers.push(DelegatedHeader {
                name: header.clone(),
                value: email.clone(),
            });
        }
        if let (Some(header), Some(full_name)) =
            (&self.full_name_header, &mapping.forgejo_full_name)
        {
            headers.push(DelegatedHeader {
                name: header.clone(),
                value: full_name.clone(),
            });
        }
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn principal(subject: &str) -> Principal {
        Principal {
            issuer: "https://sso.example/realms/agents".to_string(),
            subject: subject.to_string(),
            oauth_client: Some("agent".to_string()),
            scopes: BTreeSet::new(),
            preferred_username: None,
        }
    }

    #[test]
    fn maps_issuer_subject_to_forgejo_login() {
        let mapper = PrincipalMapper::from_json_str(
            r#"{
              "mappings": [{
                "issuer": "https://sso.example/realms/agents/",
                "subject": "sub-1",
                "forgejo_login": "agent-one",
                "principal_type": "agent",
                "api_token_env": "FORGEJO_AGENT_ONE_TOKEN"
              }]
            }"#,
        )
        .unwrap();
        let mapping = mapper.resolve(&principal("sub-1")).unwrap();
        assert_eq!(mapping.forgejo_login, "agent-one");
        assert_eq!(
            mapping.api_token_env.as_deref(),
            Some("FORGEJO_AGENT_ONE_TOKEN")
        );
    }

    #[test]
    fn rejects_disabled_mapping() {
        let mapper = PrincipalMapper::from_json_str(
            r#"{
              "mappings": [{
                "issuer": "https://sso.example/realms/agents",
                "subject": "sub-1",
                "forgejo_login": "agent-one",
                "enabled": false
              }]
            }"#,
        )
        .unwrap();
        assert!(matches!(
            mapper.resolve(&principal("sub-1")),
            Err(PrincipalMapError::Disabled)
        ));
    }

    #[test]
    fn builds_trusted_headers_from_mapping() {
        let config = TrustedHeaderConfig::new(
            "X-WEBAUTH-USER",
            Some("X-WEBAUTH-EMAIL".to_string()),
            Some("X-WEBAUTH-FULLNAME".to_string()),
        );
        let mapping = PrincipalMapping {
            issuer: "issuer".to_string(),
            subject: "subject".to_string(),
            forgejo_login: "agent-one".to_string(),
            forgejo_user_id: Some(1),
            forgejo_email: Some("agent@example.org".to_string()),
            forgejo_full_name: Some("Agent One".to_string()),
            enabled: true,
            principal_type: PrincipalKind::Agent,
            api_token_env: None,
        };
        let headers = config.delegated_headers(&mapping);
        assert_eq!(headers[0].name, "X-WEBAUTH-USER");
        assert_eq!(headers[0].value, "agent-one");
        assert_eq!(headers[1].name, "X-WEBAUTH-EMAIL");
        assert_eq!(headers[2].name, "X-WEBAUTH-FULLNAME");
    }
}
