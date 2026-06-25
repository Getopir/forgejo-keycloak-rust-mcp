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
    IssueComment(CommentArgs),
    PullRequests(ListTargetArgs),
    PullReviews(NumberedListArgs),
    Releases(ListTargetArgs),
    Notifications(NotificationArgs),
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
struct CommentArgs {
    target: String,
    #[arg(long)]
    body: String,
}

#[derive(Debug, Serialize)]
struct McpRequest {
    operation: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
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
            Command::IssueComment(args) => McpRequest {
                operation: "create_issue_comment",
                target: Some(args.target),
                limit: None,
                cursor: None,
                state: None,
                body: Some(args.body),
            },
            Command::PullRequests(args) => list_request(
                "list_pull_requests",
                Some(args.target),
                args.state,
                args.limit,
                args.cursor,
            ),
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
            Command::Notifications(args) => list_request(
                "list_notifications",
                None,
                args.state,
                args.limit,
                args.cursor,
            ),
        }
    }
}

fn target_request(operation: &'static str, target: String) -> McpRequest {
    McpRequest {
        operation,
        target: Some(target),
        limit: None,
        cursor: None,
        state: None,
        body: None,
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
        target,
        limit,
        cursor,
        state,
        body: None,
    }
}
