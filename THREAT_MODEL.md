# THREAT_MODEL.md

| Threat | Area | Mitigation | Evidence / Residual Risk |
|---|---|---|---|
| T-001 | Local malware reads vault file | Payloads are stored as opaque encrypted envelopes, with no plaintext marker at rest. | `crates/tests/invariants/tests/invariants.rs::i1_no_plaintext_payload_marker_at_rest`; `cargo nextest run --test invariants` |
| T-002 | Local process scrapes JIT payloads | Builder reveal returns a payload handle with bounded TTL, not plaintext payload bytes. | `crates/tests/invariants/tests/invariants.rs::i2_builder_reveal_returns_jit_payload_handle_not_plaintext`; `app/tests/e2e/reveal.spec.ts` |
| T-003 | Metadata/payload confusion leaks ciphertext controls | Payload columns are separate from indexed metadata columns. | `crates/tests/invariants/tests/invariants.rs::i3_metadata_indexes_do_not_include_payload_columns` |
| T-004 | Unauthorized local write path | Builder write attempts require Verifier authorization before audit/write intent. | `crates/tests/invariants/tests/invariants.rs::i5_builder_write_requires_verifier_authorization`; `crates/tests/invariants/tests/builder_service.rs` |
| T-005 | Local database tampering | FSM and persistence tests reject illegal lifecycle transitions and audit tampering. | `crates/core/tests/persistence_repos.rs`; `cargo nextest run --test invariants` |
| T-006 | Real user vault data in tests | EP-007 fixtures are deterministic synthetic data only. | R-1 accepted until `scripts/generate-test-vaults.sh` lands in EP-007 Milestone 3 |
| T-007 | Phishing-resistant unlock | Passkey path is the preferred local unlock ceremony; passphrase remains fallback only. | `crates/builder/src/auth/passkey.rs`; `app/tests/e2e/unlock-passkey.spec.ts`; `cargo nextest run -p vaultcore-builder auth`; `pnpm --dir app test:e2e -- unlock-passkey.spec.ts` |
| T-008 | Credential theft | Passphrase fallback uses sealed bytes and Argon2id at SPEC-005 minimum install parameters. | `crates/builder/src/auth/passphrase.rs`; `cargo nextest run -p vaultcore-builder auth::passphrase` |
| T-009 | Brute-force unlock attempts | Per-device lockout state applies exponential backoff and resets only after success. | `crates/builder/src/session.rs`; `cargo nextest run -p vaultcore-builder session` |
| T-010 | Crypto primitive downgrade | ADR-0008 primitives are locked to Argon2id, HKDF-SHA-512, XChaCha20-Poly1305, and Ed25519. | `crates/core/src/crypto/*`; `cargo nextest run -p vaultcore-core crypto` |
| T-011 | Key material logged or debug-printed | Sealed key and payload wrappers redact `Debug` output and zeroize on drop. | `crates/core/src/crypto/sealed.rs`; `crates/core/src/crypto/aead.rs`; `./scripts/security-check.sh` |
| T-012 | SpecAnchor tampering | Signed SpecAnchor verification rejects modified payloads. | `crates/tests/invariants/tests/invariants.rs::i6_specanchor_signature_rejects_tampering` |
| T-013 | Dependency supply-chain downgrade | `cargo deny`, `cargo audit`, and `pnpm audit` are part of security and verification gates. | `scripts/security-check.sh`; `scripts/dependency-audit.sh`; `deny.toml` |
| T-014 | IPC message manipulation | Trinity frames are signed and rejected when signatures are invalid. | `crates/core/src/trinity.rs`; `crates/tests/invariants/tests/trinity_ipc.rs`; `./scripts/security-check.sh` |
| T-015 | IPC replay | Trinity framed messages carry monotonic counters and replayed frames are rejected. | `crates/core/src/trinity.rs`; `crates/tests/invariants/tests/trinity_ipc.rs`; `./scripts/security-check.sh` |
| T-016 | Unauthorized operation injection | Verifier policy is default-deny and tests every role-operation matrix cell. | `crates/verifier/src/policy.rs`; `cargo nextest run -p vaultcore-verifier policy` |
| T-017 | Audit chain tampering | Audit hash-chain verification rejects modified entries. | `crates/tests/invariants/tests/invariants.rs::i8_audit_chain_detects_missing_or_tampered_entries`; `crates/core/tests/persistence_repos.rs` |
| T-018 | Silent write without audit | Builder write path emits authorization and append-audit requests. | `crates/tests/invariants/tests/invariants.rs::i8_write_path_emits_audit_entry_after_authorization` |
| T-019 | Log injection or sensitive log fields | Stable error payloads and UI messages are redaction-safe. | `crates/core/src/error.rs`; `app/tests/unit/ui-state.test.ts`; R-2 accepted for richer log redaction filters until EP-008 |
| T-020 | Backup blob disclosure | Backups are not implemented before EP-009; any backup/export path remains out of runtime scope. | R-3 accepted until deployment/release backup handling in EP-009 |
| T-021 | Export or migration destroys audit continuity | Additive migrations preserve audit-chain continuity; destructive migrations require approval. | `crates/core/tests/persistence_repos.rs::migration_continuity_preserves_audit_chain`; R-4 for future export UX |
| T-022 | Timing/performance side channel | Current gates assert functional auth and reveal behavior; formal side-channel testing is not implemented. | R-5 accepted until production readiness budgets in EP-010 |
| T-023 | UI or telemetry side channel | Runtime sources are statically checked for remote unlock/backdoor call markers; telemetry remains disabled by scope. | `crates/tests/invariants/tests/invariants.rs::i7_runtime_sources_do_not_contain_remote_unlock_or_escrow_calls`; `SECURITY.md` |
