# Testing

Run the Rust test suite:

```sh
cargo test --workspace
```

## Coverage

Install the stable Rust coverage tooling and reproduce the CI measurement:

```sh
rustup component add llvm-tools-preview
cargo install --locked --version 0.8.7 cargo-llvm-cov
cargo llvm-cov --workspace --all-features --summary-only \
  --ignore-filename-regex 'forgejo-mcpd[\\/]src[\\/](main|bin[\\/]forgejo-mcpctl)\.rs' \
  --fail-under-lines 55 \
  --fail-under-functions 50 \
  --fail-under-regions 55
```

The `1.2.11` baseline is 59.62% lines, 52.48% functions, and 58.50%
regions across domain code. The measurement includes identity, policy, approval,
audit, rate limiting, principal mapping, and the complete Forgejo client/parser
module. It excludes only the daemon and CLI process-entrypoint files from the
percentage calculation; those files remain compiled, strictly linted, and
covered by focused validation tests where their logic is separable.

Both internal Forgejo and public Codeberg CI enforce the thresholds. Coverage
summaries are therefore published in the corresponding push and pull-request
job logs. Lowering a threshold or expanding the exclusion requires maintainer
review and a documented rationale.

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
curl -sS http://127.0.0.1:7080/capabilities
```

For a configured `2.x` deployment, `/health` must report required version
`16.0.0` and the verified Forgejo version. Startup must fail before listening
when `/api/v1/version` reports an older, newer, prerelease, malformed, or
unreachable contract. Unit tests also assert that all existing semantic
operations remain mapped to the pinned Forgejo 16 document while all 15 new
upstream endpoints remain disabled.

The `2.1.0` branch-status tests additionally cover typed and malformed targets,
unknown and inapplicable request fields, scope denial, approval-free policy,
encoded branch names, successful two-request readback, downstream failure,
request timeout inheritance, 64 KiB and 256 KiB downstream byte caps, 50-item
context/status caps, and bounded UTF-8 string fields.

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
| `get_branch_status` | `forgejo:repo:read` | `200 allowed=true approval_required=false` |
| `get_branch_status` | no `forgejo:repo:read` | `403 allowed=false` before Forgejo access |
| `create_pull_request` | `forgejo:pr:write` | `200 allowed=true approval_required=true` |
| `merge_pull_request` | `forgejo:pr:merge` | `200 allowed=true approval_required=true` |
| `delete_repository` | `forgejo:org:admin` | `200 allowed=true approval_required=true` |
| `unknown_operation` | any | `400` |

## Live Keycloak Agent Matrix

Before cutting `0.4.0`, the gateway was tested with two Keycloak service-account agents against an internal Forgejo target repository.

The full-scope agent carried:

```text
forgejo:repo:read forgejo:issue:write forgejo:pr:write forgejo:pr:merge forgejo:org:admin
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
