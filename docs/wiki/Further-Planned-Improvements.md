# Further Planned Improvements

This page tracks completed hardening work and remaining improvements after the
stable `1.0` line. Items marked complete are implemented or documented in the
repository as of `1.1.0`.

## Open Source Publication

| Improvement | Status | Notes |
| --- | --- | --- |
| Add complete REUSE metadata with `SPDX-FileCopyrightText` entries for maintained source files. | Remaining | SPDX license identifiers exist in Rust source and Cargo manifests, but full REUSE copyright metadata is not complete. |
| Decide whether documentation should remain under `AGPL-3.0-or-later` with the code or move to a documentation-specific copyleft license. | Complete | The repository uses `AGPL-3.0-or-later` consistently for code and docs. |
| Add signed release artifacts and document how users can verify them. | Remaining | Tags and Codeberg releases exist, but signed binary/source artifacts are not yet documented. |
| Publish a software bill of materials for release builds. | Started | `.forgejo/workflows/ci.yml` and `docs/security-checks.md` include CycloneDX SBOM generation. Hosted release SBOM artifacts still need to be attached during release. |
| Add an OpenSSF Best Practices badge or equivalent public security checklist once the project is hosted publicly. | Started | The checklist is started in `docs/promotion/openssf-best-practices-checklist.md`; the public badge should wait until the OpenSSF project entry exists. |

## Supply Chain Security

| Improvement | Status | Notes |
| --- | --- | --- |
| Add `cargo audit` or an equivalent dependency advisory check to CI. | Complete | `.forgejo/workflows/ci.yml` installs `cargo-audit` and runs `cargo audit`. |
| Add dependency policy checks for duplicate crates, banned dependencies, and unexpected licenses. | Complete | `deny.toml` exists and CI runs `cargo deny check`. |
| Add automated dependency update review with tests before merge. | Remaining | No dependency-update automation is configured yet. |
| Protect the public default branch with required review and status checks. | Remaining | This is a hosted Forgejo/Codeberg repository setting, not a repo-file change. |

## Secret And Token Safety

| Improvement | Status | Notes |
| --- | --- | --- |
| Add a documented incident process for leaked Keycloak or Forgejo credentials. | Started | `SECURITY.md` documents secret handling and private reporting, but rotation-specific incident steps still need more detail. |
| Add secret scanning to CI for pull requests. | Remaining | Local secret scans have been run, but no CI secret-scanning job is configured. |
| Keep Keycloak client secrets, Forgejo tokens, and bearer tokens in runtime secret stores only. | Complete | Configuration docs require runtime environment variables or secret managers; principal maps store token environment variable names, not token values. |
| Add examples for rotating Keycloak client credentials and Forgejo service credentials. | Remaining | Rotation examples are not yet documented. |

## MCP And Forgejo Capability Roadmap

| Improvement | Status | Notes |
| --- | --- | --- |
| Add read-only Forgejo repository metadata tools after policy and identity checks. | Complete | `list_repository_metadata` is shipped with principal mapping and Forgejo ACL enforcement. |
| Add issue and pull-request tools with explicit operation classes and audit records. | Complete | Bounded issue, pull-request, review, release, notification, and comment tools are shipped and documented. |
| Add write operations only after approval flow, idempotency, and rollback behavior are specified. | Complete for current high-risk writes | `create_pull_request`, `merge_pull_request`, and `create_release` require exact-payload, single-use approval records and support dry-run preview. New write classes still need separate review before exposure. |
| Add branch-to-PR bootstrap so agents can create a PR after pushing a branch. | Complete | `create_pull_request` accepts `owner/repo`, `head`, `base`, `title`, optional body, assignee, assignees, and reviewers. Reviewer requests are reported separately after PR creation. |
| Add operation discovery so agents do not need source inspection to know supported tools. | Complete | `GET /capabilities` lists operation names, scopes, risk classes, approval requirements, and planned disabled operations. |
| Add full PR workflow readbacks for branch status and checks. | Remaining | `update_pull_request`, standalone `request_reviewers`, `get_branch_status`, `get_required_checks`, and `get_pr_checks` remain planned disabled operations. |
| Add policy controls for destructive operations such as delete, force push, and repository transfer. | Complete as deny-by-default classification | Destructive and admin operations are classified and approval-required, but executable destructive tools remain intentionally disabled. |
| Add per-agent rate limiting and replay protection for sensitive MCP calls. | Partly complete | Approval replay protection is implemented. Per-agent rate limiting is still remaining. |

## Identity And Audit

| Improvement | Status | Notes |
| --- | --- | --- |
| Add configurable Keycloak role and group mapping to operation classes. | Remaining | Authorization currently uses token scopes and explicit principal mapping. |
| Add JWKS refresh and key-rotation behavior with clear cache limits. | Started | OIDC discovery and JWKS validation are implemented; documented cache/rotation policy still needs hardening. |
| Export structured audit records to a durable sink such as OpenTelemetry, syslog, or append-only storage. | Started | Structured token-free audit events are emitted through logging. Dedicated OpenTelemetry/syslog/export sinks are not implemented. |
| Add a public threat model for identity, authorization, secret handling, audit integrity, and Forgejo delegation. | Remaining | `docs/wiki/Security-Model.md` exists, but a full public threat model is still needed. |
