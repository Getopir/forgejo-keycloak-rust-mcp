# forgejo-keycloak-mcp-policy

Policy registry and generated Forgejo API classification data for
`forgejo-keycloak-rust-mcp`.

This crate defines the operation classes, required scopes, approval
requirements, and generated Forgejo API coverage metadata used by the gateway.
It keeps generated endpoints disabled until they have a reviewed semantic MCP
operation.

The installable gateway is published as:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

Source, issues, releases, and documentation are hosted on Codeberg:

https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp
