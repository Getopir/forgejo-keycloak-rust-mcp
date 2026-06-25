// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const FORGEJO_API_SPEC_JSON: &str =
    include_str!("../../../vendor/forgejo-api/forgejo-15.0.3-gitea-1.22.0-swagger.v1.json");
const FORGEJO_API_SPEC_SHA256: &str =
    "a90f2fe1266a7a08dfcf682cd28db96c364e18a7de2a4e559a26afe3485bb26f";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskClass {
    ReadPublic,
    ReadPrivate,
    WriteAdditive,
    WriteMutating,
    Destructive,
    NetworkEgress,
    Secret,
    SiteAdmin,
    LongRunning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub name: &'static str,
    pub scope: &'static str,
    pub risk: RiskClass,
    pub approval_required: bool,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointExposure {
    SemanticOverlay,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointTargetType {
    ActivityPub,
    Admin,
    Issue,
    Notification,
    Organization,
    PullRequest,
    Release,
    Repository,
    Settings,
    User,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgejoApiEndpoint {
    pub method: String,
    pub path: String,
    pub operation_id: Option<String>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub target_type: EndpointTargetType,
    pub risk: RiskClass,
    pub approval_required: bool,
    pub exposure: EndpointExposure,
    pub semantic_operation: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ForgejoApiCatalog {
    pub source_version: String,
    pub source_sha256: &'static str,
    pub endpoint_count: usize,
    pub endpoints: Vec<ForgejoApiEndpoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ForgejoApiCoverageSummary {
    pub source_version: String,
    pub source_sha256: &'static str,
    pub endpoint_count: usize,
    pub semantic_overlay_count: usize,
    pub disabled_count: usize,
    pub approval_required_count: usize,
    pub destructive_count: usize,
    pub admin_count: usize,
    pub by_risk: BTreeMap<String, usize>,
    pub by_target_type: BTreeMap<String, usize>,
}

#[derive(Debug, thiserror::Error)]
pub enum ForgejoApiCatalogError {
    #[error("pinned Forgejo API spec could not be parsed: {0}")]
    Parse(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: String,
    pub required_scope: &'static str,
    pub risk: RiskClass,
    pub approval_required: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("unknown operation: {0}")]
    UnknownOperation(String),
}

#[derive(Debug, Clone)]
pub struct OperationRegistry {
    operations: BTreeMap<&'static str, Operation>,
}

impl OperationRegistry {
    pub fn current() -> Self {
        // Keep the registry explicit: every exposed operation has a required
        // OAuth scope, risk class, and approval flag that clients can test
        // deterministically before any Forgejo request is made.
        let operations = [
            Operation {
                name: "gateway_probe",
                scope: "forgejo:repo:read",
                risk: RiskClass::ReadPrivate,
                approval_required: false,
                description: "Authenticate the caller and return bounded gateway identity metadata.",
            },
            Operation {
                name: "list_repository_metadata",
                scope: "forgejo:repo:read",
                risk: RiskClass::ReadPrivate,
                approval_required: false,
                description: "Read repository metadata through mapped Forgejo identity.",
            },
            Operation {
                name: "list_repository_issues",
                scope: "forgejo:issue:read",
                risk: RiskClass::ReadPrivate,
                approval_required: false,
                description: "List bounded issue summaries through mapped Forgejo identity.",
            },
            Operation {
                name: "create_issue_comment",
                scope: "forgejo:issue:write",
                risk: RiskClass::WriteAdditive,
                approval_required: false,
                description: "Add an issue or pull-request conversation comment.",
            },
            Operation {
                name: "list_pull_requests",
                scope: "forgejo:pr:read",
                risk: RiskClass::ReadPrivate,
                approval_required: false,
                description: "List bounded pull-request summaries through mapped Forgejo identity.",
            },
            Operation {
                name: "list_pull_request_reviews",
                scope: "forgejo:pr:read",
                risk: RiskClass::ReadPrivate,
                approval_required: false,
                description: "List bounded pull-request review summaries through mapped Forgejo identity.",
            },
            Operation {
                name: "list_releases",
                scope: "forgejo:release:read",
                risk: RiskClass::ReadPrivate,
                approval_required: false,
                description: "List bounded repository release summaries through mapped Forgejo identity.",
            },
            Operation {
                name: "list_notifications",
                scope: "forgejo:notification:read",
                risk: RiskClass::ReadPrivate,
                approval_required: false,
                description: "List bounded notification summaries for the mapped Forgejo principal.",
            },
            Operation {
                name: "forgejo_api_coverage",
                scope: "forgejo:repo:read",
                risk: RiskClass::ReadPrivate,
                approval_required: false,
                description: "Return bounded generated Forgejo API endpoint classification and coverage metadata.",
            },
            Operation {
                name: "create_approval",
                scope: "forgejo:approval:grant",
                risk: RiskClass::WriteMutating,
                approval_required: false,
                description: "Create a short-lived approval record bound to one exact high-risk operation payload.",
            },
            Operation {
                name: "create_release",
                scope: "forgejo:release:write",
                risk: RiskClass::WriteMutating,
                approval_required: true,
                description: "Create or publish a repository release after exact-payload approval.",
            },
            Operation {
                name: "merge_pull_request",
                scope: "forgejo:pr:merge",
                risk: RiskClass::WriteMutating,
                approval_required: true,
                description: "Merge a pull request after policy and Forgejo ACL checks.",
            },
            Operation {
                name: "delete_repository",
                scope: "forgejo:org:admin",
                risk: RiskClass::Destructive,
                approval_required: true,
                description: "High-risk repository deletion with exact-argument-bound approval.",
            },
        ]
        .into_iter()
        .map(|operation| (operation.name, operation))
        .collect();
        Self { operations }
    }

    pub fn phase0() -> Self {
        Self::current()
    }

    pub fn operation(&self, name: &str) -> Result<&Operation, PolicyError> {
        self.operations
            .get(name)
            .ok_or_else(|| PolicyError::UnknownOperation(name.to_string()))
    }

    pub fn operations(&self) -> impl Iterator<Item = &Operation> {
        self.operations.values()
    }

    pub fn decide(
        &self,
        name: &str,
        scopes: &BTreeSet<String>,
    ) -> Result<PolicyDecision, PolicyError> {
        let operation = self.operation(name)?;
        let allowed = scopes.contains(operation.scope);
        let reason = if allowed {
            "required scope present".to_string()
        } else {
            format!("missing required scope {}", operation.scope)
        };
        Ok(PolicyDecision {
            allowed,
            reason,
            required_scope: operation.scope,
            risk: operation.risk,
            approval_required: operation.approval_required,
        })
    }
}

impl ForgejoApiCatalog {
    pub fn current() -> Result<Self, ForgejoApiCatalogError> {
        let spec: serde_json::Value = serde_json::from_str(FORGEJO_API_SPEC_JSON)
            .map_err(|err| ForgejoApiCatalogError::Parse(err.to_string()))?;
        let source_version = spec
            .get("info")
            .and_then(|info| info.get("version"))
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string();
        let mut endpoints = Vec::new();
        let Some(paths) = spec.get("paths").and_then(|paths| paths.as_object()) else {
            return Err(ForgejoApiCatalogError::Parse(
                "OpenAPI spec has no paths object".to_string(),
            ));
        };
        for (path, item) in paths {
            let Some(methods) = item.as_object() else {
                continue;
            };
            for (method, operation) in methods {
                if !is_http_method(method) {
                    continue;
                }
                endpoints.push(classify_endpoint(method, path, operation));
            }
        }
        endpoints.sort_by(|left, right| {
            left.path
                .cmp(&right.path)
                .then_with(|| left.method.cmp(&right.method))
        });
        Ok(Self {
            source_version,
            source_sha256: FORGEJO_API_SPEC_SHA256,
            endpoint_count: endpoints.len(),
            endpoints,
        })
    }

    pub fn summary(&self) -> ForgejoApiCoverageSummary {
        let mut by_risk = BTreeMap::new();
        let mut by_target_type = BTreeMap::new();
        for endpoint in &self.endpoints {
            *by_risk
                .entry(format!("{:?}", endpoint.risk).to_ascii_lowercase())
                .or_insert(0) += 1;
            *by_target_type
                .entry(format!("{:?}", endpoint.target_type).to_ascii_lowercase())
                .or_insert(0) += 1;
        }
        ForgejoApiCoverageSummary {
            source_version: self.source_version.clone(),
            source_sha256: self.source_sha256,
            endpoint_count: self.endpoint_count,
            semantic_overlay_count: self
                .endpoints
                .iter()
                .filter(|endpoint| endpoint.exposure == EndpointExposure::SemanticOverlay)
                .count(),
            disabled_count: self
                .endpoints
                .iter()
                .filter(|endpoint| endpoint.exposure == EndpointExposure::Disabled)
                .count(),
            approval_required_count: self
                .endpoints
                .iter()
                .filter(|endpoint| endpoint.approval_required)
                .count(),
            destructive_count: self
                .endpoints
                .iter()
                .filter(|endpoint| endpoint.risk == RiskClass::Destructive)
                .count(),
            admin_count: self
                .endpoints
                .iter()
                .filter(|endpoint| endpoint.target_type == EndpointTargetType::Admin)
                .count(),
            by_risk,
            by_target_type,
        }
    }

    pub fn filtered_endpoints(
        &self,
        filter: Option<&str>,
        query: Option<&str>,
    ) -> Vec<ForgejoApiEndpoint> {
        let filter = filter.map(str::trim).filter(|value| !value.is_empty());
        let query = query
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_ascii_lowercase);
        self.endpoints
            .iter()
            .filter(|endpoint| match filter {
                Some("semantic_overlay") => endpoint.exposure == EndpointExposure::SemanticOverlay,
                Some("disabled") => endpoint.exposure == EndpointExposure::Disabled,
                Some("approval_required") => endpoint.approval_required,
                Some("destructive") => endpoint.risk == RiskClass::Destructive,
                Some("admin") => endpoint.target_type == EndpointTargetType::Admin,
                Some(value) => risk_name(endpoint.risk) == value,
                None => true,
            })
            .filter(|endpoint| {
                let Some(query) = &query else {
                    return true;
                };
                endpoint.path.to_ascii_lowercase().contains(query)
                    || endpoint.method.to_ascii_lowercase().contains(query)
                    || endpoint
                        .operation_id
                        .as_deref()
                        .unwrap_or_default()
                        .to_ascii_lowercase()
                        .contains(query)
                    || endpoint
                        .semantic_operation
                        .as_deref()
                        .unwrap_or_default()
                        .to_ascii_lowercase()
                        .contains(query)
            })
            .cloned()
            .collect()
    }
}

fn is_http_method(method: &str) -> bool {
    matches!(method, "get" | "post" | "put" | "patch" | "delete" | "head")
}

fn classify_endpoint(
    method: &str,
    path: &str,
    operation: &serde_json::Value,
) -> ForgejoApiEndpoint {
    let method = method.to_ascii_uppercase();
    let operation_id = operation
        .get("operationId")
        .and_then(|value| value.as_str())
        .map(ToString::to_string);
    let tags = operation
        .get("tags")
        .and_then(|value| value.as_array())
        .map(|tags| {
            tags.iter()
                .filter_map(|tag| tag.as_str().map(ToString::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let summary = operation
        .get("summary")
        .or_else(|| operation.get("description"))
        .and_then(|value| value.as_str())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let target_type = classify_target_type(path, &tags);
    let semantic_operation = semantic_operation(&method, path).map(ToString::to_string);
    let risk = classify_risk(&method, path, operation_id.as_deref(), target_type);
    let approval_required = matches!(
        risk,
        RiskClass::WriteMutating
            | RiskClass::Destructive
            | RiskClass::NetworkEgress
            | RiskClass::Secret
            | RiskClass::SiteAdmin
            | RiskClass::LongRunning
    );
    ForgejoApiEndpoint {
        method,
        path: path.to_string(),
        operation_id,
        tags,
        summary,
        target_type,
        risk,
        approval_required,
        exposure: if semantic_operation.is_some() {
            EndpointExposure::SemanticOverlay
        } else {
            EndpointExposure::Disabled
        },
        semantic_operation,
    }
}

fn classify_target_type(path: &str, tags: &[String]) -> EndpointTargetType {
    if path.contains("/admin") || tags.iter().any(|tag| tag == "admin") {
        EndpointTargetType::Admin
    } else if tags.iter().any(|tag| tag == "activitypub") || path.starts_with("/activitypub") {
        EndpointTargetType::ActivityPub
    } else if path.contains("/pulls") {
        EndpointTargetType::PullRequest
    } else if path.contains("/issues") {
        EndpointTargetType::Issue
    } else if path.contains("/releases") {
        EndpointTargetType::Release
    } else if path.contains("/notifications") {
        EndpointTargetType::Notification
    } else if path.contains("/orgs") {
        EndpointTargetType::Organization
    } else if path.contains("/repos") || tags.iter().any(|tag| tag == "repository") {
        EndpointTargetType::Repository
    } else if path.contains("/settings") {
        EndpointTargetType::Settings
    } else if path.contains("/user") || path.contains("/users") {
        EndpointTargetType::User
    } else {
        EndpointTargetType::Unknown
    }
}

fn classify_risk(
    method: &str,
    path: &str,
    operation_id: Option<&str>,
    target_type: EndpointTargetType,
) -> RiskClass {
    let operation_id = operation_id.unwrap_or_default().to_ascii_lowercase();
    let path_lower = path.to_ascii_lowercase();
    if target_type == EndpointTargetType::Admin || path_lower.contains("/sudo") {
        RiskClass::SiteAdmin
    } else if method == "DELETE"
        || operation_id.starts_with("delete")
        || operation_id.contains("delete")
        || operation_id.contains("purge")
    {
        RiskClass::Destructive
    } else if path_lower.contains("token")
        || path_lower.contains("secret")
        || path_lower.contains("/keys")
        || path_lower.contains("/hooks")
        || path_lower.contains("/oauth2")
    {
        RiskClass::Secret
    } else if target_type == EndpointTargetType::ActivityPub && method != "GET" {
        RiskClass::NetworkEgress
    } else if method == "GET" || method == "HEAD" {
        RiskClass::ReadPrivate
    } else if path_lower.contains("/comments") || operation_id.contains("comment") {
        RiskClass::WriteAdditive
    } else {
        RiskClass::WriteMutating
    }
}

fn semantic_operation(method: &str, path: &str) -> Option<&'static str> {
    match (method, path) {
        ("GET", "/repos/{owner}/{repo}") => Some("list_repository_metadata"),
        ("GET", "/repos/{owner}/{repo}/issues") => Some("list_repository_issues"),
        ("POST", "/repos/{owner}/{repo}/issues/{index}/comments") => Some("create_issue_comment"),
        ("GET", "/repos/{owner}/{repo}/pulls") => Some("list_pull_requests"),
        ("GET", "/repos/{owner}/{repo}/pulls/{index}/reviews") => Some("list_pull_request_reviews"),
        ("GET", "/repos/{owner}/{repo}/releases") => Some("list_releases"),
        ("POST", "/repos/{owner}/{repo}/releases") => Some("create_release"),
        ("GET", "/notifications") => Some("list_notifications"),
        ("POST", "/repos/{owner}/{repo}/pulls/{index}/merge") => Some("merge_pull_request"),
        _ => None,
    }
}

fn risk_name(risk: RiskClass) -> &'static str {
    match risk {
        RiskClass::ReadPublic => "read_public",
        RiskClass::ReadPrivate => "read_private",
        RiskClass::WriteAdditive => "write_additive",
        RiskClass::WriteMutating => "write_mutating",
        RiskClass::Destructive => "destructive",
        RiskClass::NetworkEgress => "network_egress",
        RiskClass::Secret => "secret",
        RiskClass::SiteAdmin => "site_admin",
        RiskClass::LongRunning => "long_running",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_requires_repo_read_scope() {
        let registry = OperationRegistry::phase0();
        let mut scopes = BTreeSet::new();
        assert!(!registry.decide("gateway_probe", &scopes).unwrap().allowed);
        scopes.insert("forgejo:repo:read".to_string());
        assert!(registry.decide("gateway_probe", &scopes).unwrap().allowed);
    }

    #[test]
    fn every_current_operation_has_enforced_scope_and_approval_policy() {
        let registry = OperationRegistry::current();
        for operation in registry.operations() {
            let empty_scopes = BTreeSet::new();
            let denied = registry.decide(operation.name, &empty_scopes).unwrap();
            assert!(
                !denied.allowed,
                "{} should deny missing scope",
                operation.name
            );
            assert_eq!(denied.required_scope, operation.scope);

            let mut granted_scopes = BTreeSet::new();
            granted_scopes.insert(operation.scope.to_string());
            let allowed = registry.decide(operation.name, &granted_scopes).unwrap();
            assert!(
                allowed.allowed,
                "{} should allow required scope",
                operation.name
            );
            assert_eq!(allowed.approval_required, operation.approval_required);
        }
    }

    #[test]
    fn pinned_forgejo_api_catalog_classifies_every_operation() {
        let catalog = ForgejoApiCatalog::current().unwrap();
        assert_eq!(catalog.source_version, "15.0.3+gitea-1.22.0");
        assert_eq!(catalog.endpoint_count, 491);
        assert_eq!(catalog.endpoints.len(), catalog.endpoint_count);
        assert!(
            catalog
                .endpoints
                .iter()
                .all(|endpoint| !endpoint.path.is_empty())
        );
    }

    #[test]
    fn unknown_generated_endpoints_are_disabled_until_reviewed() {
        let catalog = ForgejoApiCatalog::current().unwrap();
        let semantic = catalog
            .endpoints
            .iter()
            .filter(|endpoint| endpoint.exposure == EndpointExposure::SemanticOverlay)
            .count();
        assert_eq!(semantic, 9);
        assert!(
            catalog
                .endpoints
                .iter()
                .filter(|endpoint| endpoint.exposure == EndpointExposure::Disabled)
                .all(|endpoint| endpoint.semantic_operation.is_none())
        );
    }

    #[test]
    fn destructive_and_admin_endpoints_require_approval() {
        let catalog = ForgejoApiCatalog::current().unwrap();
        let delete_repo = catalog
            .endpoints
            .iter()
            .find(|endpoint| {
                endpoint.method == "DELETE" && endpoint.path == "/repos/{owner}/{repo}"
            })
            .unwrap();
        assert_eq!(delete_repo.risk, RiskClass::Destructive);
        assert!(delete_repo.approval_required);
        assert_eq!(delete_repo.exposure, EndpointExposure::Disabled);

        assert!(
            catalog
                .endpoints
                .iter()
                .filter(|endpoint| endpoint.target_type == EndpointTargetType::Admin)
                .all(|endpoint| endpoint.approval_required
                    && endpoint.exposure == EndpointExposure::Disabled)
        );
    }

    #[test]
    fn semantic_overlay_matches_current_registry_operations() {
        let catalog = ForgejoApiCatalog::current().unwrap();
        let registry = OperationRegistry::current();
        for endpoint in catalog
            .endpoints
            .iter()
            .filter(|endpoint| endpoint.exposure == EndpointExposure::SemanticOverlay)
        {
            let operation = endpoint.semantic_operation.as_deref().unwrap();
            assert!(
                registry.operation(operation).is_ok(),
                "{operation} must be a registered policy operation"
            );
        }
    }
}
