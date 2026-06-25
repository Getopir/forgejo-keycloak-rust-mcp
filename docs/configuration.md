# Configuration

`forgejo-mcpd` accepts command-line flags and matching environment variables.

| Flag | Environment | Required | Description |
| --- | --- | --- | --- |
| `--issuer` | `FORGEJO_MCPD_ISSUER` | Yes | Expected JWT issuer. Must match the `iss` claim exactly. |
| `--discovery-url` | `FORGEJO_MCPD_DISCOVERY_URL` | No | OIDC discovery URL. Defaults to `<issuer>/.well-known/openid-configuration`. |
| `--audience` | `FORGEJO_MCPD_AUDIENCE` | Yes | Required JWT audience for this MCP resource. |
| `--resource` | `FORGEJO_MCPD_RESOURCE` | Yes | Public resource URL advertised to MCP clients. |
| `--bind` | `FORGEJO_MCPD_BIND` | No | Socket address to listen on. Defaults to `127.0.0.1:7080`. |
| `--principal-map` | `FORGEJO_MCPD_PRINCIPAL_MAP` | No | Path to the Phase 1 Keycloak-to-Forgejo mapping JSON file. Required for Forgejo-backed tools. |
| `--forgejo-url` | `FORGEJO_MCPD_FORGEJO_URL` | No | Base URL for the Forgejo instance. Required for Forgejo-backed tools. |
| `--trusted-user-header` | `FORGEJO_MCPD_TRUSTED_USER_HEADER` | No | Trusted reverse-proxy username header generated from the mapped Forgejo login. Defaults to `X-WEBAUTH-USER`. |
| `--trusted-email-header` | `FORGEJO_MCPD_TRUSTED_EMAIL_HEADER` | No | Optional trusted reverse-proxy email header generated from the mapping. |
| `--trusted-full-name-header` | `FORGEJO_MCPD_TRUSTED_FULL_NAME_HEADER` | No | Optional trusted reverse-proxy full-name header generated from the mapping. |
| `--max-page-limit` | `FORGEJO_MCPD_MAX_PAGE_LIMIT` | No | Maximum item count for list-style Phase 2 responses. Defaults to `50`. |
| `--approval-store` | `FORGEJO_MCPD_APPROVAL_STORE` | No | Path to an append-only JSONL file for short-lived approval records. Required to validate approval IDs for high-risk gates. |
| `--approval-ttl-seconds` | `FORGEJO_MCPD_APPROVAL_TTL_SECONDS` | No | Approval lifetime in seconds. Defaults to `900`. |

## Keycloak Setup

Create a dedicated client or audience for the gateway. The access token presented to `/mcp` must contain:

- `iss`: the configured issuer.
- `aud`: the configured audience, either as a string or an array.
- `exp`: a future expiry.
- `sub`: immutable subject for the human or agent.
- `scope`: space-separated scopes used by the policy registry.

For the current release, useful scopes are:

- `forgejo:repo:read`
- `forgejo:issue:read`
- `forgejo:issue:write`
- `forgejo:pr:read`
- `forgejo:pr:merge`
- `forgejo:release:read`
- `forgejo:release:write`
- `forgejo:notification:read`
- `forgejo:org:admin`
- `forgejo:approval:grant`

## Principal Mapping

Phase 1 Forgejo-backed tools require an explicit mapping from immutable Keycloak identity to Forgejo account.

Example mapping file:

```json
{
  "mappings": [
    {
      "issuer": "https://keycloak.example.org/realms/forgejo-agents",
      "subject": "00000000-0000-0000-0000-000000000001",
      "forgejo_login": "agent-reader",
      "forgejo_user_id": 42,
      "forgejo_email": "agent-reader@example.org",
      "forgejo_full_name": "Agent Reader",
      "enabled": true,
      "principal_type": "agent",
      "api_token_env": "FORGEJO_AGENT_READER_TOKEN"
    }
  ]
}
```

The mapping file stores the environment variable name, not the token value. Set the token separately in the runtime environment:

```sh
export FORGEJO_AGENT_READER_TOKEN=...
```

Unknown or disabled mappings are denied before any Forgejo call.

The gateway validates the mapping file at startup:

- `(issuer, subject)` entries must be unique after issuer normalization.
- `issuer`, `subject`, and `forgejo_login` must not be empty.
- `api_token_env` may contain only ASCII letters, digits, and underscore.
- token values are never read from the mapping file.

## Reverse Proxy

Terminate TLS at a reverse proxy and forward to `127.0.0.1:7080`.

Minimum recommended headers:

```text
Host
X-Forwarded-Proto
X-Forwarded-For
```

Do not trust caller-supplied Forgejo identity headers. Forgejo principal mapping must derive from validated Keycloak identity and the configured principal map. Requests that arrive at `/mcp` with configured trusted identity headers, such as `X-WEBAUTH-USER`, are rejected as spoof attempts.

## Forgejo Delegation

Forgejo-backed tools use the Forgejo API. Forgejo supports `Authorization: token ...` and `Authorization: Bearer ...` API authentication. The gateway uses the mapped principal's token environment variable and does not expose that token in responses or audit records.

Forgejo reverse-proxy authentication can read trusted user, email, and full-name headers. Use it only on a trusted private path where public clients cannot send or spoof those headers. The gateway can derive those header names and values from the mapped principal for deployments that use a reverse proxy in front of Forgejo.

## Pagination

List-style tools accept `limit` and `cursor` request fields. The gateway treats `cursor` as an opaque page token and caps `limit` at `FORGEJO_MCPD_MAX_PAGE_LIMIT`.

Example:

```json
{
  "operation": "list_repository_issues",
  "target": "rawholding/forgejo-keycloak-rust-mcp",
  "state": "open",
  "limit": 25,
  "cursor": "2"
}
```

## Approval Store

Set `FORGEJO_MCPD_APPROVAL_STORE` on deployments that need approval-gated operations to reject fake approval IDs and validate real ones:

```sh
export FORGEJO_MCPD_APPROVAL_STORE=/var/lib/forgejo-mcpd/approvals.jsonl
export FORGEJO_MCPD_APPROVAL_TTL_SECONDS=900
```

The approval file is append-only JSONL. Store it on a filesystem readable and writable only by the gateway service account. Do not place it inside the public repository, a web root, or a shared workspace.

Approval-backed execution requires a different mapped principal from the one that created the approval. For example, a human maintainer can create an approval and an identified agent can execute it, or one service principal can approve and another service principal can execute. Reusing the same mapped Forgejo login for both roles is denied.
