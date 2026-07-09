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
    pub resource_uri: String,
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
    pub resource_uri: String,
    pub number: u64,
    pub title: String,
    pub state: Option<String>,
    pub html_url: Option<String>,
    pub comments: Option<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub is_pull_request: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateIssueOptions {
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub assignee: Option<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    #[serde(default)]
    pub labels: Vec<i64>,
    #[serde(default)]
    pub milestone: Option<i64>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateIssueResult {
    pub resource_uri: String,
    pub issue: IssueSummary,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoPullRequest {
    #[serde(default)]
    number: Option<u64>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    html_url: Option<String>,
    #[serde(default)]
    draft: Option<bool>,
    #[serde(default)]
    mergeable: Option<bool>,
    #[serde(default)]
    merged: Option<bool>,
    #[serde(default)]
    merge_commit_sha: Option<String>,
    #[serde(default)]
    head: Option<ForgejoPullRequestBranch>,
    #[serde(default)]
    base: Option<ForgejoPullRequestBranch>,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoCombinedStatus {
    #[serde(default)]
    sha: Option<String>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    total_count: Option<i64>,
    #[serde(default)]
    statuses: Vec<ForgejoCommitStatus>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoCommitStatus {
    #[serde(default)]
    context: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    target_url: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoPullRequestBranch {
    #[serde(default, rename = "ref")]
    ref_name: Option<String>,
    #[serde(default)]
    sha: Option<String>,
    #[serde(default)]
    label: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PullRequestBranchSummary {
    #[serde(rename = "ref")]
    pub ref_name: Option<String>,
    pub sha: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PullRequestSummary {
    pub resource_uri: String,
    pub number: u64,
    pub url: Option<String>,
    pub title: String,
    pub state: Option<String>,
    pub html_url: Option<String>,
    pub draft: Option<bool>,
    pub mergeable: Option<bool>,
    pub merged: Option<bool>,
    pub merge_commit_sha: Option<String>,
    pub head: Option<PullRequestBranchSummary>,
    pub base: Option<PullRequestBranchSummary>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommitStatusReadiness {
    pub sha: String,
    pub state: Option<String>,
    pub total_count: Option<i64>,
    pub statuses: Vec<CommitStatusSummary>,
    pub failing_contexts: Vec<CommitStatusSummary>,
    pub pending_contexts: Vec<CommitStatusSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommitStatusSummary {
    pub context: Option<String>,
    pub status: Option<String>,
    pub target_url: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PullRequestStaleClassification {
    pub is_stale: bool,
    pub reason: String,
    pub commit_count: usize,
    pub changed_file_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct PullRequestReadback {
    pub pull_request: PullRequestSummary,
    pub number: u64,
    pub head_sha: Option<String>,
    pub state: Option<String>,
    pub merged: Option<bool>,
    pub merge_commit_sha: Option<String>,
    pub required_check_state: Option<CommitStatusReadiness>,
    pub branch_ref_exists: Option<bool>,
    pub stale: PullRequestStaleClassification,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatePullRequestOptions {
    pub head: String,
    pub base: String,
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub draft: Option<bool>,
    #[serde(default)]
    pub assignee: Option<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    #[serde(default)]
    pub reviewers: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePullRequestResult {
    pub resource_uri: String,
    pub pull_request: PullRequestSummary,
    pub readback: PullRequestReadback,
    pub requested_reviewers: Vec<String>,
    pub reviewer_request_status: Option<u16>,
    pub reviewer_request_error: Option<String>,
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
    pub resource_uri: String,
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
    pub resource_uri: String,
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
    pub resource_uri: String,
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
    pub resource_uri: String,
    pub id: i64,
    pub html_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoWikiPage {
    title: String,
    html_url: Option<String>,
    sub_url: Option<String>,
    content_base64: Option<String>,
    commit_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
struct ForgejoWikiPageMetaData {
    title: String,
    html_url: Option<String>,
    sub_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WikiPageSummary {
    pub resource_uri: String,
    pub title: String,
    pub html_url: Option<String>,
    pub sub_url: Option<String>,
    pub commit_count: Option<i64>,
    pub has_content_base64: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct WikiPageMetaSummary {
    pub resource_uri: String,
    pub title: String,
    pub html_url: Option<String>,
    pub sub_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WikiPageOptions {
    pub title: String,
    pub content_base64: String,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateReleaseOptions {
    pub tag_name: String,
    #[serde(default)]
    pub target_commitish: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub draft: Option<bool>,
    #[serde(default)]
    pub prerelease: Option<bool>,
    #[serde(default)]
    pub hide_archive_links: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateReleaseResult {
    pub resource_uri: String,
    pub release: ReleaseSummary,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MergePullRequestOptions {
    #[serde(default = "default_merge_method")]
    pub method: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub delete_branch_after_merge: Option<bool>,
    #[serde(default)]
    pub force_merge: Option<bool>,
    #[serde(default)]
    pub head_commit_id: Option<String>,
    #[serde(default)]
    pub status_check_wait_seconds: Option<u64>,
    #[serde(default)]
    pub status_check_poll_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MergePullRequestResult {
    pub resource_uri: String,
    pub merged: bool,
    pub method: String,
    pub forgejo_response: serde_json::Value,
    pub readback: PullRequestReadback,
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
    #[error("issue options body must be JSON when supplied")]
    InvalidIssueOptions,
    #[error("issue title is required")]
    MissingIssueTitle,
    #[error("wiki page options body must be JSON when supplied")]
    InvalidWikiPageOptions,
    #[error("wiki page title is required")]
    MissingWikiPageTitle,
    #[error("wiki page content_base64 is required")]
    MissingWikiPageContent,
    #[error("release options body must be JSON when supplied")]
    InvalidReleaseOptions,
    #[error("release tag_name is required")]
    MissingReleaseTag,
    #[error("pull request options body must be JSON when supplied")]
    InvalidPullRequestOptions,
    #[error("pull request head, base, and title are required")]
    MissingPullRequestFields,
    #[error(
        "pull request creation succeeded but readback found no open PR for repo={repo} head={head} base={base} title={title}"
    )]
    PullRequestReadbackNoMatch {
        repo: String,
        head: String,
        base: String,
        title: String,
    },
    #[error(
        "pull request creation readback is ambiguous for repo={repo} head={head} base={base} title={title}; candidates={candidates}"
    )]
    PullRequestReadbackAmbiguous {
        repo: String,
        head: String,
        base: String,
        title: String,
        candidates: String,
    },
    #[error("pull request #{number} is not stale: {reason}")]
    PullRequestNotStale { number: u64, reason: String },
    #[error("required checks did not pass for head {sha}: {contexts}")]
    RequiredChecksFailed { sha: String, contexts: String },
    #[error("merge options body must be JSON when supplied")]
    InvalidMergeOptions,
    #[error("unsupported merge method")]
    UnsupportedMergeMethod,
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
        Ok((
            RepositoryMetadata::from_repository(repository, target),
            status.as_u16(),
        ))
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
            Page::new(
                items
                    .into_iter()
                    .map(|issue| IssueSummary::from_issue(issue, target))
                    .collect(),
                page,
            ),
            status,
        ))
    }

    pub async fn create_issue(
        &self,
        token: &str,
        target: &RepositoryTarget,
        options: &CreateIssueOptions,
    ) -> Result<(CreateIssueResult, u16), ForgejoError> {
        options.validate()?;
        let url = format!(
            "{}/api/v1/repos/{}/{}/issues",
            self.base_url, target.owner, target.repo
        );
        let response = self
            .http
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .json(&options.to_forgejo_payload())
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let issue = response
            .json::<ForgejoIssue>()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let issue = IssueSummary::from_issue(issue, target);
        Ok((
            CreateIssueResult {
                resource_uri: issue.resource_uri.clone(),
                issue,
            },
            status.as_u16(),
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
        Ok((
            IssueCommentSummary::from_comment(comment, target),
            status.as_u16(),
        ))
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
                items
                    .into_iter()
                    .filter_map(|pull| PullRequestSummary::from_pull_request(pull, target))
                    .collect(),
                page,
            ),
            status,
        ))
    }

    pub async fn get_pull_request(
        &self,
        token: &str,
        target: &NumberedTarget,
    ) -> Result<(PullRequestSummary, u16), ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls/{}",
            self.base_url, target.repository.owner, target.repository.repo, target.number
        );
        let (pull, status) = self.get_json::<ForgejoPullRequest>(token, url, &[]).await?;
        let pull =
            PullRequestSummary::from_pull_request(pull, &target.repository).ok_or_else(|| {
                ForgejoError::Request("Forgejo returned a sparse pull-request readback".to_string())
            })?;
        Ok((pull, status))
    }

    pub async fn get_pull_request_by_branch(
        &self,
        token: &str,
        target: &RepositoryTarget,
        base: &str,
        head: &str,
    ) -> Result<(PullRequestSummary, u16), ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls/{}/{}",
            self.base_url,
            target.owner,
            target.repo,
            percent_encode_path_segment(base),
            percent_encode_path_segment(head)
        );
        let (pull, status) = self.get_json::<ForgejoPullRequest>(token, url, &[]).await?;
        let pull = PullRequestSummary::from_pull_request(pull, target).ok_or_else(|| {
            ForgejoError::Request(
                "Forgejo returned a sparse pull-request branch readback".to_string(),
            )
        })?;
        Ok((pull, status))
    }

    pub async fn create_pull_request(
        &self,
        token: &str,
        target: &RepositoryTarget,
        options: &CreatePullRequestOptions,
    ) -> Result<(CreatePullRequestResult, u16), ForgejoError> {
        options.validate()?;
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls",
            self.base_url, target.owner, target.repo
        );
        let response = self
            .http
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .json(&options.to_forgejo_payload())
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let raw_body = response.text().await.unwrap_or_default();
        let pull = match serde_json::from_str::<ForgejoPullRequest>(&raw_body)
            .ok()
            .and_then(|pull| PullRequestSummary::from_pull_request(pull, target))
        {
            Some(pull) => pull,
            None => {
                self.readback_created_pull_request(token, target, options)
                    .await?
            }
        };
        let mut reviewer_request_status = None;
        let mut reviewer_request_error = None;
        if !options.reviewers.is_empty() {
            let review_url = format!(
                "{}/api/v1/repos/{}/{}/pulls/{}/requested_reviewers",
                self.base_url, target.owner, target.repo, pull.number
            );
            let review_response = self
                .http
                .post(review_url)
                .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
                .json(&serde_json::json!({ "reviewers": &options.reviewers }))
                .send()
                .await
                .map_err(|err| ForgejoError::Request(err.to_string()))?;
            let review_status = review_response.status();
            reviewer_request_status = Some(review_status.as_u16());
            if !review_status.is_success() {
                reviewer_request_error = Some(review_response.text().await.unwrap_or_default());
            }
        }
        let readback = self
            .pull_request_readback_for_pull(token, target, pull)
            .await?;
        let pull = readback.pull_request.clone();
        Ok((
            CreatePullRequestResult {
                resource_uri: pull.resource_uri.clone(),
                pull_request: pull,
                readback,
                requested_reviewers: options.reviewers.clone(),
                reviewer_request_status,
                reviewer_request_error,
            },
            status.as_u16(),
        ))
    }

    async fn readback_created_pull_request(
        &self,
        token: &str,
        target: &RepositoryTarget,
        options: &CreatePullRequestOptions,
    ) -> Result<PullRequestSummary, ForgejoError> {
        match self
            .get_pull_request_by_branch(token, target, &options.base, &options.head)
            .await
        {
            Ok((pull, _)) => return Ok(pull),
            Err(ForgejoError::Api { status, .. }) if status == StatusCode::NOT_FOUND => {}
            Err(_) => {}
        }
        let mut candidates = Vec::new();
        let mut page = 1;
        loop {
            let (pulls, _) = self
                .list_pull_requests(token, target, Some("open"), PageRequest { page, limit: 50 })
                .await?;
            let item_count = pulls.items.len();
            let has_next = pulls.next_cursor.is_some();
            candidates.extend(pulls.items);
            if !has_next || item_count == 0 {
                break;
            }
            page += 1;
        }
        select_created_pull_request(candidates, target, options)
    }

    pub async fn pull_request_readback(
        &self,
        token: &str,
        target: &NumberedTarget,
    ) -> Result<(PullRequestReadback, u16), ForgejoError> {
        let (pull, status) = self.get_pull_request(token, target).await?;
        let readback = self
            .pull_request_readback_for_pull(token, &target.repository, pull)
            .await?;
        Ok((readback, status))
    }

    async fn pull_request_readback_for_pull(
        &self,
        token: &str,
        target: &RepositoryTarget,
        pull: PullRequestSummary,
    ) -> Result<PullRequestReadback, ForgejoError> {
        let numbered = NumberedTarget {
            repository: target.clone(),
            number: pull.number,
        };
        let head_sha = pull.head.as_ref().and_then(|head| head.sha.clone());
        let branch_ref_exists = match pull
            .head
            .as_ref()
            .and_then(|head| head.ref_name.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            Some(branch) => Some(self.branch_exists(token, target, branch).await?),
            None => None,
        };
        let required_check_state = match head_sha.as_deref() {
            Some(sha) => self.combined_status(token, target, sha).await?,
            None => None,
        };
        let commit_count = self.pull_request_commit_count(token, &numbered).await?;
        let changed_file_count = self
            .pull_request_changed_file_count(token, &numbered)
            .await?;
        let no_diff_ahead = commit_count == 0 && changed_file_count == 0;
        let stale = PullRequestStaleClassification {
            is_stale: no_diff_ahead,
            reason: if no_diff_ahead {
                "no commits or changed files ahead of base".to_string()
            } else {
                "pull request has commits or changed files ahead of base".to_string()
            },
            commit_count,
            changed_file_count,
        };
        Ok(PullRequestReadback {
            number: pull.number,
            head_sha,
            state: pull.state.clone(),
            merged: pull.merged,
            merge_commit_sha: pull.merge_commit_sha.clone(),
            required_check_state,
            branch_ref_exists,
            stale,
            pull_request: pull,
        })
    }

    async fn combined_status(
        &self,
        token: &str,
        target: &RepositoryTarget,
        sha: &str,
    ) -> Result<Option<CommitStatusReadiness>, ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/commits/{}/status",
            self.base_url,
            target.owner,
            target.repo,
            percent_encode_path_segment(sha)
        );
        let Some((status, _)) = self
            .get_json_optional::<ForgejoCombinedStatus>(token, url, &[])
            .await?
        else {
            return Ok(None);
        };
        Ok(Some(CommitStatusReadiness::from_combined_status(
            status, sha,
        )))
    }

    async fn branch_exists(
        &self,
        token: &str,
        target: &RepositoryTarget,
        branch: &str,
    ) -> Result<bool, ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/branches/{}",
            self.base_url,
            target.owner,
            target.repo,
            percent_encode_path_segment(branch)
        );
        Ok(self
            .get_json_optional::<serde_json::Value>(token, url, &[])
            .await?
            .is_some())
    }

    async fn pull_request_commit_count(
        &self,
        token: &str,
        target: &NumberedTarget,
    ) -> Result<usize, ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls/{}/commits",
            self.base_url, target.repository.owner, target.repository.repo, target.number
        );
        self.count_paged::<serde_json::Value>(
            token,
            url,
            vec![
                ("verification", "false".to_string()),
                ("files", "false".to_string()),
            ],
        )
        .await
    }

    async fn pull_request_changed_file_count(
        &self,
        token: &str,
        target: &NumberedTarget,
    ) -> Result<usize, ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls/{}/files",
            self.base_url, target.repository.owner, target.repository.repo, target.number
        );
        self.count_paged::<serde_json::Value>(token, url, Vec::new())
            .await
    }

    async fn count_paged<T: for<'de> Deserialize<'de>>(
        &self,
        token: &str,
        url: String,
        extra_query: Vec<(&'static str, String)>,
    ) -> Result<usize, ForgejoError> {
        let mut count = 0usize;
        let mut page = 1u32;
        let limit = 100u32;
        loop {
            let mut query = vec![("page", page.to_string()), ("limit", limit.to_string())];
            query.extend(extra_query.iter().cloned());
            let (items, _) = self.get_json::<Vec<T>>(token, url.clone(), &query).await?;
            let item_count = items.len();
            count = count.saturating_add(item_count);
            if item_count < limit as usize {
                return Ok(count);
            }
            page += 1;
        }
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
                    .map(|review| PullRequestReviewSummary::from_review(review, target))
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
            Page::new(
                items
                    .into_iter()
                    .map(|release| ReleaseSummary::from_release(release, target))
                    .collect(),
                page,
            ),
            status,
        ))
    }

    pub async fn create_release(
        &self,
        token: &str,
        target: &RepositoryTarget,
        options: &CreateReleaseOptions,
    ) -> Result<(CreateReleaseResult, u16), ForgejoError> {
        options.validate()?;
        let url = format!(
            "{}/api/v1/repos/{}/{}/releases",
            self.base_url, target.owner, target.repo
        );
        let response = self
            .http
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .json(&options.to_forgejo_payload())
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let release = response
            .json::<ForgejoRelease>()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let release = ReleaseSummary::from_release(release, target);
        Ok((
            CreateReleaseResult {
                resource_uri: release.resource_uri.clone(),
                release,
            },
            status.as_u16(),
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

    pub async fn list_wiki_pages(
        &self,
        token: &str,
        target: &RepositoryTarget,
        page: PageRequest,
    ) -> Result<(Page<WikiPageMetaSummary>, u16), ForgejoError> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/wiki/pages",
            self.base_url, target.owner, target.repo
        );
        let query = page.query();
        let (items, status) = self
            .get_json::<Vec<ForgejoWikiPageMetaData>>(token, url, &query)
            .await?;
        Ok((
            Page::new(
                items
                    .into_iter()
                    .map(|item| WikiPageMetaSummary::from_page(item, target))
                    .collect(),
                page,
            ),
            status,
        ))
    }

    pub async fn get_wiki_page(
        &self,
        token: &str,
        target: &RepositoryTarget,
        page_name: &str,
    ) -> Result<(WikiPageSummary, u16), ForgejoError> {
        let page_name = page_name.trim();
        if page_name.is_empty() {
            return Err(ForgejoError::MissingWikiPageTitle);
        }
        let url = format!(
            "{}/api/v1/repos/{}/{}/wiki/page/{}",
            self.base_url,
            target.owner,
            target.repo,
            percent_encode_path_segment(page_name)
        );
        let (page, status) = self.get_json::<ForgejoWikiPage>(token, url, &[]).await?;
        Ok((WikiPageSummary::from_page(page, target), status))
    }

    pub async fn create_wiki_page(
        &self,
        token: &str,
        target: &RepositoryTarget,
        options: &WikiPageOptions,
    ) -> Result<(WikiPageSummary, u16), ForgejoError> {
        options.validate()?;
        let url = format!(
            "{}/api/v1/repos/{}/{}/wiki/new",
            self.base_url, target.owner, target.repo
        );
        self.write_wiki_page(token, target, url, reqwest::Method::POST, options)
            .await
    }

    pub async fn update_wiki_page(
        &self,
        token: &str,
        target: &RepositoryTarget,
        options: &WikiPageOptions,
    ) -> Result<(WikiPageSummary, u16), ForgejoError> {
        options.validate()?;
        let url = format!(
            "{}/api/v1/repos/{}/{}/wiki/page/{}",
            self.base_url,
            target.owner,
            target.repo,
            percent_encode_path_segment(&options.title)
        );
        self.write_wiki_page(token, target, url, reqwest::Method::PATCH, options)
            .await
    }

    async fn write_wiki_page(
        &self,
        token: &str,
        target: &RepositoryTarget,
        url: String,
        method: reqwest::Method,
        options: &WikiPageOptions,
    ) -> Result<(WikiPageSummary, u16), ForgejoError> {
        let response = self
            .http
            .request(method, url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .json(&options.to_forgejo_payload())
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let page = response
            .json::<ForgejoWikiPage>()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        Ok((WikiPageSummary::from_page(page, target), status.as_u16()))
    }

    pub async fn merge_pull_request_with_readback(
        &self,
        token: &str,
        target: &NumberedTarget,
        options: &MergePullRequestOptions,
        readback: &PullRequestReadback,
    ) -> Result<(MergePullRequestResult, u16), ForgejoError> {
        options.validate()?;
        self.wait_for_merge_checks(token, &target.repository, readback, options)
            .await?;
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls/{}/merge",
            self.base_url, target.repository.owner, target.repository.repo, target.number
        );
        let response = self
            .http
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .json(&options.to_forgejo_payload())
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(ForgejoError::Api { status, body });
        }
        let forgejo_response = if body.trim().is_empty() {
            serde_json::json!({})
        } else {
            serde_json::from_str(&body).unwrap_or_else(|_| serde_json::json!({ "body": body }))
        };
        let (post_merge_readback, _) = self.pull_request_readback(token, target).await?;
        Ok((
            MergePullRequestResult {
                resource_uri: format!(
                    "forgejo://pull/{}/{}/{}",
                    target.repository.owner, target.repository.repo, target.number
                ),
                merged: true,
                method: options.method.clone(),
                forgejo_response,
                readback: post_merge_readback,
            },
            status.as_u16(),
        ))
    }

    pub async fn close_stale_pull_request(
        &self,
        token: &str,
        target: &NumberedTarget,
    ) -> Result<(PullRequestReadback, u16), ForgejoError> {
        let (readback, _) = self.pull_request_readback(token, target).await?;
        if !readback.stale.is_stale {
            return Err(ForgejoError::PullRequestNotStale {
                number: target.number,
                reason: readback.stale.reason,
            });
        }
        let comment = format!(
            "Closing stale PR #{number}: no commits or diff are ahead of the base branch. head_sha={head_sha} branch_ref_exists={branch_ref_exists:?}",
            number = target.number,
            head_sha = readback.head_sha.as_deref().unwrap_or("unknown"),
            branch_ref_exists = readback.branch_ref_exists,
        );
        self.create_issue_comment(token, target, &comment).await?;
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls/{}",
            self.base_url, target.repository.owner, target.repository.repo, target.number
        );
        let response = self
            .http
            .patch(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .json(&serde_json::json!({ "state": "closed" }))
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let (closed_readback, _) = self.pull_request_readback(token, target).await?;
        Ok((closed_readback, status.as_u16()))
    }

    async fn wait_for_merge_checks(
        &self,
        token: &str,
        target: &RepositoryTarget,
        readback: &PullRequestReadback,
        options: &MergePullRequestOptions,
    ) -> Result<(), ForgejoError> {
        let Some(head_sha) = readback.head_sha.as_deref() else {
            return Ok(());
        };
        let wait_seconds = options.status_check_wait_seconds.unwrap_or(0);
        let poll_seconds = options.status_check_poll_seconds.unwrap_or(5).max(1);
        let started = std::time::Instant::now();
        loop {
            let readiness = self.combined_status(token, target, head_sha).await?;
            let Some(readiness) = readiness else {
                return Ok(());
            };
            if readiness.is_success() {
                return Ok(());
            }
            if started.elapsed().as_secs() >= wait_seconds {
                return Err(ForgejoError::RequiredChecksFailed {
                    sha: head_sha.to_string(),
                    contexts: readiness.context_report(),
                });
            }
            tokio::time::sleep(std::time::Duration::from_secs(poll_seconds)).await;
        }
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

    async fn get_json_optional<T: for<'de> Deserialize<'de>>(
        &self,
        token: &str,
        url: String,
        query: &[(&'static str, String)],
    ) -> Result<Option<(T, u16)>, ForgejoError> {
        let response = self
            .http
            .get(url)
            .header(reqwest::header::AUTHORIZATION, format!("token {token}"))
            .query(query)
            .send()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        let status = response.status();
        if status == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(ForgejoError::Api { status, body });
        }
        let value = response
            .json::<T>()
            .await
            .map_err(|err| ForgejoError::Request(err.to_string()))?;
        Ok(Some((value, status.as_u16())))
    }
}

impl CreateIssueOptions {
    pub fn from_body(body: Option<&str>) -> Result<Self, ForgejoError> {
        let Some(body) = body.map(str::trim).filter(|body| !body.is_empty()) else {
            return Err(ForgejoError::MissingIssueTitle);
        };
        let options: Self =
            serde_json::from_str(body).map_err(|_| ForgejoError::InvalidIssueOptions)?;
        options.validate()?;
        Ok(options)
    }

    fn validate(&self) -> Result<(), ForgejoError> {
        if self.title.trim().is_empty() {
            return Err(ForgejoError::MissingIssueTitle);
        }
        Ok(())
    }

    fn to_forgejo_payload(&self) -> serde_json::Value {
        let mut value = serde_json::json!({ "title": self.title.trim() });
        if let Some(body) = &self.body {
            value["body"] = serde_json::json!(body);
        }
        if let Some(assignee) = self
            .assignee
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            value["assignee"] = serde_json::json!(assignee);
        }
        let assignees = self
            .assignees
            .iter()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        if !assignees.is_empty() {
            value["assignees"] = serde_json::json!(assignees);
        }
        if !self.labels.is_empty() {
            value["labels"] = serde_json::json!(self.labels);
        }
        if let Some(milestone) = self.milestone {
            value["milestone"] = serde_json::json!(milestone);
        }
        if let Some(due_date) = &self.due_date {
            value["due_date"] = serde_json::json!(due_date);
        }
        if let Some(reference) = self
            .reference
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            value["ref"] = serde_json::json!(reference);
        }
        value
    }
}

impl WikiPageOptions {
    pub fn from_body(body: Option<&str>) -> Result<Self, ForgejoError> {
        let Some(body) = body.map(str::trim).filter(|body| !body.is_empty()) else {
            return Err(ForgejoError::MissingWikiPageTitle);
        };
        let options: Self =
            serde_json::from_str(body).map_err(|_| ForgejoError::InvalidWikiPageOptions)?;
        options.validate()?;
        Ok(options)
    }

    fn validate(&self) -> Result<(), ForgejoError> {
        if self.title.trim().is_empty() {
            return Err(ForgejoError::MissingWikiPageTitle);
        }
        if self.content_base64.trim().is_empty() {
            return Err(ForgejoError::MissingWikiPageContent);
        }
        Ok(())
    }

    fn to_forgejo_payload(&self) -> serde_json::Value {
        let mut value = serde_json::json!({
            "title": self.title.trim(),
            "content_base64": self.content_base64.trim(),
        });
        if let Some(message) = self
            .message
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            value["message"] = serde_json::json!(message);
        }
        value
    }
}

impl CreateReleaseOptions {
    pub fn from_body(body: Option<&str>) -> Result<Self, ForgejoError> {
        let Some(body) = body.map(str::trim).filter(|body| !body.is_empty()) else {
            return Err(ForgejoError::MissingReleaseTag);
        };
        let options: Self =
            serde_json::from_str(body).map_err(|_| ForgejoError::InvalidReleaseOptions)?;
        options.validate()?;
        Ok(options)
    }

    fn validate(&self) -> Result<(), ForgejoError> {
        if self.tag_name.trim().is_empty() {
            return Err(ForgejoError::MissingReleaseTag);
        }
        Ok(())
    }

    fn to_forgejo_payload(&self) -> serde_json::Value {
        let mut value = serde_json::json!({
            "tag_name": self.tag_name.trim(),
        });
        if let Some(target_commitish) = self
            .target_commitish
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            value["target_commitish"] = serde_json::json!(target_commitish);
        }
        if let Some(name) = self
            .name
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            value["name"] = serde_json::json!(name);
        }
        if let Some(body) = &self.body {
            value["body"] = serde_json::json!(body);
        }
        if let Some(draft) = self.draft {
            value["draft"] = serde_json::json!(draft);
        }
        if let Some(prerelease) = self.prerelease {
            value["prerelease"] = serde_json::json!(prerelease);
        }
        if let Some(hide_archive_links) = self.hide_archive_links {
            value["hide_archive_links"] = serde_json::json!(hide_archive_links);
        }
        value
    }
}

impl CreatePullRequestOptions {
    pub fn from_body(body: Option<&str>) -> Result<Self, ForgejoError> {
        let Some(body) = body.map(str::trim).filter(|body| !body.is_empty()) else {
            return Err(ForgejoError::MissingPullRequestFields);
        };
        let options: Self =
            serde_json::from_str(body).map_err(|_| ForgejoError::InvalidPullRequestOptions)?;
        options.validate()?;
        Ok(options)
    }

    fn validate(&self) -> Result<(), ForgejoError> {
        if self.head.trim().is_empty()
            || self.base.trim().is_empty()
            || self.title.trim().is_empty()
        {
            return Err(ForgejoError::MissingPullRequestFields);
        }
        Ok(())
    }

    fn to_forgejo_payload(&self) -> serde_json::Value {
        let mut value = serde_json::json!({
            "head": self.head.trim(),
            "base": self.base.trim(),
            "title": self.title.trim(),
        });
        if let Some(body) = &self.body {
            value["body"] = serde_json::json!(body);
        }
        if let Some(draft) = self.draft {
            value["draft"] = serde_json::json!(draft);
        }
        if let Some(assignee) = self
            .assignee
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            value["assignee"] = serde_json::json!(assignee);
        }
        let assignees = self
            .assignees
            .iter()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        if !assignees.is_empty() {
            value["assignees"] = serde_json::json!(assignees);
        }
        value
    }
}

impl Default for MergePullRequestOptions {
    fn default() -> Self {
        Self {
            method: default_merge_method(),
            title: None,
            message: None,
            delete_branch_after_merge: None,
            force_merge: None,
            head_commit_id: None,
            status_check_wait_seconds: None,
            status_check_poll_seconds: None,
        }
    }
}

impl MergePullRequestOptions {
    pub fn from_body(body: Option<&str>) -> Result<Self, ForgejoError> {
        let Some(body) = body.map(str::trim).filter(|body| !body.is_empty()) else {
            return Ok(Self::default());
        };
        serde_json::from_str(body).map_err(|_| ForgejoError::InvalidMergeOptions)
    }

    fn validate(&self) -> Result<(), ForgejoError> {
        match self.method.as_str() {
            "merge" | "squash" | "rebase" | "rebase-merge" => Ok(()),
            _ => Err(ForgejoError::UnsupportedMergeMethod),
        }
    }

    fn to_forgejo_payload(&self) -> serde_json::Value {
        let mut value = serde_json::json!({
            "Do": self.method,
        });
        if let Some(title) = &self.title {
            value["MergeTitleField"] = serde_json::json!(title);
        }
        if let Some(message) = &self.message {
            value["MergeMessageField"] = serde_json::json!(message);
        }
        if let Some(delete_branch_after_merge) = self.delete_branch_after_merge {
            value["delete_branch_after_merge"] = serde_json::json!(delete_branch_after_merge);
        }
        if let Some(force_merge) = self.force_merge {
            value["ForceMerge"] = serde_json::json!(force_merge);
        }
        if let Some(head_commit_id) = &self.head_commit_id {
            value["HeadCommitID"] = serde_json::json!(head_commit_id);
        }
        value
    }
}

fn default_merge_method() -> String {
    "merge".to_string()
}

impl RepositoryTarget {
    pub fn parse(value: &str) -> Result<Self, ForgejoError> {
        if let Some(target) = value.strip_prefix("forgejo://repository/") {
            return Self::parse_owner_repo_parts(target);
        }
        if let Some(target) = value.strip_prefix("forgejo://repo/") {
            return Self::parse_owner_repo_parts(target);
        }
        Self::parse_owner_repo_parts(value)
    }

    fn parse_owner_repo_parts(value: &str) -> Result<Self, ForgejoError> {
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

    pub fn resource_uri(&self) -> String {
        format!("forgejo://repository/{}/{}", self.owner, self.repo)
    }

    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

impl NumberedTarget {
    pub fn parse(value: &str) -> Result<Self, ForgejoError> {
        if let Some(target) = value.strip_prefix("forgejo://issue/") {
            return Self::parse_owner_repo_number_parts(target);
        }
        if let Some(target) = value.strip_prefix("forgejo://pull/") {
            return Self::parse_owner_repo_number_parts(target);
        }
        if let Some((repo_target, number)) = value.rsplit_once('#') {
            return Ok(Self {
                repository: RepositoryTarget::parse(repo_target)?,
                number: parse_positive_number(number)?,
            });
        }
        Self::parse_owner_repo_number_parts(value)
    }

    fn parse_owner_repo_number_parts(value: &str) -> Result<Self, ForgejoError> {
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

impl CommitStatusReadiness {
    fn from_combined_status(value: ForgejoCombinedStatus, fallback_sha: &str) -> Self {
        let statuses = value
            .statuses
            .into_iter()
            .map(CommitStatusSummary::from_status)
            .collect::<Vec<_>>();
        let failing_contexts = statuses
            .iter()
            .filter(|status| status.is_failure())
            .cloned()
            .collect::<Vec<_>>();
        let pending_contexts = statuses
            .iter()
            .filter(|status| status.is_pending())
            .cloned()
            .collect::<Vec<_>>();
        Self {
            sha: value.sha.unwrap_or_else(|| fallback_sha.to_string()),
            state: value.state,
            total_count: value.total_count,
            statuses,
            failing_contexts,
            pending_contexts,
        }
    }

    fn is_success(&self) -> bool {
        self.state.as_deref() == Some("success")
            && self.failing_contexts.is_empty()
            && self.pending_contexts.is_empty()
    }

    fn context_report(&self) -> String {
        let report = self
            .statuses
            .iter()
            .filter(|status| !status.is_success())
            .map(CommitStatusSummary::report)
            .collect::<Vec<_>>();
        if report.is_empty() {
            format!(
                "combined_state={}",
                self.state.as_deref().unwrap_or("unknown")
            )
        } else {
            report.join("; ")
        }
    }
}

impl CommitStatusSummary {
    fn from_status(value: ForgejoCommitStatus) -> Self {
        Self {
            context: non_empty(value.context),
            status: non_empty(value.status),
            target_url: non_empty(value.target_url),
            url: non_empty(value.url),
            description: non_empty(value.description),
        }
    }

    fn is_success(&self) -> bool {
        self.status.as_deref() == Some("success")
    }

    fn is_failure(&self) -> bool {
        matches!(self.status.as_deref(), Some("error" | "failure"))
    }

    fn is_pending(&self) -> bool {
        !self.is_success() && !self.is_failure()
    }

    fn report(&self) -> String {
        format!(
            "context={} status={} target_url={} status_url={} description={}",
            self.context.as_deref().unwrap_or("unknown"),
            self.status.as_deref().unwrap_or("unknown"),
            self.target_url.as_deref().unwrap_or("none"),
            self.url.as_deref().unwrap_or("none"),
            self.description.as_deref().unwrap_or("")
        )
    }
}

fn select_created_pull_request(
    pulls: Vec<PullRequestSummary>,
    target: &RepositoryTarget,
    options: &CreatePullRequestOptions,
) -> Result<PullRequestSummary, ForgejoError> {
    let head = options.head.trim();
    let base = options.base.trim();
    let mut exact_branch_matches = pulls
        .into_iter()
        .filter(|pull| {
            pull_branch_matches(pull.head.as_ref(), head)
                && pull_branch_matches(pull.base.as_ref(), base)
        })
        .collect::<Vec<_>>();

    match exact_branch_matches.len() {
        0 => Err(readback_no_match(target, options)),
        1 => Ok(exact_branch_matches.remove(0)),
        _ => {
            let mut title_matches = exact_branch_matches
                .iter()
                .filter(|pull| pull.title == options.title.trim())
                .cloned()
                .collect::<Vec<_>>();
            match title_matches.len() {
                1 => Ok(title_matches.remove(0)),
                0 => Err(readback_ambiguous(target, options, &exact_branch_matches)),
                _ => Err(readback_ambiguous(target, options, &title_matches)),
            }
        }
    }
}

fn readback_no_match(
    target: &RepositoryTarget,
    options: &CreatePullRequestOptions,
) -> ForgejoError {
    ForgejoError::PullRequestReadbackNoMatch {
        repo: target.full_name(),
        head: options.head.trim().to_string(),
        base: options.base.trim().to_string(),
        title: options.title.trim().to_string(),
    }
}

fn readback_ambiguous(
    target: &RepositoryTarget,
    options: &CreatePullRequestOptions,
    candidates: &[PullRequestSummary],
) -> ForgejoError {
    ForgejoError::PullRequestReadbackAmbiguous {
        repo: target.full_name(),
        head: options.head.trim().to_string(),
        base: options.base.trim().to_string(),
        title: options.title.trim().to_string(),
        candidates: candidates
            .iter()
            .map(|pull| {
                format!(
                    "#{} head={} base={} title={}",
                    pull.number,
                    branch_label(pull.head.as_ref()),
                    branch_label(pull.base.as_ref()),
                    pull.title
                )
            })
            .collect::<Vec<_>>()
            .join("; "),
    }
}

fn pull_branch_matches(branch: Option<&PullRequestBranchSummary>, expected: &str) -> bool {
    let expected = expected.trim();
    if expected.is_empty() {
        return false;
    }
    branch.is_some_and(|branch| {
        branch.ref_name.as_deref() == Some(expected)
            || branch.label.as_deref() == Some(expected)
            || branch
                .label
                .as_deref()
                .and_then(|label| label.rsplit_once(':').map(|(_, name)| name))
                == Some(expected)
    })
}

fn branch_label(branch: Option<&PullRequestBranchSummary>) -> String {
    branch
        .and_then(|branch| {
            branch
                .label
                .clone()
                .or_else(|| branch.ref_name.clone())
                .or_else(|| branch.sha.clone())
        })
        .unwrap_or_else(|| "<unknown>".to_string())
}

fn non_empty(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
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

fn percent_encode_path_segment(value: &str) -> String {
    let mut encoded = String::new();
    for &byte in value.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(byte as char);
        } else {
            const HEX: &[u8; 16] = b"0123456789ABCDEF";
            encoded.push('%');
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }
    encoded
}

impl RepositoryMetadata {
    fn from_repository(value: ForgejoRepository, target: &RepositoryTarget) -> Self {
        Self {
            resource_uri: target.resource_uri(),
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

impl IssueSummary {
    fn from_issue(value: ForgejoIssue, target: &RepositoryTarget) -> Self {
        Self {
            resource_uri: format!(
                "forgejo://issue/{}/{}/{}",
                target.owner, target.repo, value.number
            ),
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

impl WikiPageSummary {
    fn from_page(value: ForgejoWikiPage, target: &RepositoryTarget) -> Self {
        let title = value.title;
        Self {
            resource_uri: format!(
                "forgejo://wiki-page/{}/{}/{}",
                target.owner, target.repo, title
            ),
            title,
            html_url: value.html_url,
            sub_url: value.sub_url,
            commit_count: value.commit_count,
            has_content_base64: value.content_base64.is_some(),
        }
    }
}

impl WikiPageMetaSummary {
    fn from_page(value: ForgejoWikiPageMetaData, target: &RepositoryTarget) -> Self {
        let title = value.title;
        Self {
            resource_uri: format!(
                "forgejo://wiki-page/{}/{}/{}",
                target.owner, target.repo, title
            ),
            title,
            html_url: value.html_url,
            sub_url: value.sub_url,
        }
    }
}

impl PullRequestSummary {
    fn from_pull_request(value: ForgejoPullRequest, target: &RepositoryTarget) -> Option<Self> {
        let number = value.number?;
        let title = non_empty(value.title)?;
        let state = non_empty(value.state)?;
        let url = non_empty(value.url);
        let html_url = non_empty(value.html_url);
        if url.is_none() && html_url.is_none() {
            return None;
        }
        Some(Self {
            resource_uri: format!("forgejo://pull/{}/{}/{}", target.owner, target.repo, number),
            number,
            url,
            title,
            state: Some(state),
            html_url,
            draft: value.draft,
            mergeable: value.mergeable,
            merged: value.merged,
            merge_commit_sha: non_empty(value.merge_commit_sha),
            head: value
                .head
                .and_then(PullRequestBranchSummary::from_forgejo_branch),
            base: value
                .base
                .and_then(PullRequestBranchSummary::from_forgejo_branch),
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

impl PullRequestBranchSummary {
    fn from_forgejo_branch(value: ForgejoPullRequestBranch) -> Option<Self> {
        let ref_name = non_empty(value.ref_name);
        let sha = non_empty(value.sha);
        let label = non_empty(value.label);
        if ref_name.is_none() && sha.is_none() && label.is_none() {
            return None;
        }
        Some(Self {
            ref_name,
            sha,
            label,
        })
    }
}

impl PullRequestReviewSummary {
    fn from_review(value: ForgejoPullRequestReview, target: &NumberedTarget) -> Self {
        Self {
            resource_uri: format!(
                "forgejo://pull-review/{}/{}/{}/{}",
                target.repository.owner, target.repository.repo, target.number, value.id
            ),
            id: value.id,
            state: value.state,
            body: value.body,
            user: value.user.and_then(|user| user.login),
            submitted_at: value.submitted_at,
        }
    }
}

impl ReleaseSummary {
    fn from_release(value: ForgejoRelease, target: &RepositoryTarget) -> Self {
        let tag_name = value.tag_name;
        Self {
            resource_uri: format!(
                "forgejo://release/{}/{}/{}",
                target.owner, target.repo, tag_name
            ),
            id: value.id,
            tag_name,
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
            resource_uri: format!("forgejo://notification/{}", value.id),
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

impl IssueCommentSummary {
    fn from_comment(value: ForgejoIssueComment, target: &NumberedTarget) -> Self {
        Self {
            resource_uri: format!(
                "forgejo://issue-comment/{}/{}/{}/{}",
                target.repository.owner, target.repository.repo, target.number, value.id
            ),
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
        let target =
            RepositoryTarget::parse("forgejo://repository/rawholding/forgejo-keycloak-rust-mcp")
                .unwrap();
        assert_eq!(
            target.resource_uri(),
            "forgejo://repository/rawholding/forgejo-keycloak-rust-mcp"
        );
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
        let target = NumberedTarget::parse("forgejo://pull/rawholding/example/8").unwrap();
        assert_eq!(target.repository.owner, "rawholding");
        assert_eq!(target.repository.repo, "example");
        assert_eq!(target.number, 8);
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
    fn normal_create_pull_request_response_includes_number() {
        let target = RepositoryTarget::parse("rawholding/example").unwrap();
        let pull = serde_json::from_value::<ForgejoPullRequest>(serde_json::json!({
            "number": 42,
            "url": "https://forgejo.example/api/v1/repos/rawholding/example/pulls/42",
            "html_url": "https://forgejo.example/rawholding/example/pulls/42",
            "title": "Add feature",
            "state": "open",
            "mergeable": true,
            "head": { "ref": "feature", "sha": "abc123" },
            "base": { "ref": "main", "sha": "def456" }
        }))
        .unwrap();
        let pull = PullRequestSummary::from_pull_request(pull, &target).unwrap();

        assert_eq!(pull.number, 42);
        assert_eq!(
            pull.html_url.as_deref(),
            Some("https://forgejo.example/rawholding/example/pulls/42")
        );
        assert_eq!(pull.state.as_deref(), Some("open"));
        assert_eq!(
            pull.head.as_ref().and_then(|head| head.ref_name.as_deref()),
            Some("feature")
        );
        assert_eq!(
            pull.base.as_ref().and_then(|base| base.ref_name.as_deref()),
            Some("main")
        );
        assert_eq!(pull.mergeable, Some(true));
    }

    #[test]
    fn empty_create_pull_request_response_readback_finds_pr() {
        let target = RepositoryTarget::parse("rawholding/example").unwrap();
        let options = create_pr_options();
        let pull = serde_json::from_value::<ForgejoPullRequest>(serde_json::json!({})).unwrap();
        assert!(PullRequestSummary::from_pull_request(pull, &target).is_none());

        let pull = select_created_pull_request(
            vec![pull_summary(7, "feature", "main", "Add feature")],
            &target,
            &options,
        )
        .unwrap();

        assert_eq!(pull.number, 7);
    }

    #[test]
    fn empty_create_pull_request_response_without_readback_match_fails_loudly() {
        let target = RepositoryTarget::parse("rawholding/example").unwrap();
        let options = create_pr_options();
        let err = select_created_pull_request(
            vec![pull_summary(7, "other-feature", "main", "Add feature")],
            &target,
            &options,
        )
        .unwrap_err();
        let message = err.to_string();

        assert!(message.contains("readback found no open PR"));
        assert!(message.contains("repo=rawholding/example"));
        assert!(message.contains("head=feature"));
        assert!(message.contains("base=main"));
        assert!(message.contains("title=Add feature"));
    }

    #[test]
    fn multiple_create_pull_request_readback_matches_fail_as_ambiguous() {
        let target = RepositoryTarget::parse("rawholding/example").unwrap();
        let options = create_pr_options();
        let err = select_created_pull_request(
            vec![
                pull_summary(7, "feature", "main", "Add feature"),
                pull_summary(8, "feature", "main", "Add feature"),
            ],
            &target,
            &options,
        )
        .unwrap_err();
        let message = err.to_string();

        assert!(message.contains("readback is ambiguous"));
        assert!(message.contains("#7 head=feature base=main title=Add feature"));
        assert!(message.contains("#8 head=feature base=main title=Add feature"));
    }

    #[test]
    fn parses_merge_options_from_json_body() {
        let options = MergePullRequestOptions::from_body(Some(
            r#"{"method":"squash","delete_branch_after_merge":true}"#,
        ))
        .unwrap();
        assert_eq!(options.method, "squash");
        assert_eq!(options.delete_branch_after_merge, Some(true));
        assert!(MergePullRequestOptions::from_body(Some("not json")).is_err());
        assert!(
            MergePullRequestOptions {
                method: "invalid".to_string(),
                ..MergePullRequestOptions::default()
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn parses_create_release_options_from_json_body() {
        let options = CreateReleaseOptions::from_body(Some(
            r#"{"tag_name":"v1.2.3","name":"Release 1.2.3","draft":true}"#,
        ))
        .unwrap();
        assert_eq!(options.tag_name, "v1.2.3");
        assert_eq!(options.name.as_deref(), Some("Release 1.2.3"));
        assert_eq!(options.draft, Some(true));

        let payload = options.to_forgejo_payload();
        assert_eq!(payload["tag_name"], "v1.2.3");
        assert_eq!(payload["name"], "Release 1.2.3");
        assert_eq!(payload["draft"], true);

        assert!(CreateReleaseOptions::from_body(Some("not json")).is_err());
        assert!(CreateReleaseOptions::from_body(Some(r#"{"name":"missing tag"}"#)).is_err());
        assert!(CreateReleaseOptions::from_body(None).is_err());
    }

    #[test]
    fn parses_create_pull_request_options_from_json_body() {
        let options = CreatePullRequestOptions::from_body(Some(
            r#"{"head":"feature","base":"main","title":"Add feature","body":"details","assignees":["alice"],"reviewers":["bob"],"draft":true}"#,
        ))
        .unwrap();
        assert_eq!(options.head, "feature");
        assert_eq!(options.base, "main");
        assert_eq!(options.title, "Add feature");
        assert_eq!(options.reviewers, vec!["bob"]);

        let payload = options.to_forgejo_payload();
        assert_eq!(payload["head"], "feature");
        assert_eq!(payload["base"], "main");
        assert_eq!(payload["title"], "Add feature");
        assert_eq!(payload["draft"], true);
        assert_eq!(payload["assignees"][0], "alice");
        assert!(payload.get("reviewers").is_none());

        assert!(CreatePullRequestOptions::from_body(Some("not json")).is_err());
        assert!(
            CreatePullRequestOptions::from_body(Some(r#"{"head":"feature","base":"main"}"#))
                .is_err()
        );
        assert!(CreatePullRequestOptions::from_body(None).is_err());
    }

    fn create_pr_options() -> CreatePullRequestOptions {
        CreatePullRequestOptions {
            head: "feature".to_string(),
            base: "main".to_string(),
            title: "Add feature".to_string(),
            body: None,
            draft: None,
            assignee: None,
            assignees: Vec::new(),
            reviewers: Vec::new(),
        }
    }

    fn pull_summary(
        number: u64,
        head: impl Into<String>,
        base: impl Into<String>,
        title: impl Into<String>,
    ) -> PullRequestSummary {
        PullRequestSummary {
            resource_uri: format!("forgejo://pull/rawholding/example/{number}"),
            number,
            url: Some(format!(
                "https://forgejo.example/api/v1/repos/rawholding/example/pulls/{number}"
            )),
            title: title.into(),
            state: Some("open".to_string()),
            html_url: Some(format!(
                "https://forgejo.example/rawholding/example/pulls/{number}"
            )),
            draft: None,
            mergeable: None,
            merged: Some(false),
            merge_commit_sha: None,
            head: Some(PullRequestBranchSummary {
                ref_name: Some(head.into()),
                sha: Some(format!("head-sha-{number}")),
                label: None,
            }),
            base: Some(PullRequestBranchSummary {
                ref_name: Some(base.into()),
                sha: Some(format!("base-sha-{number}")),
                label: None,
            }),
            created_at: None,
            updated_at: None,
        }
    }

    #[test]
    fn parses_create_issue_options_from_json_body() {
        let options = CreateIssueOptions::from_body(Some(
            r#"{"title":"Repair adapter","body":"details","assignees":["agent"],"labels":[1,2],"reference":"main"}"#,
        ))
        .unwrap();
        assert_eq!(options.title, "Repair adapter");
        let payload = options.to_forgejo_payload();
        assert_eq!(payload["title"], "Repair adapter");
        assert_eq!(payload["assignees"][0], "agent");
        assert_eq!(payload["labels"][1], 2);
        assert_eq!(payload["ref"], "main");

        assert!(CreateIssueOptions::from_body(Some("not json")).is_err());
        assert!(CreateIssueOptions::from_body(Some(r#"{"body":"missing title"}"#)).is_err());
    }

    #[test]
    fn parses_wiki_page_options_without_decoding_content() {
        let options = WikiPageOptions::from_body(Some(
            r#"{"title":"Agent-Runbook","content_base64":"IyBSdW5ib29rCg==","message":"Publish runbook"}"#,
        ))
        .unwrap();
        let payload = options.to_forgejo_payload();
        assert_eq!(payload["title"], "Agent-Runbook");
        assert_eq!(payload["content_base64"], "IyBSdW5ib29rCg==");
        assert_eq!(payload["message"], "Publish runbook");
        assert_eq!(
            percent_encode_path_segment("Agent Runbook"),
            "Agent%20Runbook"
        );

        assert!(WikiPageOptions::from_body(Some("not json")).is_err());
        assert!(WikiPageOptions::from_body(Some(r#"{"title":"missing content"}"#)).is_err());
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
        let target = RepositoryTarget::parse("rawholding/example").unwrap();
        let metadata = RepositoryMetadata::from_repository(repository, &target);
        assert_eq!(
            metadata.resource_uri,
            "forgejo://repository/rawholding/example"
        );
        assert_eq!(metadata.full_name, "rawholding/example");
        assert!(metadata.permissions.unwrap().pull);
    }
}
