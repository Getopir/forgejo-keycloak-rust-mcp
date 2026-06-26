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

## Probe Example

```sh
ACCESS_JWT="$(get-agent-token)"

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

Current PR lifecycle support covers branch-to-PR bootstrap, PR listing, review listing, approval-gated merge, comments, and releases. Standalone PR update, standalone reviewer request, branch status, required checks, and PR check readback are intentionally listed as planned capabilities until their schemas and output limits are reviewed.

## Agent Policy

Agents should request the least powerful operation scope available. High-risk operations such as pull-request creation, pull-request merge, release creation, and repository deletion are policy-classified as approval-required.
