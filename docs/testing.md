# Testing

Run the Rust test suite:

```sh
cargo test --workspace
```

Run the daemon locally:

```sh
cargo run -p forgejo-keycloak-rust-mcp --bin forgejo-keycloak-rust-mcpd -- \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource http://127.0.0.1:7080/mcp \
  --bind 127.0.0.1:7080
```

Smoke checks:

```sh
curl -sS http://127.0.0.1:7080/health
curl -sS http://127.0.0.1:7080/.well-known/oauth-protected-resource
```

Unauthenticated `/mcp` requests should return `401`:

```sh
curl -i \
  -H "Content-Type: application/json" \
  -d '{"operation":"gateway_probe"}' \
  http://127.0.0.1:7080/mcp
```

With a valid token, test these operation decisions:

| Operation | Token scope | Expected |
| --- | --- | --- |
| `gateway_probe` | `forgejo:repo:read` | `200 allowed=true` |
| `gateway_probe` | no `forgejo:repo:read` | `403 allowed=false` |
| `merge_pull_request` | `forgejo:pr:merge` | `200 allowed=true approval_required=true` |
| `delete_repository` | `forgejo:org:admin` | `200 allowed=true approval_required=true` |
| `unknown_operation` | any | `400` |

## Live Keycloak Agent Matrix

Before cutting `0.4.0`, the gateway was tested with two Keycloak service-account agents against an internal Forgejo target repository.

The full-scope agent carried:

```text
forgejo:repo:read forgejo:issue:write forgejo:pr:merge forgejo:org:admin
```

The read-only agent carried:

```text
forgejo:repo:read
```

Required checks:

| Case | Expected |
| --- | --- |
| Full-scope agent calls all registered operations | `200 allowed=true` |
| Read-only agent calls `gateway_probe` | `200 allowed=true` |
| Read-only agent calls write, merge, or delete operations | `403 allowed=false` |
| Unknown operation | `400` |
| Missing bearer token | `401` with protected-resource metadata |
| Invalid scheme or invalid JWT | `401` |

Do not store the test client secrets or bearer tokens in this repository. If a live realm advertises an internal hostname in `jwks_uri`, run the gateway where that hostname resolves or use an environment-local discovery shim that preserves the real issuer and points `jwks_uri` at the same Keycloak instance through a resolvable LAN name.

Before publishing a public release, run the test suite and a repository secret scanner such as `gitleaks`, `trufflehog`, or your hosting provider's equivalent. Also grep for internal hostnames, private network addresses, and deployment-only project IDs.

```sh
cargo test --workspace
```

The scan should not return committed infrastructure details or secret material.
