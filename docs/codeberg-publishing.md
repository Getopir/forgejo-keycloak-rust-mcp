# Codeberg Publishing

Target namespace: `https://codeberg.org/rawholding`

Target repository: `forgejo-keycloak-rust-mcp`

Recommended description:

> Rust MCP gateway for Forgejo with Keycloak identity and policy checks.

## Repository Settings

Create the repository in Codeberg before pushing. Codeberg does not support creating repositories by pushing to a new remote.

Recommended public settings:

- Owner: `rawholding`
- Repository name: `forgejo-keycloak-rust-mcp`
- Visibility: public
- Wiki: enabled
- Issues: enabled if the maintainers can triage public support
- Releases: enabled

## Push Source

Add a Codeberg remote:

```sh
git remote add codeberg git@codeberg.org:rawholding/forgejo-keycloak-rust-mcp.git
```

Push the main branch and release tags:

```sh
git push codeberg main
git push codeberg --tags
```

## Publish Wiki

The source-controlled wiki lives in `docs/wiki`. Codeberg stores the hosted wiki in a separate Git repository named `forgejo-keycloak-rust-mcp.wiki.git`.

```sh
git clone git@codeberg.org:rawholding/forgejo-keycloak-rust-mcp.wiki.git /tmp/forgejo-keycloak-rust-mcp.wiki
cp docs/wiki/*.md /tmp/forgejo-keycloak-rust-mcp.wiki/
cd /tmp/forgejo-keycloak-rust-mcp.wiki
git add .
git commit -m "Publish project wiki"
git push origin main
```

If the hosted wiki repository uses another default branch, push to that branch and update the wiki branch setting in Codeberg.

## Release Checklist

- `cargo fmt --check`
- `cargo test --workspace`
- `git diff --check`
- Secret scan with token, private-key, and internal-hostname patterns.
- Confirm `LICENSE` and `LICENSES/AGPL-3.0-or-later.txt` are present.
- Confirm `SECURITY.md`, `CONTRIBUTING.md`, and `docs/wiki` are present.
- Create or update the Codeberg release for the current version.

## Public Data Rule

The public Codeberg repository and wiki must not contain live tokens, private keys, Keycloak client secrets, Forgejo personal access tokens, internal credentials, or production-only host details.
