# Release Artifact Verification

Each maintained release publishes these signed source artifacts:

- `forgejo-keycloak-rust-mcp-<version>.tar.gz`
- `forgejo-keycloak-rust-mcp-<version>.zip`
- `forgejo-keycloak-mcp-audit.cdx.json`
- `forgejo-keycloak-mcp-identity.cdx.json`
- `forgejo-keycloak-mcp-policy.cdx.json`
- `forgejo-keycloak-rust-mcp.cdx.json`
- `SHA256SUMS`
- `SHA256SUMS.sig`

`SHA256SUMS.sig` is an SSH signature over the exact `SHA256SUMS` file. The
allowed signer is committed in `RELEASE_SIGNING_KEYS` with identity
`release@forgejo-keycloak-rust-mcp` and namespace `file`.

Verify the signature from a clean checkout:

```sh
ssh-keygen -Y verify \
  -f RELEASE_SIGNING_KEYS \
  -I release@forgejo-keycloak-rust-mcp \
  -n file \
  -s SHA256SUMS.sig < SHA256SUMS
```

The expected signer fingerprint for the `1.2.4` release is:

```text
SHA256:MtI1AAdPMX0v3uRCxqyS+yissU/8gHkmZ2sYPpPLHm8
```

After the signature succeeds, verify all downloaded artifact hashes:

```sh
sha256sum --check SHA256SUMS
```

On PowerShell, compare each line with `Get-FileHash -Algorithm SHA256`.
Verification fails if the checksum manifest, signature, signer identity,
namespace, or any release artifact has been changed.

Maintainers generate artifacts from the annotated release tag:

```powershell
powershell -ExecutionPolicy Bypass -File tools\build-release-artifacts.ps1 `
  -Version 1.2.4 `
  -SigningKey <private-key-path> `
  -OutputDirectory <empty-output-directory>
```

Only the public key belongs in the repository. Keep the private signing key in
the approved credential store or protected release environment.
