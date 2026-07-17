# Forgejo 16 Release Plan

The `1.x` line supports the pre-Forgejo-16 contract and ends at `1.3.1`. The
`2.x` line targets Forgejo `16.0.0` only.

Version `2.0.0` established and verified the Forgejo 16-only baseline. Version
`2.1.0` adds the bounded `get_branch_status` operation. Each new
semantic operation is delivered alone as a minor release beginning with
`2.1.0`. Compatible repairs use patch releases such as `2.2.1`. Work stops after
each release until source, tag, notes, artifacts, wikis, crates.io, deployment,
and readback agree.

Every new operation requires a stable name, typed schemas, least-privilege
scope, enforced output bounds, structured audit behavior, an explicit approval
decision, Forgejo ACL enforcement, tests, public documentation, and release
evidence. Read operations normally need no approval. Mutations use exact-payload,
different-principal, single-use approval unless a documented security review
establishes a safer narrower rule.

The detailed version sequence and release evidence checklist are maintained in
[`docs/forgejo-16-release-plan.md`](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/src/branch/main/docs/forgejo-16-release-plan.md).

Forgejo 16 admin-token management, artifact deletion, action-run deletion, and
ActivityPub follow endpoints remain intentionally disabled and are outside this
release train.
