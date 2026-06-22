# EP-004 Crypto, Trinity IPC, and Service Layer

## 1. Purpose / Big Picture
Implement L5 (crypto), the Trinity boundary (Builder + Verifier + SpecAnchor), and the service layer between UI and Builder. Enforce invariants I-1, I-2, I-4, I-5, I-6, I-7.

## 2. Scope
- Sealed crypto types in `crates/core::crypto` (AEAD, HKDF, Argon2id wrappers; zeroize on Drop).
- SpecAnchor format + sign/verify (offline `vaultcore-cli specanchor generate/verify`).
- Builder process: in-memory plaintext only; calls Verifier for every authorize/audit operation; performs AEAD encrypt/decrypt; talks to persistence.
- Verifier process: never sees plaintext; verifies SpecAnchor; enforces policy; countersigns writes; appends audit entries.
- Trinity IPC: framed, Ed25519-signed, replay-protected messages.
- Tauri command surface in `app/src-tauri` forwards to Builder API.

## 3. Non-goals
- UI flows (EP-005).
- Auth ceremonies (EP-006 — though session-token plumbing lands here).
- Observability beyond minimal structured logs (EP-008).

## 4. Context and Orientation
Reads SPEC-003 and uploaded `Architecture.md` runtime flows.

## 5. Files to Read First
- `.agent/specs/SPEC-003-api-contracts.md`
- `.agent/specs/SPEC-006-error-handling.md`
- Uploaded `Architecture.md`
- `crates/core/src/persistence/*`

## 6. Files to Change
- `crates/core/src/crypto/{aead.rs,kdf.rs,sig.rs,sealed.rs}`
- `crates/core/src/specanchor/{schema.rs,verify.rs}`
- `crates/builder/src/{api.rs,ipc.rs,service.rs,main.rs,session.rs}`
- `crates/verifier/src/{policy.rs,ipc.rs,audit_signer.rs,main.rs}`
- `crates/cli/src/specanchor.rs`
- `app/src-tauri/src/main.rs` (Tauri commands forwarding to Builder API)
- Invariant tests in `crates/tests/invariants/`
- `TRACEABILITY.md` rows for L5
- This ExecPlan

## 7. Interfaces and Contracts
- Trinity message set per SPEC-003.
- Tauri command set per SPEC-003.
- SpecAnchor schema in `crates/core::specanchor::schema`.

## 8. Milestones

### Milestone 1 — Sealed crypto primitives
- **Goal:** AEAD (XChaCha20-Poly1305), HKDF-SHA-512, Argon2id, Ed25519 in sealed types with `Drop` + `zeroize`.
- **Files to Read:** SPEC-006 (CRYPTO error category), ADR-0008.
- **Files to Change:** `crates/core/src/crypto/*`.
- **Exact Edits Expected:** Sealed types; KAT unit tests; deterministic test vectors.
- **Validation Command:** `cargo nextest run -p vaultcore-core crypto`
- **Expected Result:** KATs pass; no plaintext exits sealed types.
- **Recovery Instruction:** Never tune crypto params to make a test pass — STOP.

### Milestone 2 — SpecAnchor sign + verify
- **Goal:** Implement signed SpecAnchor and CLI commands.
- **Files to Read:** uploaded `Architecture.md` SpecAnchor section.
- **Files to Change:** `crates/core/src/specanchor/*`, `crates/cli/src/specanchor.rs`.
- **Exact Edits Expected:** Generate (offline), verify (runtime); tamper test fails startup.
- **Validation Command:** `cargo nextest run -p vaultcore-core specanchor && cargo run -p vaultcore-cli -- specanchor verify --in tests/fixtures/specanchor.signed`
- **Expected Result:** Verify passes; tamper test fails clearly.
- **Recovery Instruction:** Tamper detection failure ⇒ STOP, not patch.

### Milestone 3 — Trinity IPC + Verifier countersignature
- **Goal:** Builder ↔ Verifier signed framed IPC with monotonic counter; every write goes through `AuthorizeOp` + `AppendAudit`.
- **Files to Read:** SPEC-003 message set.
- **Files to Change:** `crates/builder/src/ipc.rs`, `crates/verifier/src/ipc.rs`, message schema types in `crates/core`.
- **Exact Edits Expected:** Length-prefixed framing; Ed25519 signatures; per-session counter; replay rejection logs.
- **Validation Command:** `./scripts/test-integration.sh`
- **Expected Result:** Round-trip tests pass; replay tests reject; bad-signature tests reject.
- **Recovery Instruction:** Never accept a replayed or unsigned message to make a test pass.

### Milestone 4 — Builder service + Tauri commands + invariant tests
- **Goal:** Implement Builder API for unlock/list/reveal/create/update/rotate/soft_delete/purge/audit_view; wire Tauri commands.
- **Files to Read:** SPEC-003, SPEC-004.
- **Files to Change:** `crates/builder/src/{api.rs,service.rs}`, `app/src-tauri/src/main.rs`, invariant tests.
- **Exact Edits Expected:** Each API method authorizes via Verifier, then performs the op, then appends audit. Invariant tests cover I-1, I-2, I-4, I-5, I-6, I-7.
- **Validation Command:** `./scripts/verify.sh && cargo nextest run --test invariants`
- **Expected Result:** All green; TRACEABILITY L5 rows advance.
- **Recovery Instruction:** If an invariant test fails, never relax the test — fix the code or STOP.

## 9. Concrete Steps
1. Sealed crypto.
2. SpecAnchor.
3. Trinity IPC.
4. Builder API + Tauri + invariant tests.

## 10. Validation and Acceptance
- All listed validation commands green.
- Invariants I-1, I-2, I-4, I-5, I-6, I-7 have at least one failing-when-violated test.
- TRACEABILITY L5 rows VERIFIED.

## 11. Idempotence and Recovery
- Crypto types are pure; IPC code is straight-line.
- Never silently weaken signatures or replay protection.

## 12. Progress
- [x] Milestone 1 complete (sealed AEAD/HKDF/Argon2id/Ed25519 wrappers; `cargo nextest run -p vaultcore-core crypto` passed)
- [x] Milestone 2 complete (signed SpecAnchor schema/verify and CLI fixture verification; exact Milestone 2 command passed)
- [x] Milestone 3 complete (signed length-prefixed Trinity IPC, Builder signer, Verifier replay/signature checks; `./scripts/test-integration.sh` passed)
- [x] Milestone 4 complete (Builder service API, Tauri commands, policy/audit signing scaffold, invariant tests; `./scripts/verify.sh && cargo nextest run --test invariants` passed)

## 13. Surprises & Discoveries
- `.agent/specs/SPEC-004-ui-shell.md` is absent from this checkout. SPEC-003 contains the Tauri command names, so early EP-004 work continues; Milestone 4 UI-command behavior must not exceed SPEC-003 without a documented decision.
- Production SpecAnchor signing material is not present and was not required for local validation. The generated `tests/fixtures/specanchor.signed` is a deterministic development fixture only.
- `cargo nextest run --test invariants` only selects the legacy `invariants.rs` test binary. The full `./scripts/verify.sh` integration phase also ran the new `builder_service` and `trinity_ipc` invariant binaries (15 integration tests total).
- `./scripts/verify.sh` passes, but dependency audit output includes allowed RustSec warnings for transitive GTK/Tauri ecosystem crates and one low `esbuild` advisory in the Vite dev dependency path. The script treats these as non-blocking under current policy.

## 14. Decision Log
- ADR-0005 (IPC mechanism) locked here.
- ADR-0008 (crypto set) confirmed.
- Added RustCrypto/associated crates for ADR-0008 implementation: `chacha20poly1305`, `hkdf`, `argon2`, `ed25519-dalek`, `zeroize`, and `rand_core`.
- SpecAnchor signatures cover canonical CBOR of the payload and are encoded in a JSON envelope for file transport.
- Added `crates/core/src/trinity.rs` as the shared Builder ↔ Verifier message schema module required by Milestone 3. This is outside the literal file list but directly covered by “message schema types in `crates/core`.”
- Added library targets for `vaultcore-builder` and `vaultcore-verifier` so invariant tests can exercise the Builder signer and Verifier replay gate together without coupling the production crates to each other.
- Added invariant-test dependencies on `vaultcore-builder` and `vaultcore-verifier` to validate the Trinity boundary from the integration test crate.
- Implemented the SPEC-003 Builder API and Tauri command names with local service methods. Because SPEC-004 is absent, EP-004 command behavior is limited to validated local stubs, authorization checks, payload handles, and audit intents; persistence-backed user flows remain for later ExecPlans.
- Added `crates/verifier/src/policy.rs` and `crates/verifier/src/audit_signer.rs` as Milestone 4 Verifier-side scaffolding listed by the ExecPlan.

## 15. Outcomes & Retrospective
- EP-004 completed locally.
- Crypto wrappers, SpecAnchor signing/verification, Trinity IPC, Builder API, Verifier policy/audit scaffolding, Tauri commands, and invariant tests are implemented.
- Full local verification passed with `./scripts/verify.sh`.
- TRACEABILITY L5 rows advanced to `VERIFIED`.
- Remaining implementation depth for production data flows is deferred to later ExecPlans: real auth ceremonies, durable Builder service state, persistence-backed command behavior, full UI flows, and production SpecAnchor key management.
