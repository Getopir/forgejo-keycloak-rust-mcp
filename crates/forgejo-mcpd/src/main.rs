// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::Context;
use audit::{AuditDecision, AuditEvent, PrincipalType};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json};
use axum::routing::{get, post};
use axum::{Router, serve};
use clap::Parser;
use forgejo::{ForgejoClient, ForgejoError, RepositoryMetadata, RepositoryTarget};
use identity::JwtValidator;
use policy::OperationRegistry;
use principal::{DelegatedHeader, PrincipalMapper, TrustedHeaderConfig};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};
use uuid::Uuid;

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

#[derive(Debug, Deserialize)]
struct McpProbeRequest {
    #[serde(default = "default_operation")]
    operation: String,
    #[serde(default)]
    target: Option<String>,
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
        registry: OperationRegistry::phase0(),
        issuer: args.issuer,
        resource: args.resource,
        principal_mapper,
        forgejo,
        trusted_headers: TrustedHeaderConfig::new(
            args.trusted_user_header,
            args.trusted_email_header,
            args.trusted_full_name_header,
        ),
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
        .route("/mcp", post(mcp_handler))
        .with_state(state);

    let listener = TcpListener::bind(args.bind).await?;
    info!(addr = %args.bind, "forgejo-mcpd listening");
    serve(listener, app).await?;
    Ok(())
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
        .collect::<Vec<_>>();
    Json(ProtectedResourceMetadata {
        resource: state.resource.clone(),
        authorization_servers: vec![state.issuer.clone()],
        bearer_methods_supported: vec!["header"],
        scopes_supported: scopes,
        resource_signing_alg_values_supported: vec!["RS256"],
    })
}

async fn mcp_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<McpProbeRequest>,
) -> impl IntoResponse {
    let request_id = Uuid::now_v7();
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
        .and_then(|mapper| mapper.resolve(&principal).ok());
    if body.operation == "list_repository_metadata" && decision.allowed {
        return repository_metadata_response(request_id, state, principal, body, decision).await;
    }
    // Audit records intentionally include identity and policy metadata only.
    // Raw bearer tokens and downstream service credentials must never be logged.
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
            forgejo_login: mapping.map(|mapping| mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.and_then(|mapping| mapping.forgejo_user_id),
            trusted_delegation_headers: mapping
                .map(|mapping| state.trusted_headers.delegated_headers(mapping))
                .unwrap_or_default(),
            operation: body.operation,
            allowed: decision.allowed,
            reason: decision.reason,
            required_scope: decision.required_scope.to_string(),
            approval_required: decision.approval_required,
            target: body.target,
            repository: None,
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
    let Some(mapper) = &state.principal_mapper else {
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            "principal mapping is not configured",
        );
    };
    let mapping = match mapper.resolve(&principal) {
        Ok(mapping) => mapping,
        Err(err) => return error_response(StatusCode::FORBIDDEN, request_id, &err.to_string()),
    };
    let Some(forgejo) = &state.forgejo else {
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            "Forgejo API URL is not configured",
        );
    };
    let Some(token_env) = mapping.api_token_env.as_deref() else {
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            request_id,
            &ForgejoError::MissingTokenEnv.to_string(),
        );
    };
    let token = match std::env::var(token_env) {
        Ok(token) if !token.trim().is_empty() => token,
        _ => {
            return error_response(
                StatusCode::SERVICE_UNAVAILABLE,
                request_id,
                &ForgejoError::MissingToken.to_string(),
            );
        }
    };
    let (repository, forgejo_status) = match forgejo.repository_metadata(&token, &target).await {
        Ok(result) => result,
        Err(ForgejoError::Api { status, body }) => {
            let status = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            return error_response(
                status,
                request_id,
                &format!("Forgejo metadata lookup failed: {body}"),
            );
        }
        Err(err) => return error_response(StatusCode::BAD_GATEWAY, request_id, &err.to_string()),
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
            forgejo_login: Some(mapping.forgejo_login.clone()),
            forgejo_user_id: mapping.forgejo_user_id,
            trusted_delegation_headers: state.trusted_headers.delegated_headers(mapping),
            operation: body.operation,
            allowed: true,
            reason: "required scope present and Forgejo metadata returned".to_string(),
            required_scope: decision.required_scope.to_string(),
            approval_required: decision.approval_required,
            target: body.target,
            repository: Some(repository),
        }),
    )
        .into_response()
}

fn error_response(status: StatusCode, request_id: Uuid, error: &str) -> axum::response::Response {
    (
        status,
        Json(serde_json::json!({ "request_id": request_id, "error": error })),
    )
        .into_response()
}
