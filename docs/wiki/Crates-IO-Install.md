# Crates.io Install

Crates.io is the Rust package registry. Publish the gateway there so users can
find it through Cargo and install it with one command.

After publication:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

This installs:

```text
forgejo-keycloak-rust-mcpd
forgejo-mcpctl
```

The source, issue tracker, release notes, and wiki remain on Codeberg. Crates.io
is only the Cargo distribution channel.

## Publish Order

Publish workspace packages in dependency order:

```sh
cargo publish -p forgejo-keycloak-mcp-policy
cargo publish -p forgejo-keycloak-mcp-identity
cargo publish -p forgejo-keycloak-mcp-audit
cargo publish -p forgejo-keycloak-rust-mcp
```

Run dry-runs first:

```sh
cargo publish --dry-run -p forgejo-keycloak-mcp-policy
cargo publish --dry-run -p forgejo-keycloak-mcp-identity
cargo publish --dry-run -p forgejo-keycloak-mcp-audit
cargo publish --dry-run -p forgejo-keycloak-rust-mcp
```

For the first crates.io publication, dependent package dry-runs may fail until
their dependency packages have already been published. Publish in the order
shown above.

## Agent Notes

- Prefer `cargo install forgejo-keycloak-rust-mcp --locked` for a stable build.
- Start the daemon with `forgejo-keycloak-rust-mcpd`.
- Use `forgejo-mcpctl` for shell-based MCP calls.
- Never put Keycloak client secrets, Forgejo tokens, or bearer tokens in prompts,
  wiki pages, issue comments, or repository files.
