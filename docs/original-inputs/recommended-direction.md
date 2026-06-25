# Recommended Direction

This repository follows a clean-room architecture direction:

- Build a clean-room Rust project rather than porting Sqcows or goern.
- Use Sqcows as an API coverage checklist.
- Use goern as an MCP design checklist.
- Add a new Keycloak identity and policy layer.
- Keep Forgejo as the final repository and organization ACL authority.

The controlling rule is:

> Keycloak authenticates. The Rust gateway authorizes the operation class. Forgejo authorizes access to the actual repository or organization.
