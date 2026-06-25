// SPDX-License-Identifier: MIT OR Apache-2.0

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
    pub fn phase0() -> Self {
        // Phase 0 makes the policy boundary explicit before Forgejo API
        // execution is enabled: every operation has a required OAuth scope,
        // risk class, and approval flag that clients can test deterministically.
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
                name: "create_issue_comment",
                scope: "forgejo:issue:write",
                risk: RiskClass::WriteAdditive,
                approval_required: false,
                description: "Additive issue comment creation.",
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
    fn every_phase0_operation_has_enforced_scope_and_approval_policy() {
        let registry = OperationRegistry::phase0();
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
