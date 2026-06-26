# Live Readback - 2026-06-25

## Codeberg Release

Hosted Codeberg release object is present for `v1.0.0`.

- API readback: `https://codeberg.org/api/v1/repos/rawholding/forgejo-keycloak-rust-mcp/releases/tags/v1.0.0`
- HTML URL: `https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/releases/tag/v1.0.0`
- Tag: `v1.0.0`
- Title: `v1.0.0 - Stable Forgejo Keycloak Rust MCP gateway`
- Draft: `false`
- Prerelease: `false`

The temporary Codeberg API token used for release creation was stored in OpenBao at:

```text
kv/data/prod/codeberg/rawholding/forgejo-keycloak-rust-mcp/release-token
```

Do not copy the token value into repository files or operational notes.

## Live Forgejo MCP Gateway

Target host:

- VMID: `118`
- Hostname: `svc-forgejo`
- IP: `192.168.87.91`
- Forgejo UI: `http://192.168.87.91:3000`
- Rust MCP gateway: `http://192.168.87.91:7080/mcp`

Installed binaries:

```text
/usr/local/bin/forgejo-keycloak-rust-mcpd
/usr/local/bin/forgejo-keycloak-rust-mcpctl
```

Systemd unit:

```text
forgejo-keycloak-rust-mcpd.service
```

Runtime configuration:

```text
/etc/forgejo-keycloak-rust-mcpd/env
/var/lib/forgejo-keycloak-rust-mcpd/approvals.jsonl
```

Readbacks:

- Deployed checkout on VMID `118`: `0d12c82939ee10f19d6c66e9184aed7d847bab0b`.
- Deployed tag on VMID `118`: `v1.0.0`.
- `GET http://192.168.87.91:7080/health` returned `{"service":"forgejo-mcpd","status":"ok"}`.
- `GET http://192.168.87.91:7080/.well-known/oauth-protected-resource` returned the MCP resource metadata.
- `POST http://192.168.87.91:7080/mcp` with valid JSON and no bearer token returned `401`, as expected.
- `systemctl is-active forgejo-keycloak-rust-mcpd.service` returned `active`.
- `/usr/local/bin/forgejo-keycloak-rust-mcpctl --help` lists `api-coverage`.
- Listener readback showed `0.0.0.0:7080`.
- Hosted Forgejo wiki head: `ea84d651a8ea832093726a96a16e6597928de50e`.
- Hosted Codeberg wiki head: `c0203bcce69652557a861095494d7eac849a52aa`.

Authenticated MCP readback:

- Keycloak client `forgejo-mcp-live-agent` issues a service-account token with issuer `http://keycloak:8080/realms/master`, audience `mcp-server`, and scope `forgejo:repo:read`.
- The gateway principal map binds that Keycloak subject to Forgejo account `kentthoresen`; the map stores an environment-variable name only, not the Forgejo token value.
- `gateway_probe` with `requested_operation=list_repository_metadata` returned `200`.
- `list_repository_metadata` for `rawholding/forgejo-keycloak-rust-mcp` returned `200` and reported Forgejo repository `Rawholding/forgejo-keycloak-rust-mcp`.
- `forgejo_api_coverage` with `query=repository` and `limit=3` returned `200`.
- `list_repository_issues` returned `403` with reason `missing required scope forgejo:issue:read`, confirming scope enforcement.

Follow-up PR auth readback on 2026-06-26:

- VM166 `neutrino1` proved the agent-side Keycloak -> OpenBao login path, but
  its OpenBao policy denied production MCP secret paths. That is the expected
  least-privilege result for the current agent role.
- A VM166-scoped OpenBao handle was then provisioned for
  `forgejo-keycloak-rust-mcp`. The handle contains the Keycloak MCP client
  metadata and secret inside OpenBao only; no secret value was printed or added
  to source control.
- Keycloak client `forgejo-mcp-live-agent` now has optional
  `forgejo:pr:write` scope in addition to default `forgejo:repo:read`.
- Keycloak CT `102` was restarted from the `services` Proxmox node to load the
  updated client-scope relation.
- A fresh service-account token requested with `forgejo:repo:read
  forgejo:pr:write` contained issuer `http://keycloak:8080/realms/master`,
  audience `mcp-server`, and both Forgejo scopes.
- Live `gateway_probe` for `create_pull_request` returned `200`.
- Live `create_pull_request` dry-run returned `200` with `allowed=true` and
  `approval_required=true`; no pull request was created.
- VM166 read its OpenBao handle, minted a short-lived MCP bearer token, and
  repeated the live `create_pull_request` dry-run successfully from the agent
  path.
- Full mutating PR creation still needs a second mapped principal because the
  approval system rejects same-principal approval and execution.

Notes:

- The older Go `forgejo-mcp.service` was stopped and disabled because it exposed a Forgejo token in the process command line. Port `8090` is no longer listening.
- The Rust gateway now uses the live Keycloak issuer from discovery: `http://keycloak:8080/realms/master`.
- Forgejo-backed read access was tested with the mapped Keycloak service account. Additional write, merge, and release tests require separate scoped Keycloak clients and explicit approval records.
