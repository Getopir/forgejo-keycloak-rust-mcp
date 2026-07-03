// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::{Context, bail};
use clap::{Parser, Subcommand};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Parser)]
#[command(about = "CLI wrapper for forgejo-mcpd curated MCP operations")]
struct Cli {
    #[arg(
        long,
        env = "FORGEJO_MCPCTL_GATEWAY",
        default_value = "http://127.0.0.1:7080/mcp"
    )]
    gateway: String,
    #[arg(long, env = "FORGEJO_MCPCTL_TOKEN_ENV", default_value = "ACCESS_JWT")]
    token_env: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    GatewayProbe(TargetArgs),
    RepositoryMetadata(TargetArgs),
    RepositoryIssues(ListTargetArgs),
    CreateIssue(CreateIssueArgs),
    IssueComment(CommentArgs),
    PullRequests(ListTargetArgs),
    CreatePullRequest(CreatePullRequestArgs),
    PullReviews(NumberedListArgs),
    Releases(ListTargetArgs),
    WikiPages(NumberedListArgs),
    WikiPage(WikiPageReadArgs),
    CreateWikiPage(WikiPageWriteArgs),
    UpdateWikiPage(WikiPageWriteArgs),
    CredentialReferenceStatus,
    ApiCoverage(ApiCoverageArgs),
    CreateRelease(CreateReleaseArgs),
    Notifications(NotificationArgs),
    CreateApproval(ApprovalArgs),
    MergePullRequest(MergePullRequestArgs),
}

#[derive(Debug, Parser)]
struct TargetArgs {
    target: String,
}

#[derive(Debug, Parser)]
struct ListTargetArgs {
    target: String,
    #[arg(long)]
    state: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    cursor: Option<String>,
}

#[derive(Debug, Parser)]
struct NumberedListArgs {
    target: String,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    cursor: Option<String>,
}

#[derive(Debug, Parser)]
struct NotificationArgs {
    #[arg(long)]
    state: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    cursor: Option<String>,
}

#[derive(Debug, Parser)]
struct ApiCoverageArgs {
    #[arg(long)]
    filter: Option<String>,
    #[arg(long)]
    query: Option<String>,
    #[arg(long)]
    limit: Option<u32>,
    #[arg(long)]
    cursor: Option<String>,
}

#[derive(Debug, Parser)]
struct CommentArgs {
    target: String,
    #[arg(long)]
    body: String,
}

#[derive(Debug, Parser)]
struct CreateIssueArgs {
    target: String,
    #[arg(long)]
    title: String,
    #[arg(long)]
    body: Option<String>,
    #[arg(long)]
    assignee: Option<String>,
    #[arg(long)]
    assignee_user: Vec<String>,
    #[arg(long)]
    label: Vec<i64>,
}

#[derive(Debug, Parser)]
struct WikiPageReadArgs {
    target: String,
    #[arg(long)]
    page: String,
}

#[derive(Debug, Parser)]
struct WikiPageWriteArgs {
    target: String,
    #[arg(long)]
    title: String,
    #[arg(long)]
    content_base64: String,
    #[arg(long)]
    message: Option<String>,
    #[arg(long)]
    approval_id: Option<String>,
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Parser)]
struct ApprovalArgs {
    requested_operation: String,
    target: String,
    #[arg(long)]
    body: Option<String>,
    #[arg(long)]
    state: Option<String>,
}

#[derive(Debug, Parser)]
struct MergePullRequestArgs {
    target: String,
    #[arg(long)]
    approval_id: Option<String>,
    #[arg(long)]
    dry_run: bool,
    #[arg(long, default_value = "merge")]
    method: String,
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    message: Option<String>,
    #[arg(long)]
    delete_branch_after_merge: bool,
}

#[derive(Debug, Parser)]
struct CreatePullRequestArgs {
    target: String,
    #[arg(long)]
    head: String,
    #[arg(long)]
    base: String,
    #[arg(long)]
    title: String,
    #[arg(long)]
    body: Option<String>,
    #[arg(long)]
    approval_id: Option<String>,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    draft: bool,
    #[arg(long)]
    assignee: Option<String>,
    #[arg(long)]
    assignee_user: Vec<String>,
    #[arg(long)]
    reviewer: Vec<String>,
}

impl CreatePullRequestArgs {
    fn into_request(self) -> McpRequest {
        let mut body = serde_json::json!({
            "head": self.head,
            "base": self.base,
            "title": self.title,
        });
        if let Some(value) = self.body {
            body["body"] = serde_json::json!(value);
        }
        if self.draft {
            body["draft"] = serde_json::json!(true);
        }
        if let Some(value) = self.assignee {
            body["assignee"] = serde_json::json!(value);
        }
        if !self.assignee_user.is_empty() {
            body["assignees"] = serde_json::json!(self.assignee_user);
        }
        if !self.reviewer.is_empty() {
            body["reviewers"] = serde_json::json!(self.reviewer);
        }
        McpRequest {
            operation: "create_pull_request",
            requested_operation: None,
            target: Some(self.target),
            query: None,
            limit: None,
            cursor: None,
            state: None,
            body: Some(body.to_string()),
            approval_id: self.approval_id,
            dry_run: self.dry_run,
        }
    }
}

#[derive(Debug, Parser)]
struct CreateReleaseArgs {
    target: String,
    #[arg(long)]
    tag_name: String,
    #[arg(long)]
    approval_id: Option<String>,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    target_commitish: Option<String>,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    body: Option<String>,
    #[arg(long)]
    draft: bool,
    #[arg(long)]
    prerelease: bool,
    #[arg(long)]
    hide_archive_links: bool,
}

#[derive(Debug, Serialize)]
struct McpRequest {
    operation: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_id: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let token =
        std::env::var(&cli.token_env).with_context(|| format!("{} is not set", cli.token_env))?;
    if token.trim().is_empty() {
        bail!("{} is empty", cli.token_env);
    }

    let request = cli.command.into_request();
    let response = reqwest::Client::new()
        .post(cli.gateway)
        .bearer_auth(token.trim())
        .json(&request)
        .send()
        .await
        .context("MCP request failed")?;
    let status = response.status();
    let body = response
        .json::<Value>()
        .await
        .context("MCP response was not JSON")?;
    println!("{}", serde_json::to_string_pretty(&body)?);
    if !status.is_success() {
        bail!("MCP request returned {status}");
    }
    Ok(())
}

impl Command {
    fn into_request(self) -> McpRequest {
        match self {
            Command::GatewayProbe(args) => target_request("gateway_probe", args.target),
            Command::RepositoryMetadata(args) => {
                target_request("list_repository_metadata", args.target)
            }
            Command::RepositoryIssues(args) => list_request(
                "list_repository_issues",
                Some(args.target),
                args.state,
                args.limit,
                args.cursor,
            ),
            Command::CreateIssue(args) => McpRequest {
                operation: "create_issue",
                requested_operation: None,
                target: Some(args.target),
                query: None,
                limit: None,
                cursor: None,
                state: None,
                body: Some(issue_body(
                    args.title,
                    args.body,
                    args.assignee,
                    args.assignee_user,
                    args.label,
                )),
                approval_id: None,
                dry_run: false,
            },
            Command::IssueComment(args) => McpRequest {
                operation: "create_issue_comment",
                requested_operation: None,
                target: Some(args.target),
                query: None,
                limit: None,
                cursor: None,
                state: None,
                body: Some(args.body),
                approval_id: None,
                dry_run: false,
            },
            Command::PullRequests(args) => list_request(
                "list_pull_requests",
                Some(args.target),
                args.state,
                args.limit,
                args.cursor,
            ),
            Command::CreatePullRequest(args) => args.into_request(),
            Command::PullReviews(args) => list_request(
                "list_pull_request_reviews",
                Some(args.target),
                None,
                args.limit,
                args.cursor,
            ),
            Command::Releases(args) => list_request(
                "list_releases",
                Some(args.target),
                args.state,
                args.limit,
                args.cursor,
            ),
            Command::WikiPages(args) => list_request(
                "list_wiki_pages",
                Some(args.target),
                None,
                args.limit,
                args.cursor,
            ),
            Command::WikiPage(args) => McpRequest {
                operation: "get_wiki_page",
                requested_operation: None,
                target: Some(args.target),
                query: Some(args.page),
                limit: None,
                cursor: None,
                state: None,
                body: None,
                approval_id: None,
                dry_run: false,
            },
            Command::CreateWikiPage(args) => wiki_write_request("create_wiki_page", args),
            Command::UpdateWikiPage(args) => wiki_write_request("update_wiki_page", args),
            Command::CredentialReferenceStatus => McpRequest {
                operation: "credential_reference_status",
                requested_operation: None,
                target: None,
                query: None,
                limit: None,
                cursor: None,
                state: None,
                body: None,
                approval_id: None,
                dry_run: false,
            },
            Command::ApiCoverage(args) => McpRequest {
                operation: "forgejo_api_coverage",
                requested_operation: None,
                target: None,
                query: args.query,
                limit: args.limit,
                cursor: args.cursor,
                state: args.filter,
                body: None,
                approval_id: None,
                dry_run: false,
            },
            Command::CreateRelease(args) => McpRequest {
                operation: "create_release",
                requested_operation: None,
                target: Some(args.target),
                query: None,
                limit: None,
                cursor: None,
                state: None,
                body: Some(release_body(
                    args.tag_name,
                    args.target_commitish,
                    args.name,
                    args.body,
                    args.draft,
                    args.prerelease,
                    args.hide_archive_links,
                )),
                approval_id: args.approval_id,
                dry_run: args.dry_run,
            },
            Command::Notifications(args) => list_request(
                "list_notifications",
                None,
                args.state,
                args.limit,
                args.cursor,
            ),
            Command::CreateApproval(args) => McpRequest {
                operation: "create_approval",
                requested_operation: Some(args.requested_operation),
                target: Some(args.target),
                query: None,
                limit: None,
                cursor: None,
                state: args.state,
                body: args.body,
                approval_id: None,
                dry_run: false,
            },
            Command::MergePullRequest(args) => McpRequest {
                operation: "merge_pull_request",
                requested_operation: None,
                target: Some(args.target),
                query: None,
                limit: None,
                cursor: None,
                state: None,
                body: Some(merge_body(
                    args.method,
                    args.title,
                    args.message,
                    args.delete_branch_after_merge,
                )),
                approval_id: args.approval_id,
                dry_run: args.dry_run,
            },
        }
    }
}

fn target_request(operation: &'static str, target: String) -> McpRequest {
    McpRequest {
        operation,
        requested_operation: None,
        target: Some(target),
        query: None,
        limit: None,
        cursor: None,
        state: None,
        body: None,
        approval_id: None,
        dry_run: false,
    }
}

fn list_request(
    operation: &'static str,
    target: Option<String>,
    state: Option<String>,
    limit: Option<u32>,
    cursor: Option<String>,
) -> McpRequest {
    McpRequest {
        operation,
        requested_operation: None,
        target,
        query: None,
        limit,
        cursor,
        state,
        body: None,
        approval_id: None,
        dry_run: false,
    }
}

fn issue_body(
    title: String,
    body: Option<String>,
    assignee: Option<String>,
    assignees: Vec<String>,
    labels: Vec<i64>,
) -> String {
    let mut value = serde_json::json!({ "title": title });
    if let Some(body) = body {
        value["body"] = serde_json::json!(body);
    }
    if let Some(assignee) = assignee {
        value["assignee"] = serde_json::json!(assignee);
    }
    if !assignees.is_empty() {
        value["assignees"] = serde_json::json!(assignees);
    }
    if !labels.is_empty() {
        value["labels"] = serde_json::json!(labels);
    }
    value.to_string()
}

fn wiki_write_request(operation: &'static str, args: WikiPageWriteArgs) -> McpRequest {
    let mut value = serde_json::json!({
        "title": args.title,
        "content_base64": args.content_base64,
    });
    if let Some(message) = args.message {
        value["message"] = serde_json::json!(message);
    }
    McpRequest {
        operation,
        requested_operation: None,
        target: Some(args.target),
        query: None,
        limit: None,
        cursor: None,
        state: None,
        body: Some(value.to_string()),
        approval_id: args.approval_id,
        dry_run: args.dry_run,
    }
}

fn merge_body(
    method: String,
    title: Option<String>,
    message: Option<String>,
    delete_branch_after_merge: bool,
) -> String {
    let mut value = serde_json::json!({ "method": method });
    if let Some(title) = title {
        value["title"] = serde_json::json!(title);
    }
    if let Some(message) = message {
        value["message"] = serde_json::json!(message);
    }
    if delete_branch_after_merge {
        value["delete_branch_after_merge"] = serde_json::json!(true);
    }
    value.to_string()
}

fn release_body(
    tag_name: String,
    target_commitish: Option<String>,
    name: Option<String>,
    body: Option<String>,
    draft: bool,
    prerelease: bool,
    hide_archive_links: bool,
) -> String {
    let mut value = serde_json::json!({ "tag_name": tag_name });
    if let Some(target_commitish) = target_commitish {
        value["target_commitish"] = serde_json::json!(target_commitish);
    }
    if let Some(name) = name {
        value["name"] = serde_json::json!(name);
    }
    if let Some(body) = body {
        value["body"] = serde_json::json!(body);
    }
    if draft {
        value["draft"] = serde_json::json!(true);
    }
    if prerelease {
        value["prerelease"] = serde_json::json!(true);
    }
    if hide_archive_links {
        value["hide_archive_links"] = serde_json::json!(true);
    }
    value.to_string()
}

fn is_false(value: &bool) -> bool {
    !*value
}
