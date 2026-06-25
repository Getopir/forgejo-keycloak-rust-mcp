# Recommended Direction

The original user-supplied architecture brief is preserved in the Codex attachment for this run:

`C:\Users\Eier\.codex\attachments\b0d49af6-7e17-4e8d-818d-597e76b8a75c\pasted-text.txt`

This repository treats that brief as a source input. The implementation direction is:

- Build a clean-room Rust project rather than porting Sqcows or goern.
- Use Sqcows as an API coverage checklist.
- Use goern as an MCP design checklist.
- Add a new Keycloak identity and policy layer.
- Keep Forgejo as the final repository and organization ACL authority.

The controlling rule is:

> Keycloak authenticates. The Rust gateway authorizes the operation class. Forgejo authorizes access to the actual repository or organization.
