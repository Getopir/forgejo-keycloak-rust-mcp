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

### Requirement: Operation Policy Registry

Every exposed operation SHALL have a deterministic policy entry containing scope, risk class, approval requirement, and response limits.

#### Scenario: Missing scope

- **WHEN** the token lacks the operation's required scope
- **THEN** the gateway SHALL deny the operation before any Forgejo call.
