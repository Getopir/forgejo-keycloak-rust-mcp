# Remaining Maintainer Backlog

This is the short, current backlog after `1.2.4`, not a restatement of the
completed phase roadmap. Entries below are deliberate follow-up work; there is
no commitment to enable every Forgejo API endpoint.

## Release And Supply Chain

| Improvement | Status |
| --- | --- |
| Complete REUSE copyright metadata for maintained source files. | Complete in `1.2.2` via `REUSE.toml` and `LICENSES/` |
| Produce signed release artifacts and document verification. | Complete in `1.2.3`; see [Release Artifact Verification](Release-Artifact-Verification.md) |
| Attach the CI-generated CycloneDX SBOM to hosted releases. | Complete in `1.2.4`; CI validates SBOM generation and the release workflow attaches the generated documents to Forgejo releases |
| Finish the OpenSSF project entry/badge after public-hosting prerequisites are met. | Remaining |
| Add automated dependency-update review with tests. | Complete in `1.2.4`; scheduled Renovate PRs run the normal pull-request CI suite |
| Protect public default branches with required review and checks. | Release implementation ready in `1.2.4`; hosted Codeberg setting requires an organization-admin credential and verified check context |

## Credential And Operational Hardening

| Improvement | Status |
| --- | --- |
| Document credential-rotation and incident-response procedures. | Complete in `1.2.2`; see [Credential Rotation And Incident Response](Credential-Rotation-And-Incident-Response.md) |
| Add pull-request secret scanning in CI. | Complete on `main`; checksum-verified Gitleaks scans full Git history before Rust CI checks |
| Document JWKS cache limits and key-rotation behaviour. | Complete on `main`; see [JWKS Cache Limits And Key Rotation](JWKS-Cache-Limits-And-Key-Rotation.md) |
| Export structured audit records to a durable sink. | Complete on `main`; `FORGEJO_MCPD_AUDIT_LOG` enables append-only, synchronized JSONL export |
| Publish a full threat model linked from the security documentation. | Remaining |
| Add per-agent rate limiting. Approval replay prevention is already shipped. | Remaining |

## Carefully Scoped Capability Work

| Improvement | Status |
| --- | --- |
| Refresh the pinned Forgejo OpenAPI document from `15.0.3` to `16.0.0` and regenerate the reviewed coverage catalog. | Remaining |
| Add a bounded `get_branch_status` read operation. | Remaining |
| Add a bounded `get_required_checks` read operation. | Remaining |
| Add a bounded `get_pr_checks` read operation. | Remaining |
| Add an approval-gated `update_pull_request` operation. | Remaining |
| Add an approval-gated standalone `request_reviewers` operation. | Remaining |
| Add a bounded `list_action_runs` read operation. | Remaining |
| Add a bounded `list_action_run_jobs` read operation. | Remaining |
| Add a bounded `get_action_job_logs` read operation with output limits. | Remaining |
| Add a bounded `get_action_run_logs` read operation with output limits. | Remaining |
| Add a bounded `list_action_artifacts` read operation. | Remaining |
| Add a bounded `download_action_artifact` operation with size limits. | Remaining |
| Add an approval-gated `cancel_action_run` operation. | Remaining |
| Add a bounded `get_repository_branch` read operation. | Remaining |
| Add a bounded `list_repository_commits` read operation. | Remaining |
| Add a bounded `get_repository_file` read operation. | Remaining |
| Add an approval-gated `upload_release_asset` operation with size and content-type limits. | Remaining |
| Add an approval-gated `update_issue` operation. | Remaining |
| Role/group mapping to operation classes. | Remaining |
| Any further generated Forgejo endpoint. | Requires a named semantic operation, schema, scope, bounded output, audit, and approval policy where applicable |
| Destructive or instance-admin execution. | Intentionally disabled |

Completed work and the original phase plan are documented in the
[Documentation Archive](Documentation-Archive.md) and release notes.
