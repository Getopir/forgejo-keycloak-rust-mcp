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

## Keycloak Setup

Create a dedicated client or audience for the gateway. The access token presented to `/mcp` must contain:

- `iss`: the configured issuer.
- `aud`: the configured audience, either as a string or an array.
- `exp`: a future expiry.
- `sub`: immutable subject for the human or agent.
- `scope`: space-separated scopes used by the policy registry.

For the current release, useful scopes are:

- `forgejo:repo:read`
- `forgejo:issue:write`
- `forgejo:pr:merge`
- `forgejo:org:admin`

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

## Reverse Proxy

Terminate TLS at a reverse proxy and forward to `127.0.0.1:7080`.

Minimum recommended headers:

```text
Host
X-Forwarded-Proto
X-Forwarded-For
```

Do not trust caller-supplied Forgejo identity headers. Forgejo principal mapping must derive from validated Keycloak identity and the configured principal map.

## Forgejo Delegation

The read-only repository metadata tool uses the Forgejo API. Forgejo supports `Authorization: token ...` and `Authorization: Bearer ...` API authentication. The gateway uses the mapped principal's token environment variable and does not expose that token in responses or audit records.

Forgejo reverse-proxy authentication can read trusted user, email, and full-name headers. Use it only on a trusted private path where public clients cannot send or spoof those headers. The gateway can derive those header names and values from the mapped principal for deployments that use a reverse proxy in front of Forgejo.
