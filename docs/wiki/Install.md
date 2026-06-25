# Install

Build:

```sh
cargo test --workspace
cargo build --release -p forgejo-mcpd
```

Run:

```sh
forgejo-mcpd \
  --issuer https://keycloak.example.org/realms/forgejo-agents \
  --discovery-url https://keycloak.example.org/realms/forgejo-agents/.well-known/openid-configuration \
  --audience forgejo-mcp \
  --resource https://mcp.example.org/mcp \
  --bind 127.0.0.1:7080
```
