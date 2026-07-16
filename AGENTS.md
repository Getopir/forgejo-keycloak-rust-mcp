# Coding Agent Guide

This file applies to the whole repository. AI coding agents are welcome to
contribute through the same pull-request process as human contributors.

## Before Editing

- Read `CONTRIBUTING.md`, `README.md`, and `SECURITY.md`.
- Inspect the current code and tests before proposing a new abstraction.
- Keep changes focused on the requested behavior.
- Do not expose secrets, internal infrastructure, or production credentials.

## Project Constraints

- Preserve the clean-room implementation boundary.
- Keep Forgejo operations semantic, explicitly scoped, bounded, and audited.
- Do not enable generated, destructive, or administrator endpoints without a
  separately reviewed policy and approval design.
- Use Keycloak for identity and leave repository authorization to Forgejo.
- Update both normal documentation and `docs/wiki` when public behavior changes.
- Change vendored Forgejo API data only through the documented generation flow.

## Required Checks

Run these before submitting a pull request:

```sh
cargo fmt --check
cargo check --workspace
cargo test --workspace
reuse lint
```

In the pull request, summarize the change, tests run, remaining risks, and any
AI assistance that materially affected the implementation. The contributor is
responsible for the final code regardless of which tools helped create it.
