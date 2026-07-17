// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::McpProbeRequest;
use crate::principal::PrincipalMapping;
use identity::Principal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const DEFAULT_APPROVAL_TTL_SECONDS: u64 = 900;

#[derive(Debug, Clone)]
pub struct ApprovalStore {
    path: PathBuf,
    lock_path: PathBuf,
    ttl_seconds: u64,
    consume_lock: Arc<Mutex<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Approved,
    Consumed,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub approval_id: Uuid,
    pub operation: String,
    pub target: Option<String>,
    pub state: Option<String>,
    pub body_sha256: String,
    pub issuer: String,
    pub subject: String,
    pub oauth_client: Option<String>,
    pub forgejo_login: String,
    pub forgejo_user_id: Option<i64>,
    pub created_at_epoch: u64,
    pub expires_at_epoch: u64,
    pub status: ApprovalStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed_by_issuer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed_by_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed_by_oauth_client: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed_by_forgejo_login: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed_at_epoch: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApprovalGrant {
    pub approval_id: Uuid,
    pub operation: String,
    pub target: Option<String>,
    pub expires_at_epoch: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApprovalValidation {
    pub approval_id: Uuid,
    pub expires_at_epoch: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum ApprovalError {
    #[error("approval store is not configured")]
    NotConfigured,
    #[error("approval_id is required before this operation can execute")]
    MissingId,
    #[error("approval_id is not a valid UUID")]
    InvalidId,
    #[error("approval record was not found")]
    NotFound,
    #[error("approval record has been revoked")]
    Revoked,
    #[error("approval record has already been consumed")]
    AlreadyConsumed,
    #[error("approval record has expired")]
    Expired,
    #[error("approval record does not match this operation")]
    OperationMismatch,
    #[error("approval record does not match this target")]
    TargetMismatch,
    #[error("approval record does not match this request state")]
    StateMismatch,
    #[error("approval record does not match this request body")]
    BodyMismatch,
    #[error("approval record does not match this principal")]
    PrincipalMismatch,
    #[error("approval approver and executor must be different mapped principals")]
    ApproverExecutorSame,
    #[error("approval store I/O failed: {0}")]
    Io(String),
    #[error("approval record is invalid JSON: {0}")]
    Json(String),
}

#[derive(Serialize)]
struct BodyFingerprint<'a> {
    body: Option<&'a str>,
}

impl ApprovalStore {
    pub fn new(path: PathBuf, ttl_seconds: u64) -> Self {
        let mut lock_path = path.as_os_str().to_os_string();
        lock_path.push(".lock");
        Self {
            path,
            lock_path: PathBuf::from(lock_path),
            ttl_seconds: ttl_seconds.max(1),
            consume_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn default_ttl_seconds() -> u64 {
        DEFAULT_APPROVAL_TTL_SECONDS
    }

    pub fn create(
        &self,
        operation: &str,
        request: &McpProbeRequest,
        principal: &Principal,
        mapping: &PrincipalMapping,
    ) -> Result<ApprovalGrant, ApprovalError> {
        let now = now_epoch();
        let record = ApprovalRecord {
            approval_id: Uuid::now_v7(),
            operation: operation.to_string(),
            target: request.target.clone(),
            state: request.state.clone(),
            body_sha256: request_body_hash(request),
            issuer: principal.issuer.clone(),
            subject: principal.subject.clone(),
            oauth_client: principal.oauth_client.clone(),
            forgejo_login: mapping.forgejo_login.clone(),
            forgejo_user_id: mapping.forgejo_user_id,
            created_at_epoch: now,
            expires_at_epoch: now.saturating_add(self.ttl_seconds),
            status: ApprovalStatus::Approved,
            consumed_by_issuer: None,
            consumed_by_subject: None,
            consumed_by_oauth_client: None,
            consumed_by_forgejo_login: None,
            consumed_at_epoch: None,
        };
        self.append(&record)?;
        Ok(ApprovalGrant {
            approval_id: record.approval_id,
            operation: record.operation,
            target: record.target,
            expires_at_epoch: record.expires_at_epoch,
        })
    }

    pub fn validate(
        &self,
        request: &McpProbeRequest,
        principal: &Principal,
        mapping: &PrincipalMapping,
    ) -> Result<ApprovalValidation, ApprovalError> {
        let record = self.validated_record(request, principal, mapping)?;
        Ok(ApprovalValidation {
            approval_id: record.approval_id,
            expires_at_epoch: record.expires_at_epoch,
        })
    }

    pub fn consume(
        &self,
        request: &McpProbeRequest,
        principal: &Principal,
        mapping: &PrincipalMapping,
    ) -> Result<ApprovalValidation, ApprovalError> {
        let _process_guard = self
            .consume_lock
            .lock()
            .map_err(|_| ApprovalError::Io("approval consume lock is poisoned".to_string()))?;
        let _file_guard = self.lock_consumption()?;
        let mut record = self.validated_record(request, principal, mapping)?;
        record.status = ApprovalStatus::Consumed;
        record.consumed_by_issuer = Some(principal.issuer.clone());
        record.consumed_by_subject = Some(principal.subject.clone());
        record.consumed_by_oauth_client = principal.oauth_client.clone();
        record.consumed_by_forgejo_login = Some(mapping.forgejo_login.clone());
        record.consumed_at_epoch = Some(now_epoch());
        self.append(&record)?;
        Ok(ApprovalValidation {
            approval_id: record.approval_id,
            expires_at_epoch: record.expires_at_epoch,
        })
    }

    fn lock_consumption(&self) -> Result<File, ApprovalError> {
        if let Some(parent) = self.lock_path.parent() {
            std::fs::create_dir_all(parent).map_err(|err| ApprovalError::Io(err.to_string()))?;
        }
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&self.lock_path)
            .map_err(|err| ApprovalError::Io(err.to_string()))?;
        file.lock()
            .map_err(|err| ApprovalError::Io(err.to_string()))?;
        Ok(file)
    }

    fn validated_record(
        &self,
        request: &McpProbeRequest,
        principal: &Principal,
        mapping: &PrincipalMapping,
    ) -> Result<ApprovalRecord, ApprovalError> {
        let approval_id = request
            .approval_id
            .as_deref()
            .ok_or(ApprovalError::MissingId)?;
        let approval_id = Uuid::parse_str(approval_id).map_err(|_| ApprovalError::InvalidId)?;
        let record = self
            .find_latest(approval_id)?
            .ok_or(ApprovalError::NotFound)?;
        if record.status == ApprovalStatus::Revoked {
            return Err(ApprovalError::Revoked);
        }
        if record.status == ApprovalStatus::Consumed {
            return Err(ApprovalError::AlreadyConsumed);
        }
        if now_epoch() > record.expires_at_epoch {
            return Err(ApprovalError::Expired);
        }
        if record.operation != request.operation {
            return Err(ApprovalError::OperationMismatch);
        }
        if record.target != request.target {
            return Err(ApprovalError::TargetMismatch);
        }
        if record.state != request.state {
            return Err(ApprovalError::StateMismatch);
        }
        if record.body_sha256 != request_body_hash(request) {
            return Err(ApprovalError::BodyMismatch);
        }
        if (record.issuer == principal.issuer && record.subject == principal.subject)
            || record.forgejo_login == mapping.forgejo_login
        {
            return Err(ApprovalError::ApproverExecutorSame);
        }
        Ok(record)
    }

    fn append(&self, record: &ApprovalRecord) -> Result<(), ApprovalError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|err| ApprovalError::Io(err.to_string()))?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| ApprovalError::Io(err.to_string()))?;
        let line =
            serde_json::to_string(record).map_err(|err| ApprovalError::Json(err.to_string()))?;
        writeln!(file, "{line}").map_err(|err| ApprovalError::Io(err.to_string()))
    }

    fn find_latest(&self, approval_id: Uuid) -> Result<Option<ApprovalRecord>, ApprovalError> {
        if !Path::new(&self.path).exists() {
            return Ok(None);
        }
        let file = File::open(&self.path).map_err(|err| ApprovalError::Io(err.to_string()))?;
        let reader = BufReader::new(file);
        let mut records = BTreeMap::new();
        for line in reader.lines() {
            let line = line.map_err(|err| ApprovalError::Io(err.to_string()))?;
            if line.trim().is_empty() {
                continue;
            }
            let record: ApprovalRecord =
                serde_json::from_str(&line).map_err(|err| ApprovalError::Json(err.to_string()))?;
            records.insert(record.approval_id, record);
        }
        Ok(records.remove(&approval_id))
    }
}

pub fn request_body_hash(request: &McpProbeRequest) -> String {
    let value = serde_json::to_vec(&BodyFingerprint {
        body: request.body.as_deref(),
    })
    .expect("body fingerprint serialization should not fail");
    let digest = Sha256::digest(value);
    hex_lower(&digest)
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::principal::{PrincipalKind, PrincipalMapping};
    use std::collections::BTreeSet;
    use std::sync::{Arc, Barrier};

    fn request(operation: &str) -> McpProbeRequest {
        McpProbeRequest {
            operation: operation.to_string(),
            requested_operation: None,
            target: Some("rawholding/example#7".to_string()),
            query: None,
            limit: None,
            cursor: None,
            state: None,
            body: Some("ship it".to_string()),
            approval_id: None,
            dry_run: false,
        }
    }

    fn principal(subject: &str) -> Principal {
        Principal {
            issuer: "https://sso.example/realms/main".to_string(),
            subject: subject.to_string(),
            oauth_client: Some("agent".to_string()),
            scopes: BTreeSet::new(),
            preferred_username: Some("agent".to_string()),
        }
    }

    fn mapping(login: &str) -> PrincipalMapping {
        PrincipalMapping {
            issuer: "https://sso.example/realms/main".to_string(),
            subject: "subject-1".to_string(),
            forgejo_login: login.to_string(),
            forgejo_user_id: Some(42),
            forgejo_email: None,
            forgejo_full_name: None,
            enabled: true,
            principal_type: PrincipalKind::Agent,
            api_token_env: Some("FORGEJO_TOKEN".to_string()),
        }
    }

    #[test]
    fn approval_is_bound_to_exact_body() {
        let dir = std::env::temp_dir().join(format!("approval-{}.jsonl", Uuid::now_v7()));
        let store = ApprovalStore::new(dir, 60);
        let approver = principal("approver-subject");
        let approver_mapping = mapping("approver-user");
        let executor = principal("executor-subject");
        let executor_mapping = mapping("executor-user");
        let mut request = request("merge_pull_request");
        let grant = store
            .create("merge_pull_request", &request, &approver, &approver_mapping)
            .unwrap();
        request.approval_id = Some(grant.approval_id.to_string());
        assert!(
            store
                .validate(&request, &executor, &executor_mapping)
                .is_ok()
        );

        request.body = Some("different".to_string());
        assert!(matches!(
            store.validate(&request, &executor, &executor_mapping),
            Err(ApprovalError::BodyMismatch)
        ));
    }

    #[test]
    fn approval_rejects_principal_mismatch() {
        let dir = std::env::temp_dir().join(format!("approval-{}.jsonl", Uuid::now_v7()));
        let store = ApprovalStore::new(dir, 60);
        let mut request = request("delete_repository");
        let grant = store
            .create(
                "delete_repository",
                &request,
                &principal("subject-1"),
                &mapping("agent-user"),
            )
            .unwrap();
        request.approval_id = Some(grant.approval_id.to_string());
        assert!(matches!(
            store.validate(&request, &principal("subject-1"), &mapping("agent-user")),
            Err(ApprovalError::ApproverExecutorSame)
        ));
    }

    #[test]
    fn consumed_approval_cannot_be_replayed() {
        let dir = std::env::temp_dir().join(format!("approval-{}.jsonl", Uuid::now_v7()));
        let store = ApprovalStore::new(dir, 60);
        let mut request = request("merge_pull_request");
        let grant = store
            .create(
                "merge_pull_request",
                &request,
                &principal("approver"),
                &mapping("approver-user"),
            )
            .unwrap();
        request.approval_id = Some(grant.approval_id.to_string());
        assert!(
            store
                .consume(&request, &principal("executor"), &mapping("executor-user"))
                .is_ok()
        );
        assert!(matches!(
            store.validate(&request, &principal("executor"), &mapping("executor-user")),
            Err(ApprovalError::AlreadyConsumed)
        ));
    }

    #[test]
    fn concurrent_consumers_allow_exactly_one_execution() {
        let path = std::env::temp_dir().join(format!("approval-{}.jsonl", Uuid::now_v7()));
        let first_store = ApprovalStore::new(path.clone(), 60);
        let second_store = ApprovalStore::new(path, 60);
        let approved_request = request("merge_pull_request");
        let grant = first_store
            .create(
                "merge_pull_request",
                &approved_request,
                &principal("approver"),
                &mapping("approver-user"),
            )
            .unwrap();
        let barrier = Arc::new(Barrier::new(2));
        let run_consumer = |store: ApprovalStore, barrier: Arc<Barrier>| {
            let approval_id = grant.approval_id.to_string();
            std::thread::spawn(move || {
                let mut request = request("merge_pull_request");
                request.approval_id = Some(approval_id);
                barrier.wait();
                store.consume(&request, &principal("executor"), &mapping("executor-user"))
            })
        };
        let first = run_consumer(first_store, Arc::clone(&barrier));
        let second = run_consumer(second_store, barrier);
        let results = [first.join().unwrap(), second.join().unwrap()];

        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        assert_eq!(
            results
                .iter()
                .filter(|result| matches!(result, Err(ApprovalError::AlreadyConsumed)))
                .count(),
            1
        );
    }
}
