// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::{Context, ensure};
use approval::{ApprovalError, ApprovalStore};
use audit::{AuditDecision, AuditEvent, PrincipalType};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json};
use axum::routing::{get, post};
use axum::{Router, serve};
use clap::Parser;
use forgejo::{
    CreateIssueOptions, CreatePullRequestOptions, CreateReleaseOptions, ForgejoClient,
    ForgejoError, MergePullRequestOptions, NumberedTarget, PageRequest, RepositoryMetadata,
    RepositoryTarget, WikiPageOptions,
};
use identity::JwtValidator;
use policy::OperationRegistry;
use principal::{DelegatedHeader, PrincipalMapper, PrincipalMapping, TrustedHeaderConfig};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};
use uuid::Uuid;

mod approval;
mod forgejo;
mod principal;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, env = "FORGEJO_MCPD_ISSUER")]
    issuer: String,
    #[arg(long, env = "FORGEJO_MCPD_DISCOVERY_URL")]
    discovery_url: Option<String>,
    #[arg(long, env = "FORGEJO_MCPD_AUDIENCE")]
    audience: String,
    #[arg(long, env = "FORGEJO_MCPD_RESOURCE")]
    resource: String,
    #[arg(long, env = "FORGEJO_MCPD_BIND", default_value = "127.0.0.1:7080")]
    bind: SocketAddr,
    #[arg(long, env = "FORGEJO_MCPD_PRINCIPAL_MAP")]
    principal_map: Option<PathBuf>,
    #[arg(long, env = "FORGEJO_MCPD_FORGEJO_URL")]
    forgejo_url: Option<String>,
    #[arg(long, alias = "ssl", env = "FORGEJO_MCPD_TLS", default_value_t = false)]
    tls: bool,
    #[arg(
        long,
        env = "FORGEJO_MCPD_TRUSTED_USER_HEADER",
        default_value = "X-WEBAUTH-USER"
    )]
    trusted_user_header: String,
    #[arg(long, env = "FORGEJO_MCPD_TRUSTED_EMAIL_HEADER")]
    trusted_email_header: Option<String>,
    #[arg(long, env = "FORGEJO_MCPD_TRUSTED_FULL_NAME_HEADER")]
    trusted_full_name_header: Option<String>,
    #[arg(long, env = "FORGEJO_MCPD_MAX_PAGE_LIMIT", default_value_t = 50)]
    max_page_limit: u32,
    #[arg(long, env = "FORGEJO_MCPD_APPROVAL_STORE")]
    approval_store: Option<PathBuf>,
    #[arg(
        long,
        env = "FORGEJO_MCPD_APPROVAL_TTL_SECONDS",
        default_value_t = ApprovalStore::default_ttl_seconds()
    )]
    approval_ttl_seconds: u64,
}

#[derive(Clone)]
struct AppState {
    validator: Arc<JwtValidator>,
    registry: OperationRegistry,
    issuer: String,
    resource: String,
    principal_mapper: Option<Arc<PrincipalMapper>>,
    forgejo: Option<ForgejoClient>,
    trusted_headers: TrustedHeaderConfig,
    max_page_limit: u32,
    approval_store: Option<ApprovalStore>,
}

#[derive(Debug, Serialize)]
struct Health {
    service: &'static str,
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct ProtectedResourceMetadata {
    resource: String,
    authorization_servers: Vec<String>,
    bearer_methods_supported: Vec<&'static str>,
    scopes_supported: Vec<&'static str>,
    resource_signing_alg_values_supported: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct Capabilities {
    operations: Vec<policy::Operation>,
    disabled_but_planned: Vec<PlannedOperation>,
}

#[derive(Debug, Serialize)]
struct PlannedOperation {
    name: &'static str,
    scope: &'static str,
    risk: &'static str,
    approval_required: bool,
    reason: &'static str,
}

#[derive(Debug, Deserialize)]
struct McpProbeRequest {
    #[serde(default = "default_operation")]
    operation: String,
    #[serde(default)]
    requested_operation: Option<String>,
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    query: Option<String>,
    #[serde(default)]
    limit: Option<u32>,
    #[serde(default)]
    cursor: Option<String>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    approval_id: Option<String>,
    #[serde(default)]
    dry_run: bool,
}

#[derive(Debug, Serialize)]
struct McpResponse {
    request_id: Uuid,
    subject: String,
    oauth_client: Option<String>,
    preferred_username: Option<String>,
    forgejo_login: Option<String>,
    forgejo_user_id: Option<i64>,
    trusted_delegation_headers: Vec<DelegatedHeader>,
    operation: String,
    allowed: bool,
    reason: String,
    required_scope: String,
    approval_required: bool,
    target: Option<String>,
    repository: Option<RepositoryMetadata>,
    result: Option<serde_json::Value>,
    limit: Option<u32>,
    next_cursor: Option<String>,
}

#[derive(Clone)]
struct ForgejoAccess {
    mapping: PrincipalMapping,
    forgejo: ForgejoClient,
    token: String,
}

fn default_operation() -> String {
    "gateway_probe".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = Args::parse();
    validate_tls_urls(args.tls, &args.resource, args.forgejo_url.as_deref())?;
    let discovery_url = args.discovery_url.clone().unwrap_or_else(|| {
        format!(
            "{}/.well-known/openid-configuration",
            args.issuer.trim_end_matches('/')
        )
    });
    let validator = JwtValidator::discover_with_url(&args.issuer, discovery_url, &args.audience)
        .await
        .with_context(|| format!("failed to discover OIDC metadata from {}", args.issuer))?;
    let principal_mapper = args
        .principal_map
        .as_ref()
        .map(PrincipalMapper::load)
        .transpose()
        .context("failed to load principal map")?
        .map(Arc::new);
    let forgejo = args.forgejo_url.as_ref().map(ForgejoClient::new);
    let state = AppState {
        validator: Arc::new(validator),
        registry: OperationRegistry::current(),
        issuer: args.issuer,
        resource: args.resource,
        principal_mapper,
        forgejo,
        trusted_headers: TrustedHeaderConfig::new(
            args.trusted_user_header,
            args.trusted_email_header,
            args.trusted_full_name_header,
        ),
        max_page_limit: args.max_page_limit.max(1),
        approval_store: args
            .approval_store
            .map(|path| ApprovalStore::new(path, args.approval_ttl_seconds)),
    };
    let app = Router::new()
        .route("/health", get(health))
        .route(
            "/.well-known/oauth-protected-resource",
            get(protected_resource),
        )
        .route(
            "/.well-known/oauth-protected-resource/mcp",
            get(protected_resource),
        )
        .route("/capabilities", get(capabilities))
        .route("/mcp", post(mcp_handler))
        .with_state(state);

    let listener = TcpListener::bind(args.bind).await?;
    info!(addr = %args.bind, "forgejo-mcpd listening");
    serve(listener, app).await?;
    Ok(())
}

fn validate_tls_urls(
    tls_enabled: bool,
    resource: &str,
    forgejo_url: Option<&str>,
) -> anyhow::Result<()> {
    if !tls_enabled {
        return Ok(());
    }

    ensure!(
        has_https_scheme(resource),
        "--tls requires --resource to use an https:// public MCP URL"
    );
    if let Some(forgejo_url) = forgejo_url {
        ensure!(
            has_https_scheme(forgejo_url),
            "--tls requires --forgejo-url to use an https:// Forgejo URL"
        );
    }
    Ok(())
}

fn has_https_scheme(value: &str) -> bool {
    value
        .trim_start()
        .get(..8)
        .is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://"))
}

async fn health() -> Json<Health> {
    Json(Health {
        service: "forgejo-mcpd",
        status: "ok",
    })
}

async fn protected_resource(State(state): State<AppState>) -> Json<ProtectedResourceMetadata> {
    let scopes = state
        .registry
        .operations()
        .map(|operation| operation.scope)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    Json(ProtectedResourceMetadata {
        resource: state.resource.clone(),
        authorization_servers: vec![state.issuer.clone()],
        bearer_methods_supported: vec!["header"],
        scopes_supported: scopes,
        resource_signing_alg_values_supported: vec!["RS256"],
    })
}

async fn capabilities(State(state): State<AppState>) -> Json<Capabilities> {
    Json(Capabilities {
        operations: state.registry.operations().cloned().collect(),
        disabled_but_planned: vec![
            PlannedOperation {
                name: "update_pull_request",
                scope: "forgejo:pr:write",
                risk: "write_mutating",
                approval_required: true,
                reason: "planned PR lifecycle operation; not reviewed as an executable semantic overlay yet",
            },
            PlannedOperation {
                name: "request_reviewers",
                scope: "forgejo:pr:write",
                risk: "write_mutating",
                approval_required: true,
                reason: "reviewer requests are currently available only inside create_pull_request",
            },
            PlannedOperation {
                name: "get_branch_status",
                scope: "forgejo:repo:read",
                risk: "read_private",
                approval_required: false,
                reason: "planned read operation for branch-to-PR automation",
            },
            PlannedOperation {
                name: "get_required_checks",
                scope: "forgejo:repo:read",
                risk: "read_private",
                approval_required: false,
                reason: "planned read operation for merge readiness",
            },
            PlannedOperation {
                name: "get_pr_checks",
                scope: "forgejo:pr:read",
                risk: "read_private",
                approval_required: false,
                reason: "planned read operation for PR readiness",
            },
        ],
    })
}

async fn mcp_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<McpProbeRequest>,
) -> impl IntoResponse {
    let request_id = Uuid::now_v7();
    if let Some(header) = state.trusted_headers.spoofed_header(&headers) {
        warn!(
            request_id = %request_id,
            header = header,
            "request denied because it supplied a trusted identity header"
        );
        return error_response(
            StatusCode::BAD_REQUEST,
            request_id,
            "trusted identity headers must be generated by the gateway, not supplied by callers",
        );
    }
    let authorization = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    let principal = match state.validator.validate_authorization_header(authorization) {
        Ok(principal) => principal,
        Err(err) => {
            warn!(request_id = %request_id, error = %err, "authentication denied");
            return (
                StatusCode::UNAUTHORIZED,
                [(
                    axum::http::header::WWW_AUTHENTICATE,
                    format!(
                        "Bearer resource_metadata=\"{}/.well-known/oauth-protected-resource\"",
                        state.resource.trim_end_matches("/mcp")
                    ),
                )],
                Json(serde_json::json!({ "request_id": request_id, "error": err.to_string() })),
            )
                .into_response();
        }
    };
    let decision = match state.registry.decide(&body.operation, &principal.scopes) {
        Ok(decision) => decision,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "request_id": request_id, "error": err.to_string() })),
            )
                .into_response();
        }
    };
    let status = if decision.allowed {
        StatusCode::OK
    } else {
        StatusCode::FORBIDDEN
    };
    let mapping = state
        .principal_mapper
        .as_ref()
        .and_then(|mapper| mapper.resolve(&principal).ok().cloned());
    if decision.allowed {
        match body.operation.as_str() {
            "create_release" => {
                return create_release_response(request_id, state, principal, body, decision).await;
            }
            "create_wiki_page" | "update_wiki_page" => {
                return write_wiki_page_response(request_id, state, principal, body, decision)
                    .await;
            }
            "create_pull_request" => {
                return create_pull_request_response(request_id, state, principal, body, decision)
                    .await;
            }
            "merge_pull_request" => {
                return merge_pull_request_response(request_id, state, principal, body, decision)
                    .await;
            }
            _ => {}
        }
    }
    if decision.allowed && decision.approval_required {
        return approval_required_response(
            request_id,
            state,
            principal,
            body,
            decision,
            mapping.as_ref(),
        );
    }
    if decision.allowed {
        match body.operation.as_str() {
            "create_approval" => {
                return create_approval_response(request_id, state, principal, body, decision);
            }
            "forgejo_api_coverage" => {
                return forgejo_api_coverage_response(request_id, state, principal, body, decision);
            }
            "list_repository_metadata" => {
                return repository_metadata_response(request_id, state, principal, body, decision)
                    .await;
            }
            "credential_reference_status" => {
                return credential_reference_status_response(
                    request_id, state, principal, body, decision,
                );
            }
            "list_repository_issues" => {
                return phase2_list_response(request_id, state, principal, body, decision).await;
            }
            "create_issue" => {
                return create_issue_response(request_id, state, principal, body, decision).await;
            }
            "create_issue_comment" => {
                return create_issue_comment_response(request_id, state, principal, body, decision)
                    .await;
            }
            "list_pull_requests"
            | "list_pull_request_reviews"
            | "list_releases"
            | "list_notifications"
            | "list_wiki_pages" => {
                return phase2_list_response(request_id, state, principal, body, decision).await;
            }
            "get_wiki_page" => {
                return get_wiki_page_response(request_id, state, principal, body, decision).await;
            }
            _ => {}
        }
    }
    // Audit records intentionally include identity and policy metadata only.
    // Raw bearer tokens and downstream service credentials must never be logged.
    let event = AuditEvent {
        request_id,
        issuer: principal.issuer.clone(),
        subject: principal.subject.clone(),
        oauth_client: principal.oauth_client.clone(),
        principal_type: mapping
            .as_ref()
            .map(|mapping| PrincipalType::from(mapping.principal_type))
            .unwrap_or(PrincipalType::Unknown),
        forgejo_user_id: mapping.as_ref().and_then(|mapping| mapping.forgejo_user_id),
        forgejo_login: mapping
            .as_ref()
            .map(|mapping| mapping.forgejo_login.clone()),
        tool: body.operation.clone(),
        target: body.target.clone(),
        risk: decision.risk,
        decision: if decision.allowed {
            AuditDecision::Allow
        } else {
            AuditDecision::Deny
        },
        approval_id: None,
        forgejo_status: None,
        duration_ms: 0,
        response_bytes: 0,
    };
    info!(audit = %serde_json::to_string(&event).unwrap_or_default(), "audit event");
    (
        status,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: mapping
                .as_ref()
                .map(|mapping| mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.as_ref().and_then(|mapping| mapping.forgejo_user_id),
            trusted_delegation_headers: mapping
                .as_ref()
                .map(|mapping| state.trusted_headers.delegated_headers(mapping))
                .unwrap_or_default(),
            operation: body.operation.clone(),
            allowed: decision.allowed,
            reason: decision.reason,
            required_scope: decision.required_scope.to_string(),
            approval_required: decision.approval_required,
            target: body.target,
            repository: None,
            result: None,
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

async fn create_pull_request_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let target = match parse_repository_target(&body, request_id) {
        Ok(target) => target,
        Err(response) => return response,
    };
    let options = match CreatePullRequestOptions::from_body(body.body.as_deref()) {
        Ok(options) => options,
        Err(err) => return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()),
    };
    if body.dry_run {
        return create_pull_request_preview_response(
            request_id, state, principal, body, decision, target, options,
        );
    }
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let Some(store) = state.approval_store.as_ref() else {
        audit_decision(
            request_id,
            &principal,
            Some(&access.mapping),
            &body,
            &decision,
            AuditDecision::Deny,
            None,
        );
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            &ApprovalError::NotConfigured.to_string(),
        );
    };
    audit_decision(
        request_id,
        &principal,
        Some(&access.mapping),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    let approval = match store.consume(&body, &principal, &access.mapping) {
        Ok(approval) => approval,
        Err(err) => {
            audit_decision(
                request_id,
                &principal,
                Some(&access.mapping),
                &body,
                &decision,
                AuditDecision::Deny,
                None,
            );
            return error_response(StatusCode::FORBIDDEN, request_id, &err.to_string());
        }
    };
    let (pull_request_result, forgejo_status) = match access
        .forgejo
        .create_pull_request(&access.token, &target, &options)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            audit_decision(
                request_id,
                &principal,
                Some(&access.mapping),
                &body,
                &decision,
                AuditDecision::Deny,
                None,
            );
            return forgejo_error_response(request_id, "Forgejo pull-request creation failed", err);
        }
    };
    audit_success(
        request_id,
        &principal,
        &access.mapping,
        &body,
        &decision,
        forgejo_status,
    );
    let result = serde_json::json!({
        "approval": approval,
        "pull_request": pull_request_result.pull_request,
        "resource_uri": pull_request_result.resource_uri,
        "requested_reviewers": pull_request_result.requested_reviewers,
        "reviewer_request_status": pull_request_result.reviewer_request_status,
        "reviewer_request_error": pull_request_result.reviewer_request_error,
    });
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation.clone(),
            allowed: true,
            reason: "approval consumed and pull request created by Forgejo".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result: Some(result),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

fn create_pull_request_preview_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
    target: RepositoryTarget,
    options: CreatePullRequestOptions,
) -> axum::response::Response {
    let mapping = state
        .principal_mapper
        .as_ref()
        .and_then(|mapper| mapper.resolve(&principal).ok().cloned());
    audit_decision(
        request_id,
        &principal,
        mapping.as_ref(),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: mapping
                .as_ref()
                .map(|mapping| mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.as_ref().and_then(|mapping| mapping.forgejo_user_id),
            trusted_delegation_headers: mapping
                .as_ref()
                .map(|mapping| state.trusted_headers.delegated_headers(mapping))
                .unwrap_or_default(),
            operation: body.operation.clone(),
            allowed: true,
            reason: "dry-run preview only; no Forgejo mutation executed".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "dry_run": true,
                "would_execute": false,
                "operation": "create_pull_request",
                "resource_uri": target.resource_uri(),
                "pull_request_options": options,
                "approval_required": true,
                "approval_store_configured": state.approval_store.is_some(),
            })),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

async fn merge_pull_request_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let target = match parse_numbered_target(&body, request_id) {
        Ok(target) => target,
        Err(response) => return response,
    };
    let options = match MergePullRequestOptions::from_body(body.body.as_deref()) {
        Ok(options) => options,
        Err(err) => return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()),
    };
    if body.dry_run {
        return merge_pull_request_preview_response(
            request_id, state, principal, body, decision, target, options,
        );
    }
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let Some(store) = state.approval_store.as_ref() else {
        audit_decision(
            request_id,
            &principal,
            Some(&access.mapping),
            &body,
            &decision,
            AuditDecision::Deny,
            None,
        );
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            &ApprovalError::NotConfigured.to_string(),
        );
    };
    audit_decision(
        request_id,
        &principal,
        Some(&access.mapping),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    let approval = match store.consume(&body, &principal, &access.mapping) {
        Ok(approval) => approval,
        Err(err) => {
            audit_decision(
                request_id,
                &principal,
                Some(&access.mapping),
                &body,
                &decision,
                AuditDecision::Deny,
                None,
            );
            return error_response(StatusCode::FORBIDDEN, request_id, &err.to_string());
        }
    };
    let (merge_result, forgejo_status) = match access
        .forgejo
        .merge_pull_request(&access.token, &target, &options)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            audit_decision(
                request_id,
                &principal,
                Some(&access.mapping),
                &body,
                &decision,
                AuditDecision::Deny,
                None,
            );
            return forgejo_error_response(request_id, "Forgejo pull-request merge failed", err);
        }
    };
    audit_success(
        request_id,
        &principal,
        &access.mapping,
        &body,
        &decision,
        forgejo_status,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation.clone(),
            allowed: true,
            reason: "approval consumed and pull request merged by Forgejo".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "approval": approval,
                "merge": merge_result,
            })),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

fn merge_pull_request_preview_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
    target: NumberedTarget,
    options: MergePullRequestOptions,
) -> axum::response::Response {
    let mapping = state
        .principal_mapper
        .as_ref()
        .and_then(|mapper| mapper.resolve(&principal).ok().cloned());
    audit_decision(
        request_id,
        &principal,
        mapping.as_ref(),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: mapping
                .as_ref()
                .map(|mapping| mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.as_ref().and_then(|mapping| mapping.forgejo_user_id),
            trusted_delegation_headers: mapping
                .as_ref()
                .map(|mapping| state.trusted_headers.delegated_headers(mapping))
                .unwrap_or_default(),
            operation: body.operation,
            allowed: true,
            reason: "dry-run preview only; no Forgejo mutation executed".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "dry_run": true,
                "would_execute": false,
                "operation": "merge_pull_request",
                "resource_uri": format!(
                    "forgejo://pull/{}/{}/{}",
                    target.repository.owner, target.repository.repo, target.number
                ),
                "merge_options": options,
                "approval_required": true,
                "approval_store_configured": state.approval_store.is_some(),
            })),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

async fn create_release_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let target = match parse_repository_target(&body, request_id) {
        Ok(target) => target,
        Err(response) => return response,
    };
    let options = match CreateReleaseOptions::from_body(body.body.as_deref()) {
        Ok(options) => options,
        Err(err) => return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()),
    };
    if body.dry_run {
        return create_release_preview_response(
            request_id, state, principal, body, decision, target, options,
        );
    }
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let Some(store) = state.approval_store.as_ref() else {
        audit_decision(
            request_id,
            &principal,
            Some(&access.mapping),
            &body,
            &decision,
            AuditDecision::Deny,
            None,
        );
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            &ApprovalError::NotConfigured.to_string(),
        );
    };
    audit_decision(
        request_id,
        &principal,
        Some(&access.mapping),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    let approval = match store.consume(&body, &principal, &access.mapping) {
        Ok(approval) => approval,
        Err(err) => {
            audit_decision(
                request_id,
                &principal,
                Some(&access.mapping),
                &body,
                &decision,
                AuditDecision::Deny,
                None,
            );
            return error_response(StatusCode::FORBIDDEN, request_id, &err.to_string());
        }
    };
    let (release_result, forgejo_status) = match access
        .forgejo
        .create_release(&access.token, &target, &options)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            audit_decision(
                request_id,
                &principal,
                Some(&access.mapping),
                &body,
                &decision,
                AuditDecision::Deny,
                None,
            );
            return forgejo_error_response(request_id, "Forgejo release creation failed", err);
        }
    };
    audit_success(
        request_id,
        &principal,
        &access.mapping,
        &body,
        &decision,
        forgejo_status,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation,
            allowed: true,
            reason: "approval consumed and release created by Forgejo".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "approval": approval,
                "release": release_result,
            })),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

fn create_release_preview_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
    target: RepositoryTarget,
    options: CreateReleaseOptions,
) -> axum::response::Response {
    let mapping = state
        .principal_mapper
        .as_ref()
        .and_then(|mapper| mapper.resolve(&principal).ok().cloned());
    audit_decision(
        request_id,
        &principal,
        mapping.as_ref(),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: mapping
                .as_ref()
                .map(|mapping| mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.as_ref().and_then(|mapping| mapping.forgejo_user_id),
            trusted_delegation_headers: mapping
                .as_ref()
                .map(|mapping| state.trusted_headers.delegated_headers(mapping))
                .unwrap_or_default(),
            operation: body.operation,
            allowed: true,
            reason: "dry-run preview only; no Forgejo mutation executed".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "dry_run": true,
                "would_execute": false,
                "operation": "create_release",
                "resource_uri": format!(
                    "forgejo://release/{}/{}/{}",
                    target.owner, target.repo, options.tag_name
                ),
                "release_options": options,
                "approval_required": true,
                "approval_store_configured": state.approval_store.is_some(),
            })),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

async fn create_issue_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let target = match parse_repository_target(&body, request_id) {
        Ok(target) => target,
        Err(response) => return response,
    };
    let options = match CreateIssueOptions::from_body(body.body.as_deref()) {
        Ok(options) => options,
        Err(err) => return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()),
    };
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let (issue_result, forgejo_status) = match access
        .forgejo
        .create_issue(&access.token, &target, &options)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            return forgejo_error_response(request_id, "Forgejo issue creation failed", err);
        }
    };
    audit_success(
        request_id,
        &principal,
        &access.mapping,
        &body,
        &decision,
        forgejo_status,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation,
            allowed: true,
            reason: "required scope present and issue created by Forgejo".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: false,
            target: body.target,
            repository: None,
            result: Some(serde_json::to_value(issue_result).unwrap_or_default()),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

async fn get_wiki_page_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let target = match parse_repository_target(&body, request_id) {
        Ok(target) => target,
        Err(response) => return response,
    };
    let Some(page_name) = body.query.as_deref().or(body.state.as_deref()) else {
        return error_response(
            StatusCode::BAD_REQUEST,
            request_id,
            "query is required as the wiki page name for get_wiki_page",
        );
    };
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let (page, forgejo_status) = match access
        .forgejo
        .get_wiki_page(&access.token, &target, page_name)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            return forgejo_error_response(request_id, "Forgejo wiki page lookup failed", err);
        }
    };
    audit_success(
        request_id,
        &principal,
        &access.mapping,
        &body,
        &decision,
        forgejo_status,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation,
            allowed: true,
            reason: "required scope present and Forgejo wiki page returned".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: false,
            target: body.target,
            repository: None,
            result: Some(serde_json::to_value(page).unwrap_or_default()),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

async fn write_wiki_page_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let target = match parse_repository_target(&body, request_id) {
        Ok(target) => target,
        Err(response) => return response,
    };
    let options = match WikiPageOptions::from_body(body.body.as_deref()) {
        Ok(options) => options,
        Err(err) => return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()),
    };
    if body.dry_run {
        return write_wiki_page_preview_response(
            request_id, state, principal, body, decision, target, options,
        );
    }
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let Some(store) = state.approval_store.as_ref() else {
        audit_decision(
            request_id,
            &principal,
            Some(&access.mapping),
            &body,
            &decision,
            AuditDecision::Deny,
            None,
        );
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            &ApprovalError::NotConfigured.to_string(),
        );
    };
    audit_decision(
        request_id,
        &principal,
        Some(&access.mapping),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    let approval = match store.consume(&body, &principal, &access.mapping) {
        Ok(approval) => approval,
        Err(err) => {
            audit_decision(
                request_id,
                &principal,
                Some(&access.mapping),
                &body,
                &decision,
                AuditDecision::Deny,
                None,
            );
            return error_response(StatusCode::FORBIDDEN, request_id, &err.to_string());
        }
    };
    let write_result = if body.operation == "create_wiki_page" {
        access
            .forgejo
            .create_wiki_page(&access.token, &target, &options)
            .await
    } else {
        access
            .forgejo
            .update_wiki_page(&access.token, &target, &options)
            .await
    };
    let (page, forgejo_status) = match write_result {
        Ok(result) => result,
        Err(err) => return forgejo_error_response(request_id, "Forgejo wiki write failed", err),
    };
    audit_success(
        request_id,
        &principal,
        &access.mapping,
        &body,
        &decision,
        forgejo_status,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation,
            allowed: true,
            reason: "approval consumed and wiki page written by Forgejo".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "approval": approval,
                "wiki_page": page,
            })),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

fn write_wiki_page_preview_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
    target: RepositoryTarget,
    options: WikiPageOptions,
) -> axum::response::Response {
    let mapping = state
        .principal_mapper
        .as_ref()
        .and_then(|mapper| mapper.resolve(&principal).ok().cloned());
    audit_decision(
        request_id,
        &principal,
        mapping.as_ref(),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: mapping
                .as_ref()
                .map(|mapping| mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.as_ref().and_then(|mapping| mapping.forgejo_user_id),
            trusted_delegation_headers: mapping
                .as_ref()
                .map(|mapping| state.trusted_headers.delegated_headers(mapping))
                .unwrap_or_default(),
            operation: body.operation.clone(),
            allowed: true,
            reason: "dry-run preview only; no Forgejo mutation executed".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "dry_run": true,
                "would_execute": false,
                "operation": body.operation,
                "resource_uri": format!("forgejo://wiki-page/{}/{}/{}", target.owner, target.repo, options.title),
                "wiki_page": {
                    "title": options.title,
                    "has_content_base64": true,
                    "message": options.message,
                },
                "approval_required": true,
                "approval_store_configured": state.approval_store.is_some(),
            })),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

fn credential_reference_status_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let mapping = match principal_mapping(&state, &principal, request_id) {
        Ok(mapping) => mapping,
        Err(response) => return response,
    };
    let token_env = mapping.api_token_env.clone();
    let token_env_present = token_env
        .as_deref()
        .and_then(|name| std::env::var(name).ok())
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    audit_decision(
        request_id,
        &principal,
        Some(&mapping),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&mapping),
            operation: body.operation,
            allowed: true,
            reason: "mapped credential reference status returned without secret values".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: false,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "issuer": mapping.issuer,
                "subject": mapping.subject,
                "principal_type": mapping.principal_type,
                "forgejo_login": mapping.forgejo_login,
                "forgejo_user_id": mapping.forgejo_user_id,
                "api_token_env": token_env,
                "api_token_env_present": token_env_present,
                "secret_value_returned": false,
            })),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

async fn repository_metadata_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let target_text = match body.target.as_deref() {
        Some(target) => target,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                request_id,
                "target is required for list_repository_metadata",
            );
        }
    };
    let target = match RepositoryTarget::parse(target_text) {
        Ok(target) => target,
        Err(err) => return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()),
    };
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let (repository, forgejo_status) = match access
        .forgejo
        .repository_metadata(&access.token, &target)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            return forgejo_error_response(request_id, "Forgejo metadata lookup failed", err);
        }
    };
    let event = AuditEvent {
        request_id,
        issuer: principal.issuer.clone(),
        subject: principal.subject.clone(),
        oauth_client: principal.oauth_client.clone(),
        principal_type: PrincipalType::from(access.mapping.principal_type),
        forgejo_user_id: access.mapping.forgejo_user_id,
        forgejo_login: Some(access.mapping.forgejo_login.clone()),
        tool: body.operation.clone(),
        target: body.target.clone(),
        risk: decision.risk,
        decision: AuditDecision::Allow,
        approval_id: None,
        forgejo_status: Some(forgejo_status),
        duration_ms: 0,
        response_bytes: 0,
    };
    info!(audit = %serde_json::to_string(&event).unwrap_or_default(), "audit event");
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation,
            allowed: true,
            reason: "required scope present and Forgejo metadata returned".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: decision.approval_required,
            target: body.target,
            repository: Some(repository),
            result: None,
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

fn forgejo_api_coverage_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let page =
        match PageRequest::from_cursor(body.cursor.as_deref(), body.limit, state.max_page_limit) {
            Ok(page) => page,
            Err(err) => {
                return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string());
            }
        };
    let catalog = match policy::ForgejoApiCatalog::current() {
        Ok(catalog) => catalog,
        Err(err) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                request_id,
                &err.to_string(),
            );
        }
    };
    let query_ref = body.query.as_deref().or(body.target.as_deref());
    let filtered = catalog.filtered_endpoints(body.state.as_deref(), query_ref);
    let start = ((page.page - 1) * page.limit) as usize;
    let end = start
        .saturating_add(page.limit as usize)
        .min(filtered.len());
    let endpoints = if start < filtered.len() {
        filtered[start..end].to_vec()
    } else {
        Vec::new()
    };
    let next_cursor = if end < filtered.len() {
        Some((page.page + 1).to_string())
    } else {
        None
    };
    let mapping = state
        .principal_mapper
        .as_ref()
        .and_then(|mapper| mapper.resolve(&principal).ok().cloned());
    audit_decision(
        request_id,
        &principal,
        mapping.as_ref(),
        &body,
        &decision,
        AuditDecision::Allow,
        None,
    );
    let filter = body.state.clone();
    let query = body.query.clone().or_else(|| body.target.clone());
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: mapping
                .as_ref()
                .map(|mapping| mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.as_ref().and_then(|mapping| mapping.forgejo_user_id),
            trusted_delegation_headers: mapping
                .as_ref()
                .map(|mapping| state.trusted_headers.delegated_headers(mapping))
                .unwrap_or_default(),
            operation: body.operation,
            allowed: true,
            reason: "generated Forgejo API coverage returned; disabled endpoints are metadata only"
                .to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: false,
            target: body.target,
            repository: None,
            result: Some(serde_json::json!({
                "summary": catalog.summary(),
                "filter": filter,
                "query": query,
                "endpoints": endpoints,
            })),
            limit: Some(page.limit),
            next_cursor,
        }),
    )
        .into_response()
}

async fn phase2_list_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let page =
        match PageRequest::from_cursor(body.cursor.as_deref(), body.limit, state.max_page_limit) {
            Ok(page) => page,
            Err(err) => {
                return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string());
            }
        };
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let response = match body.operation.as_str() {
        "list_repository_issues" => {
            let target = match parse_repository_target(&body, request_id) {
                Ok(target) => target,
                Err(response) => return response,
            };
            match access
                .forgejo
                .list_issues(&access.token, &target, body.state.as_deref(), page)
                .await
            {
                Ok((page, status)) => (serde_json::to_value(&page).unwrap_or_default(), status),
                Err(err) => {
                    return forgejo_error_response(request_id, "Forgejo issue list failed", err);
                }
            }
        }
        "list_pull_requests" => {
            let target = match parse_repository_target(&body, request_id) {
                Ok(target) => target,
                Err(response) => return response,
            };
            match access
                .forgejo
                .list_pull_requests(&access.token, &target, body.state.as_deref(), page)
                .await
            {
                Ok((page, status)) => (serde_json::to_value(&page).unwrap_or_default(), status),
                Err(err) => {
                    return forgejo_error_response(
                        request_id,
                        "Forgejo pull-request list failed",
                        err,
                    );
                }
            }
        }
        "list_pull_request_reviews" => {
            let target = match parse_numbered_target(&body, request_id) {
                Ok(target) => target,
                Err(response) => return response,
            };
            match access
                .forgejo
                .list_pull_request_reviews(&access.token, &target, page)
                .await
            {
                Ok((page, status)) => (serde_json::to_value(&page).unwrap_or_default(), status),
                Err(err) => {
                    return forgejo_error_response(request_id, "Forgejo review list failed", err);
                }
            }
        }
        "list_releases" => {
            let target = match parse_repository_target(&body, request_id) {
                Ok(target) => target,
                Err(response) => return response,
            };
            match access
                .forgejo
                .list_releases(&access.token, &target, page)
                .await
            {
                Ok((page, status)) => (serde_json::to_value(&page).unwrap_or_default(), status),
                Err(err) => {
                    return forgejo_error_response(request_id, "Forgejo release list failed", err);
                }
            }
        }
        "list_wiki_pages" => {
            let target = match parse_repository_target(&body, request_id) {
                Ok(target) => target,
                Err(response) => return response,
            };
            match access
                .forgejo
                .list_wiki_pages(&access.token, &target, page)
                .await
            {
                Ok((page, status)) => (serde_json::to_value(&page).unwrap_or_default(), status),
                Err(err) => {
                    return forgejo_error_response(
                        request_id,
                        "Forgejo wiki page list failed",
                        err,
                    );
                }
            }
        }
        "list_notifications" => match access
            .forgejo
            .list_notifications(&access.token, body.state.as_deref(), page)
            .await
        {
            Ok((page, status)) => (serde_json::to_value(&page).unwrap_or_default(), status),
            Err(err) => {
                return forgejo_error_response(request_id, "Forgejo notification list failed", err);
            }
        },
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                request_id,
                "operation is not a Phase 2 list operation",
            );
        }
    };
    audit_success(
        request_id,
        &principal,
        &access.mapping,
        &body,
        &decision,
        response.1,
    );
    let next_cursor = response
        .0
        .get("next_cursor")
        .and_then(|value| value.as_str())
        .map(ToString::to_string);
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation,
            allowed: true,
            reason: "required scope present and Forgejo list returned".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: decision.approval_required,
            target: body.target,
            repository: None,
            result: Some(response.0),
            limit: Some(page.limit),
            next_cursor,
        }),
    )
        .into_response()
}

async fn create_issue_comment_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let target = match parse_numbered_target(&body, request_id) {
        Ok(target) => target,
        Err(response) => return response,
    };
    let Some(comment_body) = body.body.as_deref() else {
        return error_response(
            StatusCode::BAD_REQUEST,
            request_id,
            &ForgejoError::MissingCommentBody.to_string(),
        );
    };
    let access = match forgejo_access(&state, &principal, request_id) {
        Ok(access) => access,
        Err(response) => return response,
    };
    let (comment, forgejo_status) = match access
        .forgejo
        .create_issue_comment(&access.token, &target, comment_body)
        .await
    {
        Ok(result) => result,
        Err(err) => return forgejo_error_response(request_id, "Forgejo issue comment failed", err),
    };
    audit_success(
        request_id,
        &principal,
        &access.mapping,
        &body,
        &decision,
        forgejo_status,
    );
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(access.mapping.forgejo_login.clone()),
            forgejo_user_id: access.mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&access.mapping),
            operation: body.operation,
            allowed: true,
            reason: "required scope present and issue comment created".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: decision.approval_required,
            target: body.target,
            repository: None,
            result: Some(serde_json::to_value(comment).unwrap_or_default()),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

fn approval_required_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
    mapping: Option<&PrincipalMapping>,
) -> axum::response::Response {
    let approval_validation =
        body.approval_id
            .as_ref()
            .map(|_| match (state.approval_store.as_ref(), mapping) {
                (Some(store), Some(mapping)) => store.validate(&body, &principal, mapping),
                (None, _) => Err(ApprovalError::NotConfigured),
                (_, None) => Err(ApprovalError::PrincipalMismatch),
            });
    let status = match approval_validation.as_ref() {
        Some(Ok(_)) => StatusCode::ACCEPTED,
        Some(Err(_)) => StatusCode::FORBIDDEN,
        None => StatusCode::ACCEPTED,
    };
    let reason = match approval_validation.as_ref() {
        Some(Ok(_)) => "approval validated; operation execution is not implemented yet".to_string(),
        Some(Err(err)) => err.to_string(),
        None => "approval is required before this operation can execute".to_string(),
    };
    let result = approval_validation
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|validation| serde_json::to_value(validation).unwrap_or_default());
    let event = AuditEvent {
        request_id,
        issuer: principal.issuer.clone(),
        subject: principal.subject.clone(),
        oauth_client: principal.oauth_client.clone(),
        principal_type: mapping
            .map(|mapping| PrincipalType::from(mapping.principal_type))
            .unwrap_or(PrincipalType::Unknown),
        forgejo_user_id: mapping.and_then(|mapping| mapping.forgejo_user_id),
        forgejo_login: mapping.map(|mapping| mapping.forgejo_login.clone()),
        tool: body.operation.clone(),
        target: body.target.clone(),
        risk: decision.risk,
        decision: AuditDecision::Deny,
        approval_id: body.approval_id.clone(),
        forgejo_status: None,
        duration_ms: 0,
        response_bytes: 0,
    };
    info!(audit = %serde_json::to_string(&event).unwrap_or_default(), "audit event");
    (
        status,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: mapping.map(|mapping| mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.and_then(|mapping| mapping.forgejo_user_id),
            trusted_delegation_headers: mapping
                .map(|mapping| state.trusted_headers.delegated_headers(mapping))
                .unwrap_or_default(),
            operation: body.operation,
            allowed: false,
            reason,
            required_scope: decision.required_scope.to_string(),
            approval_required: true,
            target: body.target,
            repository: None,
            result,
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

fn create_approval_response(
    request_id: Uuid,
    state: AppState,
    principal: identity::Principal,
    body: McpProbeRequest,
    decision: policy::PolicyDecision,
) -> axum::response::Response {
    let Some(requested_operation) = body.requested_operation.as_deref() else {
        return error_response(
            StatusCode::BAD_REQUEST,
            request_id,
            "requested_operation is required for create_approval",
        );
    };
    let operation = match state.registry.operation(requested_operation) {
        Ok(operation) => operation,
        Err(err) => return error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()),
    };
    if !operation.approval_required {
        return error_response(
            StatusCode::BAD_REQUEST,
            request_id,
            "requested_operation does not require approval",
        );
    }
    if body.target.as_deref().unwrap_or_default().trim().is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            request_id,
            "target is required for create_approval",
        );
    }
    let Some(store) = state.approval_store.as_ref() else {
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            &ApprovalError::NotConfigured.to_string(),
        );
    };
    let mapping = match principal_mapping(&state, &principal, request_id) {
        Ok(mapping) => mapping,
        Err(response) => return response,
    };
    let grant = match store.create(requested_operation, &body, &principal, &mapping) {
        Ok(grant) => grant,
        Err(err) => {
            return error_response(
                StatusCode::SERVICE_UNAVAILABLE,
                request_id,
                &err.to_string(),
            );
        }
    };
    let event = AuditEvent {
        request_id,
        issuer: principal.issuer.clone(),
        subject: principal.subject.clone(),
        oauth_client: principal.oauth_client.clone(),
        principal_type: PrincipalType::from(mapping.principal_type),
        forgejo_user_id: mapping.forgejo_user_id,
        forgejo_login: Some(mapping.forgejo_login.clone()),
        tool: body.operation.clone(),
        target: body.target.clone(),
        risk: decision.risk,
        decision: AuditDecision::Allow,
        approval_id: Some(grant.approval_id.to_string()),
        forgejo_status: None,
        duration_ms: 0,
        response_bytes: 0,
    };
    info!(audit = %serde_json::to_string(&event).unwrap_or_default(), "audit event");
    (
        StatusCode::OK,
        Json(McpResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            forgejo_login: Some(mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(&mapping),
            operation: body.operation,
            allowed: true,
            reason: "short-lived approval record created".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: false,
            target: body.target,
            repository: None,
            result: Some(serde_json::to_value(grant).unwrap_or_default()),
            limit: None,
            next_cursor: None,
        }),
    )
        .into_response()
}

#[allow(clippy::result_large_err)]
fn principal_mapping(
    state: &AppState,
    principal: &identity::Principal,
    request_id: Uuid,
) -> Result<PrincipalMapping, axum::response::Response> {
    let Some(mapper) = &state.principal_mapper else {
        return Err(error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            "principal mapping is not configured",
        ));
    };
    mapper
        .resolve(principal)
        .cloned()
        .map_err(|err| error_response(StatusCode::FORBIDDEN, request_id, &err.to_string()))
}

#[allow(clippy::result_large_err)]
fn forgejo_access(
    state: &AppState,
    principal: &identity::Principal,
    request_id: Uuid,
) -> Result<ForgejoAccess, axum::response::Response> {
    let mapping = principal_mapping(state, principal, request_id)?;
    let Some(forgejo) = &state.forgejo else {
        return Err(error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            "Forgejo API URL is not configured",
        ));
    };
    let Some(token_env) = mapping.api_token_env.as_deref() else {
        return Err(error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            &ForgejoError::MissingTokenEnv.to_string(),
        ));
    };
    let token = match std::env::var(token_env) {
        Ok(token) if !token.trim().is_empty() => token,
        _ => {
            return Err(error_response(
                StatusCode::SERVICE_UNAVAILABLE,
                request_id,
                &ForgejoError::MissingToken.to_string(),
            ));
        }
    };
    Ok(ForgejoAccess {
        mapping,
        forgejo: forgejo.clone(),
        token,
    })
}

#[allow(clippy::result_large_err)]
fn parse_repository_target(
    body: &McpProbeRequest,
    request_id: Uuid,
) -> Result<RepositoryTarget, axum::response::Response> {
    let Some(target) = body.target.as_deref() else {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            request_id,
            "repository target is required",
        ));
    };
    RepositoryTarget::parse(target)
        .map_err(|err| error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()))
}

#[allow(clippy::result_large_err)]
fn parse_numbered_target(
    body: &McpProbeRequest,
    request_id: Uuid,
) -> Result<NumberedTarget, axum::response::Response> {
    let Some(target) = body.target.as_deref() else {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            request_id,
            "numbered target is required",
        ));
    };
    NumberedTarget::parse(target)
        .map_err(|err| error_response(StatusCode::BAD_REQUEST, request_id, &err.to_string()))
}

fn forgejo_error_response(
    request_id: Uuid,
    prefix: &str,
    err: ForgejoError,
) -> axum::response::Response {
    match err {
        ForgejoError::Api { status, body } => {
            let status = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            error_response(status, request_id, &format!("{prefix}: {body}"))
        }
        err => error_response(StatusCode::BAD_GATEWAY, request_id, &err.to_string()),
    }
}

fn audit_success(
    request_id: Uuid,
    principal: &identity::Principal,
    mapping: &PrincipalMapping,
    body: &McpProbeRequest,
    decision: &policy::PolicyDecision,
    forgejo_status: u16,
) {
    let event = AuditEvent {
        request_id,
        issuer: principal.issuer.clone(),
        subject: principal.subject.clone(),
        oauth_client: principal.oauth_client.clone(),
        principal_type: PrincipalType::from(mapping.principal_type),
        forgejo_user_id: mapping.forgejo_user_id,
        forgejo_login: Some(mapping.forgejo_login.clone()),
        tool: body.operation.clone(),
        target: body.target.clone(),
        risk: decision.risk,
        decision: AuditDecision::Allow,
        approval_id: body.approval_id.clone(),
        forgejo_status: Some(forgejo_status),
        duration_ms: 0,
        response_bytes: 0,
    };
    info!(audit = %serde_json::to_string(&event).unwrap_or_default(), "audit event");
}

fn audit_decision(
    request_id: Uuid,
    principal: &identity::Principal,
    mapping: Option<&PrincipalMapping>,
    body: &McpProbeRequest,
    decision: &policy::PolicyDecision,
    audit_decision: AuditDecision,
    forgejo_status: Option<u16>,
) {
    let event = AuditEvent {
        request_id,
        issuer: principal.issuer.clone(),
        subject: principal.subject.clone(),
        oauth_client: principal.oauth_client.clone(),
        principal_type: mapping
            .map(|mapping| PrincipalType::from(mapping.principal_type))
            .unwrap_or(PrincipalType::Unknown),
        forgejo_user_id: mapping.and_then(|mapping| mapping.forgejo_user_id),
        forgejo_login: mapping.map(|mapping| mapping.forgejo_login.clone()),
        tool: body.operation.clone(),
        target: body.target.clone(),
        risk: decision.risk,
        decision: audit_decision,
        approval_id: body.approval_id.clone(),
        forgejo_status,
        duration_ms: 0,
        response_bytes: 0,
    };
    info!(audit = %serde_json::to_string(&event).unwrap_or_default(), "audit event");
}

fn error_response(status: StatusCode, request_id: Uuid, error: &str) -> axum::response::Response {
    (
        status,
        Json(serde_json::json!({ "request_id": request_id, "error": error })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::validate_tls_urls;

    #[test]
    fn tls_flag_requires_https_resource() {
        let err = validate_tls_urls(true, "http://127.0.0.1:7080/mcp", None).unwrap_err();
        assert!(err.to_string().contains("--resource"));
    }

    #[test]
    fn tls_flag_requires_https_forgejo_url_when_configured() {
        let err = validate_tls_urls(
            true,
            "https://forgejo.example.org/mcp",
            Some("http://forgejo.example.org"),
        )
        .unwrap_err();
        assert!(err.to_string().contains("--forgejo-url"));
    }

    #[test]
    fn tls_flag_accepts_https_urls() {
        validate_tls_urls(
            true,
            "https://forgejo.example.org/mcp",
            Some("https://forgejo.example.org"),
        )
        .unwrap();
    }
}
