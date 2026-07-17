# Release Artifact Verification

Maintained releases provide source archives, four workspace CycloneDX JSON
documents, `SHA256SUMS`, and `SHA256SUMS.sig`. The signature protects the
checksum manifest, which in turn protects every published artifact.

From a clean source checkout, place the downloaded checksum and signature files
in the repository root and run:

```sh
ssh-keygen -Y verify \
  -f RELEASE_SIGNING_KEYS \
  -I release@forgejo-keycloak-rust-mcp \
  -n file \
  -s SHA256SUMS.sig < SHA256SUMS
```

Expected `2.0.0` signer fingerprint:

```text
SHA256:MtI1AAdPMX0v3uRCxqyS+yissU/8gHkmZ2sYPpPLHm8
```

After signature verification, run `sha256sum --check SHA256SUMS` or compare
each entry with PowerShell `Get-FileHash -Algorithm SHA256`.

The canonical public signer entry is in `RELEASE_SIGNING_KEYS`. Never accept a
replacement key supplied only alongside downloaded artifacts without checking
it against the repository and release history.
