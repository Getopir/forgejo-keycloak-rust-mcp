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

#### Scenario: Duplicate mapping

- **WHEN** the principal map contains duplicate `(issuer, subject)` entries after issuer normalization
- **THEN** gateway startup SHALL fail
- **AND** no ambiguous Forgejo identity SHALL be selected.

#### Scenario: Malformed token environment name

- **WHEN** a principal map entry uses an `api_token_env` value outside ASCII letters, digits, and underscore
- **THEN** gateway startup SHALL fail
- **AND** no token value SHALL be read from the mapping file.

#### Scenario: Spoofed trusted identity header

- **WHEN** a caller sends a configured trusted Forgejo identity header to `/mcp`
- **THEN** the gateway SHALL reject the request before Forgejo delegation.

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

### Requirement: Curated Phase 2 Forgejo Tools

The gateway SHALL provide a curated set of named Forgejo tools instead of arbitrary endpoint forwarding.

#### Scenario: Bounded branch status readback

- **WHEN** a mapped principal calls `get_branch_status` with a typed `owner/repository@branch` or `forgejo://branch/owner/repository/branch` target and `forgejo:repo:read`
- **THEN** the gateway SHALL retrieve the branch and its combined commit status through the mapped principal
- **AND** SHALL return only bounded branch, protection, required-context, commit, and status summary fields
- **AND** SHALL return no more than 50 required contexts and 50 commit statuses
- **AND** SHALL enforce fixed downstream response-byte and request-timeout limits
- **AND** SHALL require no approval because the operation is read-only

#### Scenario: Invalid branch status input

- **WHEN** `get_branch_status` receives a missing or malformed branch target or any unsupported request field
- **THEN** the gateway SHALL return `400`
- **AND** SHALL NOT call Forgejo
- **AND** SHALL record a structured denial audit event.

#### Scenario: Branch status downstream failure

- **WHEN** Forgejo rejects or cannot complete either branch-status lookup
- **THEN** the gateway SHALL fail without returning partial or unbounded content
- **AND** SHALL record a structured downstream-failure audit event without credentials or response bodies.

#### Scenario: Bounded issue list

- **WHEN** `list_repository_issues` is called with a mapped principal and required scope
- **THEN** the gateway SHALL call Forgejo using that mapped principal's configured token
- **AND** SHALL return bounded issue summaries
- **AND** each summary SHALL include a stable resource URI
- **AND** SHALL return a next cursor when another page may exist.

#### Scenario: Bounded pull request list

- **WHEN** `list_pull_requests` is called with a mapped principal and required scope
- **THEN** the gateway SHALL return bounded pull-request summaries.

#### Scenario: Bounded pull request review list

- **WHEN** `list_pull_request_reviews` is called for `owner/repository#number`
- **THEN** the gateway SHALL return bounded review summaries for that pull request.

#### Scenario: Bounded release list

- **WHEN** `list_releases` is called with a mapped principal and required scope
- **THEN** the gateway SHALL return bounded release summaries.

#### Scenario: Bounded notification list

- **WHEN** `list_notifications` is called with a mapped principal and required scope
- **THEN** the gateway SHALL return bounded notification summaries for that mapped principal.

#### Scenario: Additive issue comment

- **WHEN** `create_issue_comment` is called for `owner/repository#number` with a non-empty body and required scope
- **THEN** the gateway SHALL create the issue or pull-request conversation comment using the mapped Forgejo principal
- **AND** SHALL audit the operation.

#### Scenario: Approval-required mutation attempt

- **WHEN** a caller requests an approval-required mutation without an approval ID
- **THEN** the gateway SHALL return an approval-required response
- **AND** SHALL NOT execute the Forgejo mutation.

#### Scenario: Exact approval validation

- **WHEN** a caller requests an approval-required mutation with an approval ID
- **THEN** the gateway SHALL validate that the approval record exists, has not expired, has not been consumed, is not revoked, matches the operation, target, state, and body hash
- **AND** SHALL require the executing mapped principal to differ from the approving mapped principal
- **AND** SHALL deny mismatched or expired approvals before any Forgejo mutation.

#### Scenario: Approval record creation

- **WHEN** a caller invokes `create_approval` with `forgejo:approval:grant` for an operation that is marked approval-required
- **THEN** the gateway SHALL create a short-lived approval record bound to that exact requested operation payload
- **AND** SHALL store the approval outside caller-controlled request data.

#### Scenario: Approval consumption prevents replay

- **WHEN** an approval-backed operation begins execution
- **THEN** the gateway SHALL append a consumed approval record before calling Forgejo
- **AND** SHALL deny later requests that reuse the same approval ID.

#### Scenario: Pull-request merge dry-run

- **WHEN** `merge_pull_request` is called with `dry_run: true`
- **THEN** the gateway SHALL return the parsed merge target and options
- **AND** SHALL NOT call Forgejo's merge endpoint.

#### Scenario: Approval-backed pull-request merge

- **WHEN** `merge_pull_request` is called without `dry_run` and with a valid approval ID
- **THEN** the gateway SHALL consume the approval
- **AND** SHALL call Forgejo's pull-request merge endpoint using the mapped executor principal's Forgejo token
- **AND** SHALL audit before and after execution.

#### Scenario: Release creation dry-run

- **WHEN** `create_release` is called with `dry_run: true`
- **THEN** the gateway SHALL return the parsed repository target and release options
- **AND** SHALL NOT call Forgejo's release creation endpoint.

#### Scenario: Approval-backed release creation

- **WHEN** `create_release` is called without `dry_run` and with a valid approval ID
- **THEN** the gateway SHALL consume the approval
- **AND** SHALL call Forgejo's release creation endpoint using the mapped executor principal's Forgejo token
- **AND** SHALL audit before and after execution.

### Requirement: Resource URI And CLI Wrapper Support

The gateway SHALL expose stable resource URI identifiers for returned Forgejo resources, and the project SHALL provide a CLI wrapper for curated operations.

#### Scenario: Repository URI target

- **WHEN** a repository target is supplied as `forgejo://repository/{owner}/{repo}`
- **THEN** the gateway SHALL resolve it as the same repository as `owner/repo`.

#### Scenario: Numbered issue or pull URI target

- **WHEN** a numbered target is supplied as `forgejo://issue/{owner}/{repo}/{number}` or `forgejo://pull/{owner}/{repo}/{number}`
- **THEN** the gateway SHALL resolve it as the same numbered target as `owner/repo#number`.

#### Scenario: CLI token handling

- **WHEN** `forgejo-mcpctl` invokes a curated MCP operation
- **THEN** it SHALL read the bearer token from a configured environment variable
- **AND** SHALL NOT require the raw token as a command-line argument.

### Requirement: Generated Forgejo API Coverage

The gateway SHALL pin the Forgejo OpenAPI document used for generated endpoint
classification, SHALL record source provenance and SHA-256, and SHALL classify
every endpoint before any generated coverage is exposed to agents.

#### Scenario: Pinned Swagger provenance

- **WHEN** the generated Forgejo API catalog is built
- **THEN** it SHALL use the pinned `15.0.3+gitea-1.22.0` Swagger document
- **AND** SHALL expose the pinned source SHA-256 in generated coverage output.

#### Scenario: Deny-by-default generated endpoint coverage

- **WHEN** an endpoint exists in the pinned Swagger document but has no reviewed semantic overlay
- **THEN** the gateway SHALL classify it for reporting
- **AND** SHALL keep it disabled as an executable MCP operation.

#### Scenario: Semantic overlay is explicit

- **WHEN** a generated endpoint is exposed through MCP
- **THEN** it SHALL map to a named policy registry operation
- **AND** SHALL use that operation's scope, risk class, approval policy, and audit behavior.

#### Scenario: Bounded coverage readback

- **WHEN** an authenticated caller invokes `forgejo_api_coverage`
- **THEN** the gateway SHALL return bounded endpoint classification metadata
- **AND** SHALL support `limit`, `cursor`, filter, and query parameters
- **AND** SHALL NOT execute arbitrary Forgejo API endpoints.
