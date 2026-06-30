# forgejo-keycloak-mcp-audit

Token-safe audit event schema for `forgejo-keycloak-rust-mcp`.

This crate provides structured audit event types shared by the gateway. Audit
records are designed to describe authorization decisions and tool execution
without recording bearer tokens, client secrets, or other secret values.

The installable gateway is published as:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

Source, issues, releases, and documentation are hosted on Codeberg:

https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp
