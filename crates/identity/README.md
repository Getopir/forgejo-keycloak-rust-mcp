# forgejo-keycloak-mcp-identity

Keycloak OIDC discovery, JWKS loading, and bearer-token validation for
`forgejo-keycloak-rust-mcp`.

This crate validates issuer, audience, expiry, and signing keys for
Keycloak-issued access tokens before the gateway evaluates Forgejo MCP policy.

The installable gateway is published as:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

Source, issues, releases, and documentation are hosted on Codeberg:

https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp
