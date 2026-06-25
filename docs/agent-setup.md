# Agent Setup

Agents should call the gateway with a short-lived Keycloak access token. Do not give agents long-lived Forgejo personal access tokens for this gateway.

## Agent Requirements

An agent needs:

- The MCP resource URL, for example `https://mcp.example.org/mcp`.
- A Keycloak token endpoint or broker that returns access tokens for the MCP audience.
- The operation name it should request.
- The target repository or organization string for audit context.

## Token Handling

Agents must treat access tokens as secrets:

- Keep tokens in memory or an OS secret store.
- Do not write tokens into prompts, logs, issue comments, wiki pages, or task records.
- Refresh tokens before expiry instead of reusing stale tokens.

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

## Agent Policy

Agents should request the least powerful operation scope available. High-risk operations such as pull-request merge and repository deletion are policy-classified as approval-required even before Forgejo delegation is implemented.
