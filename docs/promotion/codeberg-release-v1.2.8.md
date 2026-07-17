# Codeberg Release Text - v1.2.8

Title: `v1.2.8 - private vulnerability reporting`

`v1.2.8` publishes a role-address vulnerability-reporting and private-handling
process without changing the gateway's executable Forgejo capability surface.

## Highlights

- Reports go to the monitored `info@getopir.com` project role address rather
  than a personal maintainer address.
- Public issues are prohibited for undisclosed vulnerabilities.
- The policy defines acknowledgement and update targets, restricted handling,
  encrypted follow-up, coordinated disclosure, and retention.

No publicly known vulnerability was fixed in this release.

Install from crates.io:

```sh
cargo install forgejo-keycloak-rust-mcp --version 1.2.8 --locked
```

