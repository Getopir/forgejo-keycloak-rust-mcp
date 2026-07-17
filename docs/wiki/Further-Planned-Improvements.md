# Remaining Maintainer Backlog

This is the short, current backlog after `1.2.7`, not a restatement of the
completed phase roadmap. Entries below are deliberate follow-up work; there is
no commitment to enable every Forgejo API endpoint.

## Release And Supply Chain

| Improvement | Status |
| --- | --- |
| Complete REUSE copyright metadata for maintained source files. | Complete in `1.2.2` via `REUSE.toml` and `LICENSES/` |
| Produce signed release artifacts and document verification. | Complete in `1.2.3`; see [Release Artifact Verification](Release-Artifact-Verification.md) |
| Attach the CI-generated CycloneDX SBOM to hosted releases. | Complete in `1.2.4`; CI validates SBOM generation and the release workflow attaches the generated documents to Forgejo releases |
| Finish the OpenSSF project entry/badge after public-hosting prerequisites are met. | Public-hosting evidence and `.bestpractices.json` proposal included in `1.2.7`; external entry creation and self-certification require a maintainer-authenticated `bestpractices.dev` session before a real badge ID can be published |
| Add automated dependency-update review with tests. | Complete in `1.2.4`; scheduled Renovate PRs run the normal pull-request CI suite |
| Protect public default branches with required review and checks. | Release implementation ready in `1.2.4`; hosted Codeberg setting requires an organization-admin credential and verified check context |

## Credential And Operational Hardening

| Improvement | Status |
| --- | --- |
| Document credential-rotation and incident-response procedures. | Complete in `1.2.2`; see [Credential Rotation And Incident Response](Credential-Rotation-And-Incident-Response.md) |
| Add pull-request secret scanning in CI. | Complete in `1.2.6`; checksum-verified Gitleaks scans full Git history before Rust CI checks |
| Document JWKS cache limits and key-rotation behaviour. | Complete in `1.2.6`; see [JWKS Cache Limits And Key Rotation](JWKS-Cache-Limits-And-Key-Rotation.md) |
| Export structured audit records to a durable sink. | Complete in `1.2.6`; `FORGEJO_MCPD_AUDIT_LOG` enables append-only, synchronized JSONL export |
| Publish a full threat model linked from the security documentation. | Complete in `1.2.6`; see [Threat Model](Threat-Model.md) |
| Add per-agent rate limiting. Approval replay prevention is already shipped. | Complete in `1.2.7`; bounded per-agent token buckets return `429` with retry guidance and emit denied audit records |
| Publish a monitored vulnerability-reporting process using `info@getopir.com`. | Remaining; update `SECURITY.md` with the exact contact route, required report details, no-public-issue guidance, and a verified acknowledgement commitment before submitting the OpenSSF application |
| Document how vulnerability reports sent to `info@getopir.com` remain private. | Remaining; define restricted handling, redacted initial reports, an agreed secure channel for sensitive follow-up, retention expectations, and escalation ownership before claiming the OpenSSF private-reporting criterion |
| Add an administrator-only MCP settings surface linked from Forgejo, backed by validated non-secret configuration with effective-value readback, audit history, controlled reload, and rollback. | Remaining; the installer must not expose secrets or grant Forgejo permission to rewrite arbitrary service environment files |

## Quality And Assurance

| Improvement | Status |
| --- | --- |
| Demonstrate that automated tests cover most code branches, input fields, and functionality. | Remaining; add reproducible statement and branch coverage measurement to CI, publish the results, define documented thresholds, and close material gaps before claiming the OpenSSF `test_most` criterion |
| Enforce maximally strict Rust warnings in CI. | Remaining; require `cargo clippy --workspace --all-targets -- -D warnings` as a mandatory CI check before claiming the OpenSSF `warnings_strict` criterion |

## Carefully Scoped Capability Work

| Improvement | Status |
| --- | --- |
| Refresh the pinned Forgejo OpenAPI document from `15.0.3` to `16.0.0` and regenerate the reviewed coverage catalog. | Complete; 506 operations classified, with all 15 additions kept disabled pending separate semantic review |
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
