# Roadmap

## Phase 0

- Keycloak JWT validation.
- OAuth protected-resource metadata.
- `/mcp` policy probe.
- Audit event schema.

## Phase 1

Phase 1 turns the current identity and policy probe into a usable read-only Forgejo gateway. The goal is to let an agent prove who it is through Keycloak, map that identity to the correct Forgejo account, and read repository metadata without giving the agent a reusable Forgejo token.

### Principal Mapping From Keycloak Subject To Forgejo Account

The gateway will maintain a deterministic mapping between the Keycloak principal in the access token and the Forgejo account that should be used for downstream authorization.

The minimum mapping inputs are:

- Keycloak issuer.
- Keycloak subject (`sub`).
- Optional Keycloak preferred username, email, groups, and roles.
- Forgejo user ID or username.
- Mapping status, such as active, disabled, or pending review.

The mapping must be explicit. The gateway should not trust an arbitrary username supplied by the agent or infer a Forgejo user from display names. The Keycloak `sub` claim is stable and should be the primary identity key. Username and email claims are useful for operator display and audit, but they can change and should not be the only binding.

Expected behavior:

- A token from an unknown subject is rejected or receives only a low-risk discovery response.
- A disabled mapping is rejected even if the token is valid.
- A mapped subject is recorded in audit output as both the Keycloak principal and the Forgejo principal.
- Mapping changes are auditable and should be treated as security-sensitive administration.

### Forgejo Trusted-Header Delegation

Forgejo supports deployments where a trusted reverse proxy supplies authenticated user information through headers. Phase 1 should use that pattern only between the gateway and Forgejo, never directly from arbitrary clients.

The intended flow is:

- The agent calls the Rust MCP gateway with a Keycloak bearer token.
- The gateway validates issuer, signature, expiry, audience, and required claims.
- The gateway resolves the mapped Forgejo principal.
- The gateway forwards the request to Forgejo through a private network path or loopback path.
- The gateway injects the trusted identity header expected by Forgejo.
- Forgejo still performs its own repository and organization ACL checks for that mapped user.

Security requirements:

- Forgejo must only accept trusted identity headers from the gateway or trusted reverse proxy.
- Public clients must not be able to send or spoof those headers.
- Header names and trusted proxy ranges must be documented in deployment configuration.
- Every delegated request must produce an audit record that includes the Keycloak subject, Forgejo principal, operation name, target, and allow/deny decision.

### Read-Only Repository Metadata Tool

The first concrete Forgejo-backed MCP tool should be read-only repository metadata. It should let agents answer basic questions about a repository without exposing write capabilities.

Expected inputs:

- Repository owner.
- Repository name.
- Optional fields selector for metadata categories.

Expected output:

- Repository full name.
- Visibility and archived state.
- Default branch.
- Description.
- Clone URLs that are safe to expose.
- Last updated timestamp.
- Open issue and pull-request counts if available.
- Permissions for the mapped Forgejo principal, such as read, write, admin, or none.

Non-goals for Phase 1:

- Creating issues.
- Writing comments.
- Merging pull requests.
- Reading secrets, deploy keys, webhooks, private environment variables, or admin settings.

Acceptance criteria:

- A mapped read-only agent can fetch metadata for a repository it can read in Forgejo.
- The same agent is denied for a repository it cannot read in Forgejo.
- The response contains bounded, predictable fields.
- The audit log shows the Keycloak subject, Forgejo account, repository target, and policy result.

## Phase 2

Phase 2 adds a curated set of agent-safe Forgejo workflows. The goal is not full API coverage. The goal is a small, documented set of tools that agents can use reliably without surprising side effects.

### Curated Issue, Pull Request, Review, Release, And Notification Tools

The gateway should expose named tools for common work:

- List, create, and update issues within policy limits.
- Add issue comments.
- List pull requests and read pull-request metadata.
- Add review comments or review summaries.
- Prepare release notes or create releases when policy allows it.
- Read notifications relevant to the mapped Forgejo principal.

Each tool should have a stable schema and a narrow operation class. For example, `issue.comment.create` is easier to authorize and audit than a generic `forgejo.request` tool.

Tool design rules:

- Prefer specific tools over arbitrary endpoint forwarding.
- Separate read operations from write operations.
- Separate low-risk writes, such as comments, from high-risk writes, such as merge or release publication.
- Include target repository and target object IDs in the input schema.
- Return enough metadata for follow-up calls without returning unbounded payloads.

### Output Limits And Cursor-Based Responses

Agent responses must be bounded. Large repositories can have many issues, comments, reviews, releases, and notifications. Returning everything in one call is fragile and can leak more data than the agent needs.

Every list-style tool should support:

- `limit`, capped by server configuration.
- `cursor` or page token for the next response.
- Stable sort order.
- Optional filters such as state, labels, author, assignee, branch, or updated-since.

The response should include:

- Returned items.
- Cursor for the next page when more data exists.
- Count of returned items.
- Whether the response was truncated.

The gateway should enforce maximum output size even when Forgejo returns more data than expected.

### Approval Gates For Mutating Actions

Mutating actions must be split by risk. Some actions can be allowed directly by policy, while others should require explicit approval before execution.

Examples of direct low-risk actions:

- Add an issue comment.
- Add a pull-request review comment.
- Mark a notification as read.

Examples of approval-gated actions:

- Merge a pull request.
- Create or publish a release.
- Close a high-priority issue.
- Change labels, milestones, or assignments in protected repositories.

Approval gate behavior:

- The first call prepares an action plan and returns an approval request.
- The approval request records the agent, mapped user, target, operation, risk class, and exact payload.
- A separate approval decision authorizes or rejects execution.
- Execution only happens if the payload still matches the approved request.
- Expired, changed, or replayed approvals are denied.

## Phase 3

Phase 3 expands coverage after the gateway has proven identity, read-only tools, curated workflows, bounded output, and approval gates. Generated coverage is useful only after every endpoint has been classified by risk and authorization behavior.

### Generated Forgejo API Coverage After Endpoint Classification

Generated tools should be created from Forgejo API metadata only after endpoint classification. The classification step determines whether an endpoint is safe to expose and what controls it needs.

Each endpoint should be classified by:

- Operation type: read, create, update, delete, admin, or system.
- Target type: repository, issue, pull request, release, organization, user, admin, or settings.
- Risk class: low, medium, high, destructive, or admin.
- Required Forgejo permission.
- Required gateway scope.
- Whether approval is required.
- Whether output limits are required.
- Whether the endpoint is excluded.

Generated coverage should not mean unrestricted coverage. The generator should emit tools only for endpoints with a reviewed classification. Unknown endpoints should stay disabled.

Expected safeguards:

- Generated names must be stable and reviewable.
- Generated schemas must preserve typed inputs instead of accepting arbitrary JSON.
- Generated tools must still call the policy engine.
- Generated tools must still audit every decision and execution result.

### Admin Separation And Destructive-Operation Safeguards

Admin and destructive operations need stricter boundaries than normal repository work.

Admin separation means:

- Normal agents do not receive admin tools.
- Repository admin operations are separate from instance admin operations.
- Organization owner actions are separate from repository maintainer actions.
- Admin mappings require explicit operator review.

Destructive-operation safeguards should apply to:

- Repository deletion or transfer.
- Branch deletion.
- Force push or protected-branch changes.
- Webhook, deploy-key, runner, or secret changes.
- User, organization, team, or membership changes.
- Release deletion or asset replacement.

Required controls:

- High-risk scope in the Keycloak token.
- Active Forgejo principal mapping with required Forgejo permission.
- Explicit approval with exact payload binding.
- Optional multi-party approval for destructive or admin actions.
- Dry-run preview where Forgejo supports it or where the gateway can compute one.
- Clear audit record before and after execution.
- Deny-by-default behavior for unknown, unclassified, or partially classified operations.

Phase 3 is complete only when generated coverage is safer than direct API-token use by an agent. If a generated tool cannot provide policy, approval, output limits, and audit, it should remain disabled.
