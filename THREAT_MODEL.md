# THREAT_MODEL.md

| Threat | Area | Mitigation | EP-006 Evidence |
|---|---|---|---|
| T-007 | Phishing-resistant unlock | Passkey path is the preferred local unlock ceremony; passphrase remains fallback only. | `crates/builder/src/auth/passkey.rs`; `app/tests/e2e/unlock-passkey.spec.ts`; `cargo nextest run -p vaultcore-builder auth`; `pnpm --dir app test:e2e -- unlock-passkey.spec.ts` |
| T-008 | Credential theft | Passphrase fallback uses sealed bytes and Argon2id at SPEC-005 minimum install parameters. | `crates/builder/src/auth/passphrase.rs`; `cargo nextest run -p vaultcore-builder auth::passphrase` |
| T-009 | Brute-force unlock attempts | Per-device lockout state applies exponential backoff and resets only after success. | `crates/builder/src/session.rs`; `./scripts/verify.sh` |
| T-014 | IPC message manipulation | Trinity frames are signed and rejected when signatures are invalid. | `crates/core/src/trinity.rs`; `crates/tests/invariants/tests/trinity_ipc.rs`; `./scripts/security-check.sh` |
| T-015 | IPC replay | Trinity framed messages carry monotonic counters and replayed frames are rejected. | `crates/core/src/trinity.rs`; `crates/tests/invariants/tests/trinity_ipc.rs`; `./scripts/security-check.sh` |
| T-016 | Unauthorized operation injection | Verifier policy is default-deny and tests every role-operation matrix cell. | `crates/verifier/src/policy.rs`; `cargo nextest run -p vaultcore-verifier policy` |
