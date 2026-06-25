# Lab VM Plan

Goal: create an isolated VM with a blank Forgejo instance and `forgejo-mcpd`, using a Keycloak realm for token validation.

Target defaults:

- VM name: `forgejo-keycloak-mcp-lab`
- Proposed VM ID: next free ID in your virtualization platform.
- Proposed IP: allocate an unused address from your lab network.
- Forgejo UI: `http://<lab-ip>:3000`
- Gateway: `http://<lab-ip>:7080/mcp`
- Keycloak issuer: the lab Keycloak realm selected during provisioning

Deployment sequence:

1. Create or clone a cloud-init capable Debian VM template.
2. Install Forgejo, PostgreSQL or SQLite for the lab, and `forgejo-mcpd`.
3. Configure Forgejo as private to the VM except the lab UI.
4. Configure Keycloak clients for the Forgejo UI and MCP gateway.
5. Create one human mapping and one agent service-account mapping.
6. Run acceptance probes from an agent host.

Do not clone a production Forgejo VM for this lab. Start from a clean OS image and seed only disposable test users, repositories, and keys.
