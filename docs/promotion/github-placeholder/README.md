# Forgejo Keycloak Rust MCP

This GitHub repository is a pointer for discovery only.

The project has moved to Codeberg:

https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp

## What It Does

`forgejo-keycloak-rust-mcp` is a clean-room Rust MCP gateway for Forgejo.

It lets humans and AI agents authenticate with Keycloak before accessing
Forgejo through a controlled MCP surface. The gateway validates bearer tokens,
maps Keycloak principals to Forgejo accounts, applies operation policy, and then
lets Forgejo remain the final authority for repository and organization access.

The project includes:

- Keycloak OIDC discovery, JWKS, issuer, audience, and expiry validation.
- Explicit Keycloak subject to Forgejo account mapping.
- Bounded MCP tools for repository metadata, issues, pull requests, reviews,
  releases, notifications, and generated API coverage metadata.
- Approval-gated high-risk operations such as pull-request merge and release
  creation.
- Token-safe audit records.
- A CLI helper, `forgejo-mcpctl`.
- Crates.io-ready packaging for `cargo install forgejo-keycloak-rust-mcp --locked`.

## Canonical Links

- Source: https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp
- Wiki: https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/wiki
- Releases: https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/releases

Issues and contributions should be made on Codeberg, not GitHub.
