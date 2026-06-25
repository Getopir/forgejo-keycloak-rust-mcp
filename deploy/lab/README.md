# Lab VM Plan

Goal: create an isolated VM with a blank Forgejo instance and `forgejo-mcpd`, using the existing Keycloak service for token validation.

Target defaults:

- VM name: `forgejo-keycloak-mcp-lab`
- Proposed VMID: next free ID after current service VMs, normally `190`
- Proposed IP: allocate from `192.168.87.190/24` if free
- Forgejo UI: `http://<lab-ip>:3000`
- Gateway: `http://<lab-ip>:7080/mcp`
- Keycloak issuer: local Keycloak realm selected during provisioning

Deployment sequence:

1. Clone a cloud-init capable Debian VM template.
2. Install Forgejo, PostgreSQL or SQLite for the lab, and `forgejo-mcpd`.
3. Configure Forgejo as private to the VM except the lab UI.
4. Configure Keycloak clients for the Forgejo UI and MCP gateway.
5. Create one human mapping and one agent service-account mapping.
6. Run acceptance probes from VM166 with `basic_agent`.

`blankvm` at VMID 103 is a Proxmox template but still has the Debian netinst ISO attached. Confirm it boots to a usable cloud-init image before cloning it for this lab. If it is not cloud-init ready, create a fresh Debian cloud image template first.
