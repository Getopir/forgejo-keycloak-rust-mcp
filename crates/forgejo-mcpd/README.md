# forgejo-keycloak-rust-mcp

Clean-room Rust MCP gateway for Forgejo with Keycloak identity and Forgejo ACL
enforcement.

Keycloak authenticates the caller. The Rust gateway authorizes the requested
operation class. Forgejo remains the final authority for repository and
organization access.

The package installs two binaries:

```text
forgejo-keycloak-rust-mcpd
forgejo-mcpctl
```

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

For an HTTPS Forgejo deployment, run the daemon with HTTPS public URLs and add
`--tls` or `--ssl` so setup fails if a public URL is accidentally left as
`http://`:

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

The local bind can remain plain HTTP when a trusted reverse proxy terminates TLS.

Source, issues, releases, and documentation are hosted on Codeberg:

https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp
