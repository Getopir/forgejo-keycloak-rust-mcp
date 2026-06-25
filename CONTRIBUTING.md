# Contributing

Contributions are welcome when they keep the project clean-room, testable, and safe to publish.

## Before Opening a Change

- Do not copy implementation code from GPL Forgejo MCP projects into this repository.
- Keep secrets, live tokens, private keys, internal IP addresses, and production hostnames out of commits.
- Run `cargo fmt` and `cargo test --workspace`.
- Update documentation and `docs/wiki` when behavior or setup changes.
- Add tests for new policy, identity, or audit behavior.

## Commit Scope

Keep commits focused. A useful change should describe one behavior, one documentation update, or one release preparation step.

## Security Reports

Do not open public issues for suspected vulnerabilities. Follow [SECURITY.md](SECURITY.md) and contact maintainers privately first.

## License

By contributing, you agree that your contribution is provided under the repository license: `MIT OR Apache-2.0`.
