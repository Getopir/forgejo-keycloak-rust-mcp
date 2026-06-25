// SPDX-License-Identifier: AGPL-3.0-or-later

use audit::PrincipalType;
use axum::http::HeaderMap;
use identity::Principal;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
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
    #[error("principal map contains duplicate issuer/subject entry")]
    DuplicateMapping,
    #[error("principal map entry has an empty required field: {0}")]
    EmptyField(&'static str),
    #[error("api_token_env must contain only ASCII letters, digits, and underscore")]
    InvalidTokenEnv,
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
        let mut mappings = BTreeMap::new();
        for mut mapping in file.mappings {
            validate_required("issuer", &mapping.issuer)?;
            validate_required("subject", &mapping.subject)?;
            validate_required("forgejo_login", &mapping.forgejo_login)?;
            if let Some(token_env) = mapping.api_token_env.as_deref() {
                validate_token_env(token_env)?;
            }
            mapping.issuer = mapping.issuer.trim_end_matches('/').to_string();
            mapping.subject = mapping.subject.trim().to_string();
            mapping.forgejo_login = mapping.forgejo_login.trim().to_string();
            let key = (mapping.issuer.clone(), mapping.subject.clone());
            if mappings.insert(key, mapping).is_some() {
                return Err(PrincipalMapError::DuplicateMapping);
            }
        }
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

fn validate_required(field: &'static str, value: &str) -> Result<(), PrincipalMapError> {
    if value.trim().is_empty() {
        return Err(PrincipalMapError::EmptyField(field));
    }
    Ok(())
}

fn validate_token_env(value: &str) -> Result<(), PrincipalMapError> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(PrincipalMapError::InvalidTokenEnv);
    }
    Ok(())
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

    pub fn spoofed_header(&self, headers: &HeaderMap) -> Option<String> {
        self.header_names()
            .into_iter()
            .find(|name| headers.contains_key(*name))
            .map(ToString::to_string)
    }

    fn header_names(&self) -> BTreeSet<&str> {
        let mut names = BTreeSet::new();
        names.insert(self.user_header.as_str());
        if let Some(header) = &self.email_header {
            names.insert(header.as_str());
        }
        if let Some(header) = &self.full_name_header {
            names.insert(header.as_str());
        }
        names
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
    fn rejects_duplicate_mapping() {
        let err = PrincipalMapper::from_json_str(
            r#"{
              "mappings": [
                {"issuer": "https://sso.example/realms/agents", "subject": "sub-1", "forgejo_login": "agent-one"},
                {"issuer": "https://sso.example/realms/agents/", "subject": "sub-1", "forgejo_login": "agent-two"}
              ]
            }"#,
        )
        .unwrap_err();
        assert!(matches!(err, PrincipalMapError::DuplicateMapping));
    }

    #[test]
    fn rejects_invalid_token_env_name() {
        let err = PrincipalMapper::from_json_str(
            r#"{
              "mappings": [{
                "issuer": "https://sso.example/realms/agents",
                "subject": "sub-1",
                "forgejo_login": "agent-one",
                "api_token_env": "FORGEJO TOKEN"
              }]
            }"#,
        )
        .unwrap_err();
        assert!(matches!(err, PrincipalMapError::InvalidTokenEnv));
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

    #[test]
    fn detects_incoming_trusted_header_spoof() {
        let config = TrustedHeaderConfig::new("X-WEBAUTH-USER", None, None);
        let mut headers = HeaderMap::new();
        headers.insert("X-WEBAUTH-USER", "attacker".parse().unwrap());
        assert_eq!(
            config.spoofed_header(&headers),
            Some("X-WEBAUTH-USER".to_string())
        );
    }
}
