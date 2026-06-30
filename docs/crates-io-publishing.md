# Crates.io Publishing

Crates.io is the public Rust package registry. Publish there so users can find
and install the gateway with Cargo:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
```

Codeberg remains the source repository, issue tracker, release-note home, and
wiki home. Crates.io is the Cargo distribution channel.

## Package Names

Crates.io has one global namespace. The workspace uses unique public package
names while keeping short internal Rust library names for source readability.

| Package | Purpose |
| --- | --- |
| `forgejo-keycloak-mcp-policy` | Operation policy registry and generated Forgejo API classification data. |
| `forgejo-keycloak-mcp-identity` | Keycloak OIDC discovery, JWKS, and bearer-token validation. |
| `forgejo-keycloak-mcp-audit` | Token-safe audit event schema. |
| `forgejo-keycloak-rust-mcp` | Installable daemon and CLI package. |

The installable binaries are:

| Binary | Purpose |
| --- | --- |
| `forgejo-keycloak-rust-mcpd` | HTTP MCP gateway daemon. |
| `forgejo-mcpctl` | Operator and agent CLI wrapper for curated MCP calls. |

## Publish Order

Publish dependencies before the binary package. The safest path is the helper
script, which checks registry state, runs validation, publishes in dependency
order, and waits for each just-published package to appear on crates.io before
publishing the packages that depend on it:

```powershell
powershell -ExecutionPolicy Bypass -File tools\publish-crates-io.ps1
powershell -ExecutionPolicy Bypass -File tools\publish-crates-io.ps1 -Execute
```

The first command is a dry-run. The second command performs the real crates.io
publication. It requires `cargo login` or `CARGO_REGISTRY_TOKEN`.
Use `pwsh -File ...` instead when running from PowerShell 7 or non-Windows
hosts.

The manual equivalent is:

```sh
cargo publish -p forgejo-keycloak-mcp-policy
cargo publish -p forgejo-keycloak-mcp-identity
cargo publish -p forgejo-keycloak-mcp-audit
cargo publish -p forgejo-keycloak-rust-mcp
```

Run dry-runs before each publish:

```sh
cargo publish --dry-run -p forgejo-keycloak-mcp-policy
cargo publish --dry-run -p forgejo-keycloak-mcp-identity
cargo publish --dry-run -p forgejo-keycloak-mcp-audit
cargo publish --dry-run -p forgejo-keycloak-rust-mcp
```

Before the first crates.io publication, `cargo publish --dry-run` for
`forgejo-keycloak-mcp-audit` and `forgejo-keycloak-rust-mcp` fails with
`no matching package named ... found` until their dependency packages are
visible in the crates.io index. That is expected for a multi-crate first
publication. The practical sequence is:

1. Dry-run `forgejo-keycloak-mcp-policy` and `forgejo-keycloak-mcp-identity`.
2. Publish `forgejo-keycloak-mcp-policy` and `forgejo-keycloak-mcp-identity`.
3. Wait until both package versions are visible on crates.io.
4. Dry-run and publish `forgejo-keycloak-mcp-audit`.
5. Wait until `forgejo-keycloak-mcp-audit` is visible on crates.io.
6. Dry-run and publish `forgejo-keycloak-rust-mcp`.

## Authentication

Create an API token in the crates.io account settings, then log in locally:

```sh
cargo login
```

Paste the token when prompted. Do not commit the token, paste it into docs, or
store it in shell history. Cargo stores the credential in the local Cargo
credential store.

For CI or a controlled release host, use a short-lived environment variable:

```sh
CARGO_REGISTRY_TOKEN=... cargo publish -p forgejo-keycloak-rust-mcp
```

## Release Checklist

Before publishing:

- Confirm the version is bumped in every workspace package.
- Run `cargo fmt --check`.
- Run `cargo test --workspace`.
- Run `powershell -ExecutionPolicy Bypass -File tools\publish-crates-io.ps1`
  or run
  `cargo publish --dry-run` for each package immediately before publishing it.
- Confirm `README.md`, `docs/install.md`, and `docs/wiki/Install.md` show the
  `cargo install forgejo-keycloak-rust-mcp --locked` path.
- Confirm no secret values are included in package files.

After publishing:

- Tag and push the matching version.
- Create the Codeberg release.
- Add the crates.io install command to the release notes.
- Verify install from a clean machine or container:

```sh
cargo install forgejo-keycloak-rust-mcp --locked
forgejo-keycloak-rust-mcpd --help
forgejo-mcpctl --help
```

## Important Crates.io Constraints

Publishing is permanent. A published version cannot be overwritten. If a
mistake is published, yank that version and publish a new patch version.

Crate names are first-come, first-served. If any planned package name is taken,
choose a new prefixed name and update dependency `package = "..."`
declarations before publishing.
