# Roadmap

## Phase 0

- Keycloak JWT validation.
- OAuth protected-resource metadata.
- `/mcp` policy probe.
- Audit event schema.

## Phase 1

Phase 1 turns the identity and policy probe into a usable read-only Forgejo gateway. Version `0.5.0` implemented the baseline: an agent proves who it is through Keycloak, the gateway maps that identity to a Forgejo account, and the gateway can read bounded repository metadata through Forgejo API. Version `0.6.0` hardens that baseline with duplicate-map validation, required-field validation, token environment-name validation, and trusted-header spoof rejection.

### Principal Mapping From Keycloak Subject To Forgejo Account

The gateway maintains a deterministic mapping between the Keycloak principal in the access token and the Forgejo account that should be used for downstream authorization.

The mapping inputs are:

- Keycloak issuer.
- Keycloak subject (`sub`).
- Forgejo login.
- Optional Forgejo user ID.
- Optional Forgejo email and full name for trusted-header delegation.
- Enabled or disabled mapping state.
- Principal type: human, agent, or unknown.
- Environment variable name that contains the mapped principal's Forgejo API token.

The mapping must be explicit. The gateway should not trust an arbitrary username supplied by the agent or infer a Forgejo user from display names. The Keycloak `sub` claim is stable and should be the primary identity key. Username and email claims are useful for operator display and audit, but they can change and should not be the only binding.

Expected behavior:

- A token from an unknown subject is rejected before Forgejo-backed calls.
- A disabled mapping is rejected even if the token is valid.
- A mapped subject is recorded in audit output as both the Keycloak principal and the Forgejo principal.
- Mapping changes are auditable and should be treated as security-sensitive administration.
- Duplicate mappings are rejected at startup.
- Token environment names are constrained to ASCII letters, digits, and underscore.

### Forgejo Trusted-Header Delegation

Forgejo supports deployments where a trusted reverse proxy supplies authenticated user information through headers. Phase 1 derives those headers from the mapped Forgejo principal. Use that pattern only between the gateway or a trusted reverse proxy and Forgejo, never directly from arbitrary clients.

The trusted-header flow is:

- The agent calls the Rust MCP gateway with a Keycloak bearer token.
- The gateway validates issuer, signature, expiry, audience, and required claims.
- The gateway resolves the mapped Forgejo principal.
- The gateway derives the trusted identity headers expected by Forgejo from the mapping.
- A deployment-specific reverse proxy may forward those headers to Forgejo over a private path.
- Forgejo still performs its own repository and organization ACL checks for that mapped user.

Security requirements:

- Forgejo must only accept trusted identity headers from the gateway or trusted reverse proxy.
- Public clients must not be able to send or spoof those headers.
- The gateway rejects `/mcp` requests that include configured trusted identity headers.
- Header names and trusted proxy ranges must be documented in deployment configuration.
- Every delegated request must produce an audit record that includes the Keycloak subject, Forgejo principal, operation name, target, and allow/deny decision.

### Read-Only Repository Metadata Tool

The first concrete Forgejo-backed MCP tool is read-only repository metadata. It lets agents answer basic questions about a repository without exposing write capabilities.

Inputs:

- `operation`: `list_repository_metadata`.
- `target`: repository in `owner/repository` form.

Output:

- Repository full name.
- Visibility and archived state.
- Default branch.
- Description.
- Clone URLs that are safe to expose.
- Last updated timestamp.
- Open issue and pull-request counts if available.
- Permissions for the mapped Forgejo principal, such as read, write, admin, or none.
- Trusted delegation headers derived from the mapping.

Non-goals for Phase 1:

- Creating issues.
- Writing comments.
- Merging pull requests.
- Reading secrets, deploy keys, webhooks, private environment variables, or admin settings.

Implemented acceptance criteria:

- A mapped read-only agent can fetch metadata for a repository it can read in Forgejo.
- The same agent is denied for a repository it cannot read in Forgejo.
- The response contains bounded, predictable fields.
- The audit log shows the Keycloak subject, Forgejo account, repository target, and policy result.

## Phase 2

Phase 2 adds a curated set of agent-safe Forgejo workflows. Version `0.6.0` implements the first bounded baseline, `0.7.0` adds resource URIs plus CLI wrappers, and `0.8.0` hardens approval gates with file-backed exact-payload approval records. The goal is not full API coverage. The goal is a small, documented set of tools that agents can use reliably without surprising side effects.

### Curated Issue, Pull Request, Review, Release, And Notification Tools

The `0.6.0` baseline exposes named tools for common work:

- `list_repository_issues`
- `create_issue_comment`
- `list_pull_requests`
- `list_pull_request_reviews`
- `list_releases`
- `list_notifications`
- `create_release` as an approval-gated policy entry, not an executable release publisher yet

Each tool should have a stable schema and a narrow operation class. For example, `issue.comment.create` is easier to authorize and audit than a generic `forgejo.request` tool.

The `0.7.0` resource URI baseline returns stable identifiers such as:

- `forgejo://repository/{owner}/{repo}`
- `forgejo://issue/{owner}/{repo}/{number}`
- `forgejo://pull/{owner}/{repo}/{number}`
- `forgejo://pull-review/{owner}/{repo}/{pull_number}/{review_id}`
- `forgejo://release/{owner}/{repo}/{tag}`
- `forgejo://notification/{id}`
- `forgejo://issue-comment/{owner}/{repo}/{issue_number}/{comment_id}`

The `forgejo-mcpctl` CLI wraps the curated tools for operator and agent harness usage while reading bearer tokens from environment variables.

Tool design rules:

- Prefer specific tools over arbitrary endpoint forwarding.
- Separate read operations from write operations.
- Separate low-risk writes, such as comments, from high-risk writes, such as merge or release publication.
- Include target repository and target object IDs in the input schema.
- Return enough metadata for follow-up calls without returning unbounded payloads.

### Output Limits And Cursor-Based Responses

Agent responses must be bounded. Large repositories can have many issues, comments, reviews, releases, and notifications. Returning everything in one call is fragile and can leak more data than the agent needs.

Every implemented list-style tool supports:

- `limit`, capped by server configuration.
- `cursor` as the next page token.
- Optional state filters where the Forgejo endpoint supports them.

The response includes:

- Returned items.
- Cursor for the next page when more data exists.
- Effective server-capped limit.

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

The `0.6.0` approval-gate baseline denies high-risk execution when no approval ID is supplied. Version `0.8.0` adds persistent approval records with this behavior:

- A `create_approval` call records the requested high-risk operation, mapped user, target, state, body hash, and expiry.
- The approval request records both the Keycloak subject and the mapped Forgejo account.
- A later high-risk operation call must supply the approval ID and the same operation payload.
- Expired, changed, revoked, missing, or wrong-principal approvals are denied.
- High-risk execution still remains disabled until the executable mutation tools are implemented and tested.

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
