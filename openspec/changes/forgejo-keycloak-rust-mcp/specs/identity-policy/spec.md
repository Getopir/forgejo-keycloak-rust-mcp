## ADDED Requirements

### Requirement: Keycloak Resource-Bound Authentication

The gateway SHALL validate Keycloak access tokens for the configured MCP resource audience before executing any MCP operation.

#### Scenario: Missing token

- **WHEN** a request reaches `/mcp` without an `Authorization: Bearer` token
- **THEN** the gateway SHALL return `401`
- **AND** the response SHALL include a `WWW-Authenticate` challenge pointing to protected-resource metadata.

#### Scenario: Wrong audience

- **WHEN** a Keycloak token is valid for another audience
- **THEN** the gateway SHALL reject it before operation policy or Forgejo delegation.

### Requirement: Immutable Principal Mapping

The gateway SHALL map `(issuer, subject)` to a Forgejo account and SHALL NOT accept a caller-supplied Forgejo username as authority.

#### Scenario: User argument attempts impersonation

- **WHEN** a tool argument includes a Forgejo username that differs from the mapped principal
- **THEN** the gateway SHALL ignore it for identity selection or reject the operation.

#### Scenario: Unknown mapped principal

- **WHEN** a valid token calls a Forgejo-backed operation and no `(issuer, subject)` mapping exists
- **THEN** the gateway SHALL reject the request before any Forgejo call.

#### Scenario: Disabled mapped principal

- **WHEN** a valid token maps to a disabled principal entry
- **THEN** the gateway SHALL reject the request before any Forgejo call.

### Requirement: Operation Policy Registry

Every exposed operation SHALL have a deterministic policy entry containing scope, risk class, approval requirement, and response limits.

#### Scenario: Missing scope

- **WHEN** the token lacks the operation's required scope
- **THEN** the gateway SHALL deny the operation before any Forgejo call.

### Requirement: Read-Only Repository Metadata

The gateway SHALL provide a read-only repository metadata operation that uses the mapped Forgejo principal and returns bounded repository fields.

#### Scenario: Missing target

- **WHEN** `list_repository_metadata` is called without an `owner/repository` target
- **THEN** the gateway SHALL return `400`
- **AND** SHALL NOT call Forgejo.

#### Scenario: Mapped read-only access

- **WHEN** a mapped principal calls `list_repository_metadata` with `forgejo:repo:read`
- **THEN** the gateway SHALL call Forgejo using the mapped principal's configured token environment variable
- **AND** SHALL return bounded repository metadata.

#### Scenario: Mutating operation attempt

- **WHEN** a caller requests issue, pull request, release, admin, or destructive operations in Phase 1
- **THEN** the gateway SHALL either return policy-only metadata or deny the operation
- **AND** SHALL NOT execute a Forgejo mutation.
