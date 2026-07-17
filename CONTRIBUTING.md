# Contributing

Contributions from people and AI-assisted coding workflows are welcome.

## Make A Change

1. Open an issue or describe the problem before starting a large change.
2. Keep the change focused and add tests when behavior changes.
3. Update the documentation and `docs/wiki` when setup or behavior changes.
4. Run the checks below and open a pull request explaining what changed and
   how it was verified.

```sh
cargo fmt --check
cargo test --workspace
reuse lint
```

AI-generated or AI-assisted changes are accepted. The contributor remains
responsible for reviewing the code, confirming its license and origin, running
the tests, and describing the verification in the pull request.

## Project Rules

- Keep the implementation clean-room; do not copy code from incompatible
  projects.
- Never commit secrets, tokens, private keys, internal addresses, or production
  credentials.
- Preserve bounded outputs, explicit authorization, and approval requirements.
- Target one semantic operation per `2.x.0` feature release. Each operation must
  define typed schemas, scope, bounds, audit behavior, approval policy, Forgejo
  ACL enforcement, tests, documentation, and release evidence.
- Use a patch release only for a compatible repair to an existing minor line.
- Do not begin the next planned operation until the previous hosted release,
  wiki, crates.io publication, deployment, and readback are complete.
- Report security issues privately using [SECURITY.md](SECURITY.md).

By contributing, you agree to license your contribution under
`AGPL-3.0-or-later`.
