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
  --bind 127.0.0.1:7080 \
  --forgejo-url https://forgejo.example.org \
  --principal-map /etc/forgejo-mcpd/principals.json \
  --max-page-limit 50
```

Keep Forgejo token values in runtime environment variables named by the principal map. Do not store token values in the map or in source control.
