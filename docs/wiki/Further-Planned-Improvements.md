# Further Planned Improvements

This page tracks improvements that should be considered before later beta releases and before a stable `1.0` release.

## Open Source Publication

- Add complete REUSE metadata with `SPDX-FileCopyrightText` entries for maintained source files.
- Decide whether documentation should remain under `AGPL-3.0-or-later` with the code or move to a documentation-specific copyleft license.
- Add signed release artifacts and document how users can verify them.
- Publish a software bill of materials for release builds.
- Add an OpenSSF Best Practices badge or equivalent public security checklist once the project is hosted publicly.

## Supply Chain Security

- Add `cargo audit` or an equivalent dependency advisory check to CI.
- Add dependency policy checks for duplicate crates, banned dependencies, and unexpected licenses.
- Add automated dependency update review with tests before merge.
- Protect the public default branch with required review and status checks.

## Secret And Token Safety

- Add a documented incident process for leaked Keycloak or Forgejo credentials.
- Add secret scanning to CI for pull requests.
- Keep Keycloak client secrets, Forgejo tokens, and bearer tokens in runtime secret stores only.
- Add examples for rotating Keycloak client credentials and Forgejo service credentials.

## MCP And Forgejo Capability Roadmap

- Add read-only Forgejo repository metadata tools after policy and identity checks.
- Add issue and pull-request tools with explicit operation classes and audit records.
- Add write operations only after approval flow, idempotency, and rollback behavior are specified.
- Add policy controls for destructive operations such as delete, force push, and repository transfer.
- Add per-agent rate limiting and replay protection for sensitive MCP calls.

## Identity And Audit

- Add configurable Keycloak role and group mapping to operation classes.
- Add JWKS refresh and key-rotation behavior with clear cache limits.
- Export structured audit records to a durable sink such as OpenTelemetry, syslog, or append-only storage.
- Add a public threat model for identity, authorization, secret handling, audit integrity, and Forgejo delegation.
