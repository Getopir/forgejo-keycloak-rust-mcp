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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberedTarget {
    pub repository: RepositoryTarget,
    pub number: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub limit: u32,
    pub next_cursor: Option<String>,
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

#[derive(Debug, Clone, Deserialize)]
struct ForgejoUser {
    login: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoIssue {
    number: u64,
    title: String,
    state: Option<String>,
    html_url: Option<String>,
    comments: Option<u64>,
    created_at: Option<String>,
    updated_at: Option<String>,
    pull_request: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssueSummary {
    pub number: u64,
    pub title: String,
    pub state: Option<String>,
    pub html_url: Option<String>,
    pub comments: Option<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub is_pull_request: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoPullRequest {
    number: u64,
    title: String,
    state: Option<String>,
    html_url: Option<String>,
    draft: Option<bool>,
    mergeable: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PullRequestSummary {
    pub number: u64,
    pub title: String,
    pub state: Option<String>,
    pub html_url: Option<String>,
    pub draft: Option<bool>,
    pub mergeable: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoPullRequestReview {
    id: i64,
    state: Option<String>,
    body: Option<String>,
    user: Option<ForgejoUser>,
    submitted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PullRequestReviewSummary {
    pub id: i64,
    pub state: Option<String>,
    pub body: Option<String>,
    pub user: Option<String>,
    pub submitted_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoRelease {
    id: i64,
    tag_name: String,
    name: Option<String>,
    draft: Option<bool>,
    prerelease: Option<bool>,
    published_at: Option<String>,
    html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReleaseSummary {
    pub id: i64,
    pub tag_name: String,
    pub name: Option<String>,
    pub draft: Option<bool>,
    pub prerelease: Option<bool>,
    pub published_at: Option<String>,
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoNotificationRepository {
    full_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoNotificationSubject {
    title: Option<String>,
    #[serde(rename = "type")]
    subject_type: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoNotification {
    id: String,
    unread: Option<bool>,
    pinned: Option<bool>,
    updated_at: Option<String>,
    repository: Option<ForgejoNotificationRepository>,
    subject: Option<ForgejoNotificationSubject>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationSummary {
    pub id: String,
    pub unread: Option<bool>,
    pub pinned: Option<bool>,
    pub updated_at: Option<String>,
    pub repository_full_name: Option<String>,
    pub subject_title: Option<String>,
    pub subject_type: Option<String>,
    pub subject_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoIssueComment {
    id: i64,
    html_url: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssueCommentSummary {
    pub id: i64,
    pub html_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ForgejoError {
    #[error("repository target must be owner/repository")]
    InvalidTarget,
    #[error("numbered target must be owner/repository#number")]
    InvalidNumberedTarget,
    #[error("cursor must be a positive page number")]
    InvalidCursor,
    #[error("issue comment body is required")]
    MissingCommentBody,
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

    pub async fn list_issues(
        &self,
        token: &str,
        target: &RepositoryTarget,
        state: Option<&str>,
        page: PageRequest,
    ) -> Result<(Page<IssueSummary>, u16), ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/issues",
            self.base_url, target.owner, target.repo
        );
        let mut query = page.query();
        query.push(("type", "issues".to_string()));
        if let Some(state) = state {
            query.push(("state", state.to_string()));
        }
        let (items, status) = self
            .get_json::<Vec<ForgejoIssue>>(token, url, &query)
            .await?;
        Ok((
            Page::new(items.into_iter().map(IssueSummary::from).collect(), page),
            status,
        ))
    }

    pub async fn create_issue_comment(
        &self,
        token: &str,
        target: &NumberedTarget,
        body: &str,
    ) -> Result<(IssueCommentSummary, u16), ForgejoError> {
        if body.trim().is_empty() {
            return Err(ForgejoError::MissingCommentBody);
        }
        let url = format!(
            "{}/api/v1/repos/{}/{}/issues/{}/comments",
            self.base_url, target.repository.owner, target.repository.repo, target.number
        );
        let response = self
            .http
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .json(&serde_json::json!({ "body": body }))
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let comment = response
            .json::<ForgejoIssueComment>()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        Ok((IssueCommentSummary::from(comment), status.as_u16()))
    }

    pub async fn list_pull_requests(
        &self,
        token: &str,
        target: &RepositoryTarget,
        state: Option<&str>,
        page: PageRequest,
    ) -> Result<(Page<PullRequestSummary>, u16), ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls",
            self.base_url, target.owner, target.repo
        );
        let mut query = page.query();
        if let Some(state) = state {
            query.push(("state", state.to_string()));
        }
        let (items, status) = self
            .get_json::<Vec<ForgejoPullRequest>>(token, url, &query)
            .await?;
        Ok((
            Page::new(
                items.into_iter().map(PullRequestSummary::from).collect(),
                page,
            ),
            status,
        ))
    }

    pub async fn list_pull_request_reviews(
        &self,
        token: &str,
        target: &NumberedTarget,
        page: PageRequest,
    ) -> Result<(Page<PullRequestReviewSummary>, u16), ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls/{}/reviews",
            self.base_url, target.repository.owner, target.repository.repo, target.number
        );
        let query = page.query();
        let (items, status) = self
            .get_json::<Vec<ForgejoPullRequestReview>>(token, url, &query)
            .await?;
        Ok((
            Page::new(
                items
                    .into_iter()
                    .map(PullRequestReviewSummary::from)
                    .collect(),
                page,
            ),
            status,
        ))
    }

    pub async fn list_releases(
        &self,
        token: &str,
        target: &RepositoryTarget,
        page: PageRequest,
    ) -> Result<(Page<ReleaseSummary>, u16), ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/releases",
            self.base_url, target.owner, target.repo
        );
        let query = page.query();
        let (items, status) = self
            .get_json::<Vec<ForgejoRelease>>(token, url, &query)
            .await?;
        Ok((
            Page::new(items.into_iter().map(ReleaseSummary::from).collect(), page),
            status,
        ))
    }

    pub async fn list_notifications(
        &self,
        token: &str,
        status_types: Option<&str>,
        page: PageRequest,
    ) -> Result<(Page<NotificationSummary>, u16), ForgejoError> {
        let url = format!("{}/api/v1/notifications", self.base_url);
        let mut query = page.query();
        if let Some(status_types) = status_types {
            query.push(("status-types", status_types.to_string()));
        }
        let (items, status) = self
            .get_json::<Vec<ForgejoNotification>>(token, url, &query)
            .await?;
        Ok((
            Page::new(
                items.into_iter().map(NotificationSummary::from).collect(),
                page,
            ),
            status,
        ))
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        token: &str,
        url: String,
        query: &[(&'static str, String)],
    ) -> Result<(T, u16), ForgejoError> {
        let response = self
            .http
            .get(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .query(query)
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let value = response
            .json::<T>()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        Ok((value, status.as_u16()))
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

impl NumberedTarget {
    pub fn parse(value: &str) -> Result<Self, ForgejoError> {
        if let Some((repo_target, number)) = value.rsplit_once('#') {
            return Ok(Self {
                repository: RepositoryTarget::parse(repo_target)?,
                number: parse_positive_number(number)?,
            });
        }
        let mut parts = value.split('/');
        let owner = parts.next().unwrap_or_default().trim();
        let repo = parts.next().unwrap_or_default().trim();
        let number = parts.next().unwrap_or_default().trim();
        if owner.is_empty() || repo.is_empty() || number.is_empty() || parts.next().is_some() {
            return Err(ForgejoError::InvalidNumberedTarget);
        }
        Ok(Self {
            repository: RepositoryTarget {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            number: parse_positive_number(number)?,
        })
    }
}

impl PageRequest {
    pub fn from_cursor(
        cursor: Option<&str>,
        requested_limit: Option<u32>,
        max_limit: u32,
    ) -> Result<Self, ForgejoError> {
        let page = match cursor {
            Some(cursor) => parse_positive_page(cursor)?,
            None => 1,
        };
        let requested = requested_limit.unwrap_or(max_limit).max(1);
        Ok(Self {
            page,
            limit: requested.min(max_limit.max(1)),
        })
    }

    fn query(self) -> Vec<(&'static str, String)> {
        vec![
            ("page", self.page.to_string()),
            ("limit", self.limit.to_string()),
        ]
    }
}

impl<T> Page<T> {
    fn new(items: Vec<T>, request: PageRequest) -> Self {
        let next_cursor = if items.len() >= request.limit as usize {
            Some((request.page + 1).to_string())
        } else {
            None
        };
        Self {
            items,
            limit: request.limit,
            next_cursor,
        }
    }
}

fn parse_positive_number(value: &str) -> Result<u64, ForgejoError> {
    let number = value
        .trim()
        .parse::<u64>()
        .map_err(|_| ForgejoError::InvalidNumberedTarget)?;
    if number == 0 {
        return Err(ForgejoError::InvalidNumberedTarget);
    }
    Ok(number)
}

fn parse_positive_page(value: &str) -> Result<u32, ForgejoError> {
    let page = value
        .trim()
        .parse::<u32>()
        .map_err(|_| ForgejoError::InvalidCursor)?;
    if page == 0 {
        return Err(ForgejoError::InvalidCursor);
    }
    Ok(page)
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

impl From<ForgejoIssue> for IssueSummary {
    fn from(value: ForgejoIssue) -> Self {
        Self {
            number: value.number,
            title: value.title,
            state: value.state,
            html_url: value.html_url,
            comments: value.comments,
            created_at: value.created_at,
            updated_at: value.updated_at,
            is_pull_request: value.pull_request.is_some(),
        }
    }
}

impl From<ForgejoPullRequest> for PullRequestSummary {
    fn from(value: ForgejoPullRequest) -> Self {
        Self {
            number: value.number,
            title: value.title,
            state: value.state,
            html_url: value.html_url,
            draft: value.draft,
            mergeable: value.mergeable,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<ForgejoPullRequestReview> for PullRequestReviewSummary {
    fn from(value: ForgejoPullRequestReview) -> Self {
        Self {
            id: value.id,
            state: value.state,
            body: value.body,
            user: value.user.and_then(|user| user.login),
            submitted_at: value.submitted_at,
        }
    }
}

impl From<ForgejoRelease> for ReleaseSummary {
    fn from(value: ForgejoRelease) -> Self {
        Self {
            id: value.id,
            tag_name: value.tag_name,
            name: value.name,
            draft: value.draft,
            prerelease: value.prerelease,
            published_at: value.published_at,
            html_url: value.html_url,
        }
    }
}

impl From<ForgejoNotification> for NotificationSummary {
    fn from(value: ForgejoNotification) -> Self {
        let subject = value.subject;
        Self {
            id: value.id,
            unread: value.unread,
            pinned: value.pinned,
            updated_at: value.updated_at,
            repository_full_name: value.repository.and_then(|repository| repository.full_name),
            subject_title: subject.as_ref().and_then(|subject| subject.title.clone()),
            subject_type: subject
                .as_ref()
                .and_then(|subject| subject.subject_type.clone()),
            subject_url: subject.and_then(|subject| subject.url),
        }
    }
}

impl From<ForgejoIssueComment> for IssueCommentSummary {
    fn from(value: ForgejoIssueComment) -> Self {
        Self {
            id: value.id,
            html_url: value.html_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
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
    fn parses_numbered_targets() {
        let target = NumberedTarget::parse("rawholding/example#42").unwrap();
        assert_eq!(target.repository.owner, "rawholding");
        assert_eq!(target.repository.repo, "example");
        assert_eq!(target.number, 42);

        let target = NumberedTarget::parse("rawholding/example/7").unwrap();
        assert_eq!(target.number, 7);
        assert!(NumberedTarget::parse("rawholding/example#0").is_err());
        assert!(NumberedTarget::parse("rawholding/example/not-a-number").is_err());
    }

    #[test]
    fn caps_page_limits_and_uses_cursor_as_page_token() {
        let request = PageRequest::from_cursor(Some("3"), Some(500), 50).unwrap();
        assert_eq!(request.page, 3);
        assert_eq!(request.limit, 50);
        assert!(PageRequest::from_cursor(Some("0"), None, 50).is_err());
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
