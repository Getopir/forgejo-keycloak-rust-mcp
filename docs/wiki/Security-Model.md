# Security Model

Security layers:

1. Keycloak authenticates the caller.
2. The Rust gateway validates issuer, audience, signature, and expiry.
3. The policy registry checks the operation scope and risk class.
4. Forgejo ACL enforcement is the final authority for future repository operations.

The gateway rejects missing tokens, invalid tokens, wrong-audience tokens, unknown operations, and valid tokens that lack required scopes.

The maintained [Threat Model](Threat-Model.md) documents protected assets, trust boundaries, attacker profiles, controls, residual risks, operator assumptions, and review triggers.

Pull requests and pushes are scanned with a checksum-verified, pinned Gitleaks binary before Rust CI checks run. Structured audit events can be exported to an append-only JSONL file on durable storage with `FORGEJO_MCPD_AUDIT_LOG`.

See [JWKS Cache Limits And Key Rotation](JWKS-Cache-Limits-And-Key-Rotation.md) for the current startup-only JWKS lifecycle and safe signing-key rollover procedure.
