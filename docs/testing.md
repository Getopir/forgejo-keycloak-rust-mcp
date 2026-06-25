# Testing

Run the Rust test suite:

```sh
cargo test --workspace
```

Run the daemon locally:

```sh
cargo run -p forgejo-mcpd -- \
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

Before publishing a public release, run the test suite and a repository secret scanner such as `gitleaks`, `trufflehog`, or your hosting provider's equivalent. Also grep for internal hostnames, private network addresses, and deployment-only project IDs.

```sh
cargo test --workspace
```

The scan should not return committed infrastructure details or secret material.
