# Codeberg Release: v0.4.2

Tag: `v0.4.2`

Title: `v0.4.2 - Phase 0 public beta and roadmap documentation`

## Release Notes

`v0.4.2` is the public Phase 0 beta for `forgejo-keycloak-rust-mcp`.

This release is useful for validating the identity and policy boundary before Forgejo API delegation is enabled.

## Current Capabilities

- Validates Keycloak-issued bearer tokens with issuer, audience, expiry, and JWKS checks.
- Serves OAuth protected-resource metadata for MCP clients.
- Provides a deterministic operation policy registry.
- Emits structured audit records without tokens or secret values.
- Exposes an authenticated `/mcp` policy probe for agents.
- Documents Phase 1, Phase 2, and Phase 3 roadmap details in the wiki.

## Not Yet Included

- Forgejo principal mapping.
- Forgejo trusted-header delegation.
- Read-only repository metadata execution.
- Issue, pull request, review, release, notification, admin, or generated API tools.

## License

`AGPL-3.0-or-later`

## Verification

- `cargo check --workspace`
- `cargo fmt --check`
- `cargo test --workspace`
- `git diff --check`
- public Codeberg repository readback
- Codeberg wiki readback
- secret/token scan
- internal-host scan outside preserved original inputs

## Links

- Source: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp`
- Wiki: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/wiki`
- Roadmap: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/wiki/Roadmap`
