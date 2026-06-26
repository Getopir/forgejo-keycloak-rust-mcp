# Codeberg Release Text - v1.0.1

Title: `v1.0.1 - Crates.io packaging release`

Body:

`v1.0.1` prepares the Forgejo Keycloak Rust MCP gateway for public crates.io
distribution.

Highlights:

- Adds crates.io-safe package names for all workspace crates.
- Adds explicit installed binary names:
  - `forgejo-keycloak-rust-mcpd`
  - `forgejo-mcpctl`
- Documents crates.io publishing and installation.
- Keeps the AGPL-3.0-or-later license.

After the crates.io publish step, users can install with:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

This is a packaging-only patch over `v1.0.0`.
