# Configuration

`forgejo-mcpd` accepts command-line flags and matching environment variables.

| Flag | Environment | Required | Description |
| --- | --- | --- | --- |
| `--issuer` | `FORGEJO_MCPD_ISSUER` | Yes | Expected JWT issuer. Must match the `iss` claim exactly. |
| `--discovery-url` | `FORGEJO_MCPD_DISCOVERY_URL` | No | OIDC discovery URL. Defaults to `<issuer>/.well-known/openid-configuration`. |
| `--audience` | `FORGEJO_MCPD_AUDIENCE` | Yes | Required JWT audience for this MCP resource. |
| `--resource` | `FORGEJO_MCPD_RESOURCE` | Yes | Public resource URL advertised to MCP clients. |
| `--bind` | `FORGEJO_MCPD_BIND` | No | Socket address to listen on. Defaults to `127.0.0.1:7080`. |

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

## Reverse Proxy

Terminate TLS at a reverse proxy and forward to `127.0.0.1:7080`.

Minimum recommended headers:

```text
Host
X-Forwarded-Proto
X-Forwarded-For
```

Do not trust caller-supplied Forgejo identity headers. Forgejo principal mapping is planned as a later phase and must derive from validated Keycloak identity.
