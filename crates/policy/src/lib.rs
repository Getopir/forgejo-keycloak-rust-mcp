// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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
}
