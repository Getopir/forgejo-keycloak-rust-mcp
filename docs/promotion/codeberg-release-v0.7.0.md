# Codeberg Release: v0.7.0

Tag: `v0.7.0`

Title: `v0.7.0 - Phase 2 resource URIs and CLI wrappers`

Body:

```markdown
`v0.7.0` completes the remaining Phase 2 repository tasks for resource identifiers and CLI wrappers.

Highlights:

- Adds stable `forgejo://...` resource URIs to bounded response summaries.
- Accepts resource URI targets for repositories and numbered issue or pull-request targets.
- Adds `forgejo-mcpctl`, an optional CLI wrapper for curated MCP operations.
- CLI commands cover gateway probe, repository metadata, issues, comments, pull requests, reviews, releases, and notifications.
- CLI bearer tokens are read from an environment variable, not from command-line arguments.
- Adds `deny.toml`, local security-check documentation, and an optional self-hosted Forgejo Actions workflow for fmt/check/test/audit/deny/SBOM.

Still pending:

- Live Phase 2 MCP test readback once `forgejo-mcpd` is reachable on the lab gateway port.
- Persistent approval store.
- Pull-request merge execution.
- Release publication execution.
- Admin/destructive execution.
- Generated Forgejo API coverage.
```
