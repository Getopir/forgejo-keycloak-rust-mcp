# Codeberg Roadmap Issues

Open these as public Codeberg issues.

## Phase 1: principal mapping

Title: `Phase 1: principal mapping`

Body:

```markdown
Implement immutable Keycloak-to-Forgejo principal mapping.

Scope:

- Map `(issuer, subject)` from a validated Keycloak token to a Forgejo account.
- Treat `sub` as the stable identity key.
- Store/display Forgejo username and optional Forgejo user ID.
- Reject unknown or disabled mappings by default.
- Ensure caller-supplied usernames cannot impersonate another Forgejo user.
- Include both Keycloak and Forgejo principal information in audit events.

Acceptance:

- Valid mapped principal is accepted for Phase 1 read-only calls.
- Unknown principal is denied before any Forgejo call.
- Disabled mapping is denied before any Forgejo call.
- Tests cover unknown, disabled, and mapped principals.
```

## Phase 1: trusted-header delegation

Title: `Phase 1: trusted-header delegation`

Body:

```markdown
Implement and document the trusted-header delegation boundary for Forgejo.

Scope:

- Document Forgejo reverse-proxy authentication settings.
- Ensure trusted identity headers are only emitted by the MCP gateway or trusted proxy path.
- Strip or ignore caller-supplied identity headers.
- Keep Forgejo as the final repository/organization ACL authority.
- Document that Forgejo reverse-proxy auth does not apply to API requests, so API-backed tools need a safe credential strategy.

Acceptance:

- Configuration docs name required Forgejo settings and trusted proxy limits.
- Tests prove caller-supplied identity headers do not influence mapped identity.
- Audit records include delegated Forgejo login when delegation is used.
```

## Phase 1: read-only repository metadata tool

Title: `Phase 1: read-only repository metadata tool`

Body:

```markdown
Implement the first real Forgejo-backed MCP tool: read-only repository metadata.

Scope:

- Add a bounded `list_repository_metadata` implementation.
- Require `forgejo:repo:read`.
- Require a mapped Forgejo principal.
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
