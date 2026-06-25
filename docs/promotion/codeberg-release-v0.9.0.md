# Codeberg Release: v0.9.0

Tag: `v0.9.0`

Title: `v0.9.0 - Approval-backed pull-request merge`

Body:

```markdown
`v0.9.0` adds the first approval-backed high-risk Forgejo operation: pull-request merge.

Highlights:

- Consumes approval records before execution so approval IDs cannot be replayed.
- Requires the approving mapped principal and executing mapped principal to differ.
- Adds `merge_pull_request` dry-run preview with no Forgejo mutation.
- Executes `merge_pull_request` through the mapped executor's Forgejo token after approval validation and Forgejo ACL checks.
- Adds CLI support for `create-approval` and `merge-pull-request`.

Still disabled:

- Release publication execution.
- Repository deletion.
- Admin and destructive operations.
- Generated Forgejo API coverage.

The hosted Codeberg release object must be created separately from the pushed git tag.
```
