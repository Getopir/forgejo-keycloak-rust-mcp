# Agent Setup

Agents should call the gateway with a short-lived Keycloak access token. Do not give agents long-lived Forgejo personal access tokens for this gateway.

## Agent Requirements

An agent needs:

- The MCP resource URL, for example `https://mcp.example.org/mcp`.
- A Keycloak token endpoint or broker that returns access tokens for the MCP audience.
- The operation name it should request.
- The target repository or organization string for audit context.

## Discovery

Agents can discover operation names and policy shape before choosing a tool:

```sh
curl -sS https://mcp.example.org/capabilities
```

The response lists registered operations, required scopes, risk classes, approval requirements, and planned-but-disabled operations. This endpoint is unauthenticated and never returns repository data or secrets.

## Token Handling

Agents must treat access tokens as secrets:

- Keep tokens in memory or an OS secret store.
- Do not write tokens into prompts, logs, issue comments, wiki pages, or task records.
- Refresh tokens before expiry instead of reusing stale tokens.

Recommended token path:

1. The agent receives a scoped work order from OPIR-P or another trusted control plane.
2. The control plane or token broker validates the work order and returns a short-lived Keycloak access token for the MCP audience.
3. The agent stores that token in memory, commonly as `ACCESS_JWT`.
4. The agent calls `/mcp` with `Authorization: Bearer ${ACCESS_JWT}`.

The gateway does not require agents to know Forgejo personal access tokens. Downstream Forgejo tokens stay in the gateway runtime environment and are selected through the principal map.

## OpenBao-Backed Token Handle

Deployments that already use OpenBao for agent secrets can expose the Keycloak
MCP client through an agent-scoped handle instead of placing credentials in
repo files or prompts.

Recommended handle shape:

```text
kv/data/<deployment>/agents/<agent-id>/forgejo-keycloak-rust-mcp
```

Required fields inside the handle:

```json
{
  "client_id": "forgejo-mcp-agent",
  "client_secret": "<stored only in OpenBao>",
  "token_url": "https://keycloak.example.org/realms/forgejo-agents/protocol/openid-connect/token",
  "scope": "forgejo:repo:read forgejo:pr:write",
  "gateway_url": "https://mcp.example.org/mcp"
}
```

Agent flow:

1. Authenticate the agent to OpenBao with its own identity, for example a
   Keycloak JWT bound to that agent's OpenBao role.
2. Read only the agent-scoped MCP handle.
3. Exchange `client_id` and `client_secret` with Keycloak using
   `grant_type=client_credentials`.
4. Store the returned short-lived MCP bearer token in memory as `ACCESS_JWT`.
5. Call the gateway with `Authorization: Bearer ${ACCESS_JWT}`.

Do not expose this handle to every agent by default. Grant it only to agents or
work-order roles that are allowed to use the mapped Forgejo principal. For
approval-required operations, configure at least two mapped principals so the
approver and executor are different identities.

## Probe Example

```sh
ACCESS_JWT="<keycloak-access-token-from-your-token-broker>"

curl -sS \
  -H "Authorization: Bearer ${ACCESS_JWT}" \
  -H "Content-Type: application/json" \
  -d '{"operation":"gateway_probe","target":"owner/repository"}' \
  https://mcp.example.org/mcp
```

Expected outcomes:

- `200`: token is valid and contains the required operation scope.
- `401`: token is missing, expired, malformed, wrong issuer, wrong audience, or fails signature validation.
- `403`: token is valid but lacks the required operation scope.
- `400`: operation is unknown.

## Pull-Request Workflow

Use `create_pull_request` to turn a pushed branch into a Forgejo pull request.

Preview:

```sh
forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp \
  --head feature-branch \
  --base main \
  --title "Add feature" \
  --body "Implementation notes" \
  --reviewer maintainer \
  --dry-run
```

Approval and execution:

```sh
forgejo-mcpctl create-approval create_pull_request GetOpir/forgejo-keycloak-rust-mcp \
  --body '{"head":"feature-branch","base":"main","title":"Add feature","body":"Implementation notes"}'

forgejo-mcpctl create-pull-request GetOpir/forgejo-keycloak-rust-mcp \
  --head feature-branch \
  --base main \
  --title "Add feature" \
  --body "Implementation notes" \
  --approval-id "$APPROVAL_ID"
```

Current PR lifecycle support covers bounded branch-status readback, branch-to-PR bootstrap, PR listing, review listing, approval-gated merge, comments, additive issue creation, wiki readback/publication, safe credential-reference status, and releases. Standalone PR update, standalone reviewer request, required checks, and PR check readback remain planned until their schemas and output limits are reviewed.

## Issue And Wiki Workflow

Use `create_issue` for durable Forgejo repair tickets. It is additive and still depends on the mapped Forgejo user's repository ACLs:

```sh
forgejo-mcpctl create-issue GetOpir/forgejo-keycloak-rust-mcp \
  --title "Repair MCP adapter coverage" \
  --body "Observed through OPIR-P PRJ-SM-018."
```

Use wiki tools for operating packages and runbooks. Wiki create/update bodies use `content_base64`; do not put raw secrets in page content or commit messages.

```sh
forgejo-mcpctl wiki-pages GetOpir/forgejo-keycloak-rust-mcp --limit 25
forgejo-mcpctl create-wiki-page GetOpir/forgejo-keycloak-rust-mcp \
  --title Agent-Runbook \
  --content-base64 IyBBZ2VudCBSdW5ib29rCg== \
  --dry-run
```

Use `credential_reference_status` when an agent needs to prove whether its mapped runtime token reference is configured. The response reports environment-variable presence and identity metadata only; it never returns token values.

## Agent Policy

Agents should request the least powerful operation scope available. High-risk operations such as pull-request creation, pull-request merge, release creation, wiki publication, and repository deletion are policy-classified as approval-required.
