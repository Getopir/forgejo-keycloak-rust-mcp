# Testing And Coverage

Run the complete workspace test suite with `cargo test --workspace`.

The `2.1.0` contract tests accept Forgejo `16.0.0` with optional build metadata,
reject incompatible or malformed versions, verify the public version endpoint,
map every existing semantic operation to the pinned Forgejo 16 document, and
keep all 15 newly added Forgejo endpoints disabled.

Branch-status coverage verifies typed and malformed targets, unknown input
rejection, missing-scope denial, approval-free policy, encoded branch names,
successful branch plus combined-status readback, downstream failure, timeout
inheritance, response byte caps, 50-item caps, and bounded UTF-8 fields.

Both internal Forgejo and public Codeberg CI also run strict Clippy and enforce
domain-code coverage with `cargo-llvm-cov` `0.8.7`. The `1.2.11` baseline is:

| Measure | Baseline | Required minimum |
| --- | ---: | ---: |
| Lines | 59.62% | 55% |
| Functions | 52.48% | 50% |
| Regions | 58.50% | 55% |

The measurement includes identity, policy, approval, audit, rate limiting,
principal mapping, and the complete Forgejo client/parser module. Only the
daemon and CLI process-entrypoint files are excluded from percentage
calculation; they remain compiled, strictly linted, and covered by focused
validation tests where their logic is separable.

The complete reproducible command and scope policy are maintained in
[`docs/testing.md`](https://codeberg.org/GetOpir/forgejo-keycloak-rust-mcp/src/branch/main/docs/testing.md).
