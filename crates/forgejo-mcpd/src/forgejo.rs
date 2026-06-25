// SPDX-License-Identifier: AGPL-3.0-or-later

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ForgejoClient {
    base_url: String,
    http: reqwest::Client,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryTarget {
    pub owner: String,
    pub repo: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoRepository {
    full_name: String,
    name: String,
    owner: ForgejoOwner,
    private: bool,
    #[serde(default)]
    empty: bool,
    #[serde(default)]
    archived: bool,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    default_branch: Option<String>,
    #[serde(default)]
    clone_url: Option<String>,
    #[serde(default)]
    ssh_url: Option<String>,
    #[serde(default)]
    updated_at: Option<String>,
    #[serde(default)]
    open_issues_count: Option<u64>,
    #[serde(default)]
    permissions: Option<ForgejoPermissions>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoOwner {
    login: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForgejoPermissions {
    #[serde(default)]
    pub admin: bool,
    #[serde(default)]
    pub push: bool,
    #[serde(default)]
    pub pull: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepositoryMetadata {
    pub full_name: String,
    pub owner: String,
    pub name: String,
    pub private: bool,
    pub empty: bool,
    pub archived: bool,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub clone_url: Option<String>,
    pub ssh_url: Option<String>,
    pub updated_at: Option<String>,
    pub open_issues_count: Option<u64>,
    pub permissions: Option<ForgejoPermissions>,
}

#[derive(Debug, thiserror::Error)]
pub enum ForgejoError {
    #[error("repository target must be owner/repository")]
    InvalidTarget,
    #[error("mapped principal has no Forgejo API token environment variable")]
    MissingTokenEnv,
    #[error("Forgejo API token environment variable is not set")]
    MissingToken,
    #[error("Forgejo returned {status}: {body}")]
    Api { status: StatusCode, body: String },
    #[error("Forgejo request failed: {0}")]
    Request(String),
}

impl ForgejoClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::new(),
        }
    }

    pub async fn repository_metadata(
        &self,
        token: &str,
        target: &RepositoryTarget,
    ) -> Result<(RepositoryMetadata, u16), ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}",
            self.base_url, target.owner, target.repo
        );
        let response = self
            .http
            .get(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let repository = response
            .json::<ForgejoRepository>()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        Ok((RepositoryMetadata::from(repository), status.as_u16()))
    }
}

impl RepositoryTarget {
    pub fn parse(value: &str) -> Result<Self, ForgejoError> {
        let mut parts = value.split('/');
        let owner = parts.next().unwrap_or_default().trim();
        let repo = parts.next().unwrap_or_default().trim();
        if owner.is_empty() || repo.is_empty() || parts.next().is_some() {
            return Err(ForgejoError::InvalidTarget);
        }
        Ok(Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        })
    }
}

impl From<ForgejoRepository> for RepositoryMetadata {
    fn from(value: ForgejoRepository) -> Self {
        Self {
            full_name: value.full_name,
            owner: value.owner.login,
            name: value.name,
            private: value.private,
            empty: value.empty,
            archived: value.archived,
            description: value.description,
            default_branch: value.default_branch,
            clone_url: value.clone_url,
            ssh_url: value.ssh_url,
            updated_at: value.updated_at,
            open_issues_count: value.open_issues_count,
            permissions: value.permissions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_owner_repo_target() {
        let target = RepositoryTarget::parse("rawholding/forgejo-keycloak-rust-mcp").unwrap();
        assert_eq!(target.owner, "rawholding");
        assert_eq!(target.repo, "forgejo-keycloak-rust-mcp");
        assert!(RepositoryTarget::parse("rawholding").is_err());
        assert!(RepositoryTarget::parse("rawholding/repo/extra").is_err());
    }

    #[test]
    fn maps_forgejo_repository_json_to_bounded_metadata() {
        let repository: ForgejoRepository = serde_json::from_value(serde_json::json!({
            "full_name": "rawholding/example",
            "name": "example",
            "owner": { "login": "rawholding" },
            "private": false,
            "empty": false,
            "archived": false,
            "description": "demo",
            "default_branch": "main",
            "clone_url": "https://forgejo.example/rawholding/example.git",
            "ssh_url": "git@forgejo.example:rawholding/example.git",
            "updated_at": "2026-06-25T10:00:00Z",
            "open_issues_count": 2,
            "permissions": { "admin": false, "push": false, "pull": true },
            "ignored": "not copied"
        }))
        .unwrap();
        let metadata = RepositoryMetadata::from(repository);
        assert_eq!(metadata.full_name, "rawholding/example");
        assert_eq!(metadata.permissions.unwrap().pull, true);
    }
}
