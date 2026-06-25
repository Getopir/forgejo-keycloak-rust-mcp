# Live Readback 2026-06-25

Current verified environment facts:

| System | Readback |
| --- | --- |
| Forgejo | SSH to `git@192.168.87.91:222` authenticates as `kentthoresen`; web/API port `3000` is reachable. |
| Forgejo project repo | `lepton/forgejo-keycloak-rust-mcp` exists; `HEAD` and `refs/heads/main` read back from `git ls-remote`. Rawholding org creation is blocked by missing `write:organization`. |
| Keycloak MCP host | `http://192.168.87.63:8090/q/health` returns `UP`. |
| Keycloak agent realm | VM166 password-grant token for `neutrino1-agent` has issuer `http://keycloak:8080/realms/neutrino-agents`, audience `account`, and scopes `email profile`; it does not currently include `forgejo:repo:read`. |
| OPIR-O | `http://192.168.87.56:8080/api/health` returns HTTP `200`; VMID 165 is running as `OPIR-O`. |
| VM166 Neutrino | Proxmox VMID 166 is `neutrino1`, IP `192.168.87.55`; SSH as `devops` works; `basic_agent 1.0.0` is installed. |
| VM166 gateway run | `/home/devops/projects/forgejo-keycloak-rust-mcp` is cloned from Forgejo and synced to `origin/main`; `cargo test --workspace` passes; `forgejo-mcpd` runs on `127.0.0.1:7080` with PID file `/home/devops/.local/state/forgejo-keycloak-rust-mcp/forgejo-mcpd.pid`. |
| VM166 Keycloak probe | `POST /mcp` with VM166 Keycloak agent token authenticates `neutrino1-agent` and returns HTTP `403` because the token lacks `forgejo:repo:read`. |
| VM171 monitor | Proxmox VMID 171 is `neutrino-monitor`, IP `192.168.87.64`; health endpoint reports `status: ok`; OPIR-O M2M key is readable by `devops`. |
| Proxmox | Aurora `192.168.87.195`; VMID 103 is a stopped `blankvm` template but still has a Debian netinst ISO attached. |

Do not store or copy OPIR-O M2M keys, Forgejo tokens, Keycloak client secrets, or PATs into this repository.
