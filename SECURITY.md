# SECURITY.md

## Security Goals
- Enforce all eight invariants I-1..I-8 in code, not just docs.
- Make every architectural decision auditable from the repository.
- Prevent plaintext at rest, plaintext leaks across the Trinity boundary, silent mutations, and any vendor backdoor.
- Mitigate all 23 in-scope threats T-001..T-023; explicitly accept the five residual risks R-1..R-5.
- Default-deny on every authorization decision.

## Threat Model Summary
See uploaded `THREAT_MODEL.md` for the authoritative list. Primary threat clusters:
- Local malware and lateral processes (T-001..T-006)
- Phishing and credential theft (T-007..T-009)
- Crypto misuse, downgrade, and key compromise (T-010..T-013)
- IPC manipulation and replay (T-014..T-016)
- Audit tampering and log injection (T-017..T-019)
- Backup and export risks (T-020..T-021)
- Side channels (T-022..T-023)

Out-of-scope threats are documented in THREAT_MODEL.md and must not be silently expanded into scope without an ADR.

## Authentication Rules
- Preferred: WebAuthn passkey with platform authenticator (Touch ID, Windows Hello, FIDO2 keys).
- Secondary: platform biometrics tied to a hardware-backed key.
- Fallback: master passphrase derived via Argon2id with conservative parameters (m=64 MiB, t=3, p=1 minimum, tuned at install per-device).
- Every authenticated session has a finite lifetime and an idle timeout enforced by the Verifier.
- Lockout: exponential backoff after failed unlocks; rate limit is enforced in Verifier (Builder may not lower it).

## Authorization Rules
- Roles: Owner, Admin, Editor, Viewer, Auditor.
- Every operation declares the minimum role and is enforced in the Verifier.
- Default-deny on unknown operations.
- Auditor role can read audit log and metadata only, never payloads.
- Builder must request a countersignature from Verifier for every write (I-5). Verifier checks role + policy before signing.

## Input Validation Rules
- Every Builder entry point validates the request schema, the session, the role, and any operation-specific invariants before any crypto operation.
- Every Verifier entry point validates the message signature, freshness (replay counter), session, and policy.
- All UI inputs are validated client-side for UX and re-validated in Builder; UI validation is never trusted.

## Output Encoding Rules
- Audit log entries are encoded as canonical CBOR with a stable schema.
- Logs are JSON lines with a strict redaction filter.
- Clipboard writes are scoped to a single secret and a single auto-clear timer.

## Secret Management Rules
- Plaintext payloads exist only inside Builder, only for the minimum operation duration, and are zeroized on Drop (I-1, I-2).
- Long-term keys live in the OS keychain or a hardware-backed key store; ephemeral session keys are derived per-session via HKDF and zeroized on lock.
- No secret, signing key, or SpecAnchor ever appears in the repository.

## Dependency Security Rules
- `cargo deny` enforces an allowlist of crypto crates (`ring`, `rustcrypto/aead`, `dalek-cryptography`, `argon2`) and forbids known-bad sources.
- `cargo audit` runs in CI and on `./scripts/dependency-audit.sh`.
- TS dependencies pinned by `pnpm-lock.yaml`; `pnpm audit` runs in CI.
- No dependency may introduce a remote network call without an ADR.

## Logging Redaction Rules
Never log:
- secret payloads (any of the eight types),
- signing keys, derived keys, or nonces,
- passphrases or biometric templates,
- raw audit chain pre-images,
- WebAuthn challenge/response material,
- file paths to user vaults.

Allowed fields per log line: timestamp, level, component (`builder` / `verifier` / `ui`), operation name, secret_id (opaque), result status, duration, audit_seq.

## Data Protection Rules
- Retention: payloads are encrypted at rest; metadata is retained per SPEC-002.
- Deletion: soft delete sets lifecycle to `soft_deleted`; purge performs cryptographic erasure by destroying the per-secret data key and overwriting the ciphertext column.
- Backup: only encrypted backups; backup blob is opaque to anyone without the user's master credential.

## Production / Vault Data Rules
- Never use real vault data in tests.
- Never modify a user's vault file from CI.
- Destructive operations (purge, rekey, audit migration) require an ExecPlan-level approval gate and a documented rollback in `ROLLBACK.md`.

## Safe Migration Rules
- Migrations are additive by default.
- Destructive migrations require an ADR and a tested rollback path.
- Audit migrations preserve hash-chain continuity; a migration that breaks the chain is forbidden.

## API / IPC Security Rules
- Builder ↔ Verifier messages are framed (length-prefixed), Ed25519-signed, and replay-protected with a monotonic per-session counter.
- The Verifier rejects any message older than its current counter or with an invalid signature, and logs the rejection.
- UI ↔ Builder calls go through the Tauri IPC bridge; every call carries a session token validated by Builder.

## Rate Limiting
- Authentication attempts: exponential backoff per device.
- Reveal operations: throttled per session.
- Search: rate-limited only to prevent UI thrash; not a security control.

## File Upload Rules
- `BLOB` secret type accepts files up to a configurable size cap (default 1 MiB).
- File names are stored only in metadata; payload is encrypted as opaque bytes.
- File type is not inferred or executed.

## Security Checklist
- Inputs validated at every boundary
- Secrets externalized, never logged
- Logs redacted per redaction rules
- Auth/authz rules tested for every (role × operation) cell
- Dependency audit reviewed
- Migrations safe; audit chain continuity tested
- Env vars documented
- External integrations isolated behind adapters
- No telemetry that leaves the device by default
- Every invariant I-1..I-8 has an enforcement test

## STOP Conditions for Security-Sensitive Actions
Stop if:
- A signing key, hardware authenticator, or paid service is required but unavailable.
- A code path would introduce a remote network call (violates A-010 and project scope).
- A code path would weaken Trinity boundary, signature verification, or audit chain integrity.
- A destructive vault or audit operation lacks an approved rollback.
- A production deployment is requested without explicit authorization.
