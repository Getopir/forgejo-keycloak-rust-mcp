# Live Readback 2026-06-25

Current verified environment facts:

| System | Readback |
| --- | --- |
| Forgejo | SSH to `git@192.168.87.91:222` authenticates as `kentthoresen`; web/API port `3000` is reachable. |
| Keycloak MCP host | `http://192.168.87.63:8090/q/health` returns `UP`. |
| OPIR-O | `http://192.168.87.56:8080/api/health` returns HTTP `200`; VMID 165 is running as `OPIR-O`. |
| VM166 Neutrino | Proxmox VMID 166 is `neutrino1`, IP `192.168.87.55`; SSH as `devops` works; `basic_agent 1.0.0` is installed. |
| VM171 monitor | Proxmox VMID 171 is `neutrino-monitor`, IP `192.168.87.64`; health endpoint reports `status: ok`; OPIR-O M2M key is readable by `devops`. |
| Proxmox | Aurora `192.168.87.195`; VMID 103 is a stopped `blankvm` template but still has a Debian netinst ISO attached. |

Do not store or copy OPIR-O M2M keys, Forgejo tokens, Keycloak client secrets, or PATs into this repository.
