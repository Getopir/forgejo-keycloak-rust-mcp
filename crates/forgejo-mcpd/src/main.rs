// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::Context;
use audit::{AuditDecision, AuditEvent, PrincipalType};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json};
use axum::routing::{get, post};
use axum::{Router, serve};
use clap::Parser;
use identity::JwtValidator;
use policy::OperationRegistry;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};
use uuid::Uuid;

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
}

#[derive(Clone)]
struct AppState {
    validator: Arc<JwtValidator>,
    registry: OperationRegistry,
    issuer: String,
    resource: String,
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
struct McpProbeResponse {
    request_id: Uuid,
    subject: String,
    oauth_client: Option<String>,
    preferred_username: Option<String>,
    operation: String,
    allowed: bool,
    reason: String,
    required_scope: String,
    approval_required: bool,
    target: Option<String>,
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
    let state = AppState {
        validator: Arc::new(validator),
        registry: OperationRegistry::phase0(),
        issuer: args.issuer,
        resource: args.resource,
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
        .route("/mcp", post(mcp_probe))
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

async fn mcp_probe(
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
    // Audit records intentionally include identity and policy metadata only.
    // Raw bearer tokens and downstream service credentials must never be logged.
    let event = AuditEvent {
        request_id,
        issuer: principal.issuer.clone(),
        subject: principal.subject.clone(),
        oauth_client: principal.oauth_client.clone(),
        principal_type: PrincipalType::Unknown,
        forgejo_user_id: None,
        forgejo_login: None,
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
        Json(McpProbeResponse {
            request_id,
            subject: principal.subject,
            oauth_client: principal.oauth_client,
            preferred_username: principal.preferred_username,
            operation: body.operation,
            allowed: decision.allowed,
            reason: decision.reason,
            required_scope: decision.required_scope.to_string(),
            approval_required: decision.approval_required,
            target: body.target,
        }),
    )
        .into_response()
}
