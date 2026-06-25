# Codeberg Release: v0.6.0

Tag: `v0.6.0`

Title: `v0.6.0 - Phase 1 hardening and Phase 2 curated tools`

Body:

```markdown
`v0.6.0` hardens the Phase 1 identity boundary and adds the first Phase 2 curated Forgejo tools.

Highlights:

- Rejects duplicate principal mappings and malformed mapping entries.
- Rejects caller-supplied trusted identity headers on `/mcp`.
- Adds bounded `limit` and `cursor` support for list operations.
- Adds `list_repository_issues`.
- Adds executable additive `create_issue_comment`.
- Adds `list_pull_requests`.
- Adds `list_pull_request_reviews`.
- Adds `list_releases`.
- Adds `list_notifications`.
- Adds approval-required responses for high-risk operations that do not yet have a persistent approval record.

Not included:

- Persistent approval records.
- Pull-request merge execution.
- Release publication execution.
- Repository deletion or admin execution.
- Generated Forgejo API coverage.
```
