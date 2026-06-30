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

HTTPS Forgejo deployments should start the daemon with HTTPS public URLs and
`--tls` or `--ssl`:

```sh
forgejo-keycloak-rust-mcpd \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource https://forgejo.example.org/mcp \
  --tls \
  --forgejo-url https://forgejo.example.org \
  --principal-map /etc/forgejo-mcpd/principals.json \
  --approval-store /var/lib/forgejo-mcpd/approvals.jsonl \
  --bind 127.0.0.1:7080
```

The flag validates public URL configuration; it does not terminate TLS itself.

The source, issue tracker, release notes, and wiki remain on Codeberg. Crates.io
is only the Cargo distribution channel.

## Publish Order

Publish workspace packages in dependency order:

```powershell
powershell -ExecutionPolicy Bypass -File tools\publish-crates-io.ps1
powershell -ExecutionPolicy Bypass -File tools\publish-crates-io.ps1 -Execute
```

The first command is a dry-run. The second command performs the real crates.io
publication after `cargo login` or with `CARGO_REGISTRY_TOKEN` set.
Use `pwsh -File ...` instead when running from PowerShell 7 or non-Windows
hosts.

Manual order:

```sh
cargo publish -p forgejo-keycloak-mcp-policy
cargo publish -p forgejo-keycloak-mcp-identity
cargo publish -p forgejo-keycloak-mcp-audit
cargo publish -p forgejo-keycloak-rust-mcp
```

Run dry-runs before each publish:

```sh
cargo publish --dry-run -p forgejo-keycloak-mcp-policy
cargo publish --dry-run -p forgejo-keycloak-mcp-identity
cargo publish --dry-run -p forgejo-keycloak-mcp-audit
cargo publish --dry-run -p forgejo-keycloak-rust-mcp
```

For the first crates.io publication, dependent package dry-runs may fail until
their dependency packages have already been published. Publish in the order
shown above and wait for each dependency package to become visible in the
crates.io index before publishing the next dependent package.

## Agent Notes

- Prefer `cargo install forgejo-keycloak-rust-mcp --locked` for a stable build.
- Start the daemon with `forgejo-keycloak-rust-mcpd`.
- Use `forgejo-mcpctl` for shell-based MCP calls.
- Never put Keycloak client secrets, Forgejo tokens, or bearer tokens in prompts,
  wiki pages, issue comments, or repository files.
