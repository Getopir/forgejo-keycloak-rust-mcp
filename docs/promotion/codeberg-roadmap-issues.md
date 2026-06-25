# Codeberg Roadmap Issues

Open these as public Codeberg issues.

## Phase 1 hardening: principal mapping

Title: `Phase 1 hardening: principal mapping`

Body:

```markdown
Harden immutable Keycloak-to-Forgejo principal mapping after the `0.5.0` baseline.

Scope:

- Add mapping management commands or an operator workflow.
- Add mapping validation for duplicate `(issuer, subject)` entries.
- Add stricter validation for `api_token_env` names.
- Ensure caller-supplied usernames cannot impersonate another Forgejo user.
- Add audit event coverage for mapping file reloads or operator changes.

Acceptance:

- Tests cover duplicate mappings and invalid token environment names.
- Documentation explains the operator lifecycle for mapping creation, disablement, and rotation.
```

## Phase 1 hardening: trusted-header delegation

Title: `Phase 1 hardening: trusted-header delegation`

Body:

```markdown
Harden and document the trusted-header delegation boundary for Forgejo after the `0.5.0` header derivation baseline.

Scope:

- Document Forgejo reverse-proxy authentication settings.
- Add an integration deployment example where only the gateway or reverse proxy can reach the trusted Forgejo path.
- Add explicit tests or checks for incoming spoofed trusted headers.
- Keep Forgejo as the final repository/organization ACL authority.
- Document that Forgejo reverse-proxy auth does not apply to API requests, so API-backed tools need a safe credential strategy.

Acceptance:

- Configuration docs name required Forgejo settings and trusted proxy limits.
- Tests prove caller-supplied identity headers do not influence mapped identity or generated delegation values.
- Audit records include delegated Forgejo login when delegation is used.
```

## Phase 1 hardening: read-only repository metadata tool

Title: `Phase 1 hardening: read-only repository metadata tool`

Body:

```markdown
Harden the first real Forgejo-backed MCP tool: read-only repository metadata.

Scope:

- Add live integration tests against a disposable Forgejo repository.
- Add configurable response field selection if needed.
- Return repository full name, visibility, archived state, default branch, description, safe clone URLs, update timestamp, issue/PR counts when available, and mapped-principal permissions.
- Do not return secrets, deploy keys, webhooks, runners, admin settings, or private environment values.

Acceptance:

- Mapped read-only agent can fetch metadata for a repository it can read.
- Same agent is denied for a repository it cannot read.
- Missing scope is denied before any Forgejo call.
- Response is bounded and schema-stable.
```

## Security: threat model

Title: `Security: threat model`

Body:

```markdown
Write a public threat model for the gateway.

Cover:

- Keycloak token validation.
- Principal mapping.
- Forgejo delegation boundary.
- Header spoofing.
- Token and secret handling.
- Audit integrity.
- Approval gates for future mutating actions.
- Admin and destructive-operation separation.

Acceptance:

- Threat model is linked from `SECURITY.md` and the wiki.
- Each risk has an explicit mitigation or a documented future-control issue.
```

## CI: cargo audit / deny / SBOM

Title: `CI: cargo audit / deny / SBOM`

Body:

```markdown
Add supply-chain checks for public releases.

Scope:

- Add dependency vulnerability scanning.
- Add license/dependency policy checks.
- Add an SBOM generation path for release builds.
- Document how maintainers run the checks locally.

Acceptance:

- CI fails on known vulnerable dependencies unless explicitly waived.
- CI reports dependency/license policy violations.
- Release checklist includes SBOM generation.
```
