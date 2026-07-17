# forgejo-keycloak-mcp-identity

Keycloak OIDC discovery, JWKS loading, and bearer-token validation for
`forgejo-keycloak-rust-mcp`.

This crate validates issuer, audience, expiry, and signing keys for
Keycloak-issued access tokens before the gateway evaluates Forgejo MCP policy.

Signing keys are validated at startup. RSA keys must be at least 2048 bits;
supported asymmetric combinations are RS256/384/512, PS256/384/512,
ES256 with P-256, ES384 with P-384, and EdDSA with Ed25519. Symmetric JWT keys,
missing algorithms, algorithm/key-type mismatches, weak RSA keys, duplicate key
IDs, and unsupported curves prevent startup. Encryption-only JWKS entries are
not considered token-signing keys.

The installable gateway is published as:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

Source, issues, releases, and documentation are hosted on Codeberg:

https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp
