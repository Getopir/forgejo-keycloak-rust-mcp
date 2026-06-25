# Features

`1.0.0` is the first stable release of the Forgejo Keycloak Rust MCP gateway.

## Identity And Policy

- Keycloak bearer-token validation.
- OAuth protected-resource metadata.
- Explicit operation registry with required scope, risk class, and approval policy.
- Token-free audit events.
- Trusted-header spoof rejection.

## Forgejo Principal Mapping

- Explicit Keycloak `(issuer, subject)` to Forgejo account mapping.
- Disabled, unknown, duplicate, and malformed mapping rejection.
- Runtime-only Forgejo token environment variables.
- Optional trusted-header delegation derived from the server-side mapping.

## MCP Tools

- `gateway_probe`
- `list_repository_metadata`
- `list_repository_issues`
- `create_issue_comment`
- `list_pull_requests`
- `list_pull_request_reviews`
- `list_releases`
- `list_notifications`
- `forgejo_api_coverage`
- `create_approval`
- `create_release`
- `merge_pull_request`

## Approval Gates

Approval records are file-backed, short-lived, exact-payload-bound, single-use,
and require different mapped principals for approval and execution.

Executable high-risk tools in `1.0.0`:

- `merge_pull_request`
- `create_release`

Admin and destructive execution remains disabled.

## Generated API Coverage

The gateway pins the Forgejo `15.0.3+gitea-1.22.0` Swagger document and
classifies all 491 operations. Only 9 reviewed semantic-overlay operations are
executable. The other 482 are metadata-only and disabled.

Agents can inspect this safely:

```sh
forgejo-mcpctl api-coverage --filter semantic_overlay --limit 25
forgejo-mcpctl api-coverage --filter destructive --query repo --limit 25
```
