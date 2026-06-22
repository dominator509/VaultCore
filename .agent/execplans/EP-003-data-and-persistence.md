# EP-003 Data and Persistence

## 1. Purpose / Big Picture
Implement the SQLite schema in `crates/core::persistence`, repositories, additive migrations, and the hash-chained audit log primitives. Enforce I-1 (no plaintext at rest), I-3 (metadata vs. payload split), and I-8 (every action audited) at the schema and repository layer.

## 2. Scope
- SQLite schema per SPEC-002.
- Migrations (additive).
- Repositories: `SecretRepo`, `AuditRepo`.
- Audit chain hashing and verification primitives (no signing here; signing happens in Verifier in EP-004).
- Integration tests with temporary DB.
- Cryptographic erasure on purge (test stub; full crypto in EP-004).

## 3. Non-goals
- No AEAD payload encryption (EP-004).
- No SpecAnchor verification (EP-004).
- No IPC (EP-004).
- No UI.

## 4. Context and Orientation
After EP-002. Reads SPEC-002.

## 5. Files to Read First
- `.agent/specs/SPEC-002-data-model.md`
- `.agent/specs/SPEC-006-error-handling.md`
- `ARCHITECTURE.md`
- `crates/core/src/types/*`

## 6. Files to Change
- `crates/core/src/persistence/{schema.rs, migrations/*, repo/secret_repo.rs, repo/audit_repo.rs, audit_chain.rs}`
- `crates/core/src/lib.rs`
- Integration tests under `crates/core/tests/persistence_*.rs`
- `COMMANDS.md` (migration command if changed)
- `TRACEABILITY.md` rows for L6
- This ExecPlan

## 7. Interfaces and Contracts
- `SecretRepo::create/update/get/list/purge` returns domain types.
- `AuditRepo::append(entry) -> entry_hash` (countersignature is added by Verifier in EP-004).
- Payload columns accept opaque `Vec<u8>` only.

## 8. Milestones

### Milestone 1 — Schema and migrations
- **Goal:** Initial migration creates `secrets`, `audit_log`, `specanchor_meta`, `migrations` per SPEC-002.
- **Files to Read:** SPEC-002.
- **Files to Change:** `schema.rs`, `migrations/0001_initial.rs`.
- **Exact Edits Expected:** Table DDL with documented PRAGMAs; idempotent `apply`.
- **Validation Command:** `./scripts/test-integration.sh`
- **Expected Result:** Migration test creates DB; smoke check passes.
- **Recovery Instruction:** If a column is unclear, follow SPEC-002 exactly.

### Milestone 2 — Repositories
- **Goal:** Implement `SecretRepo` and `AuditRepo` with typed methods.
- **Files to Read:** schema, SPEC-002.
- **Files to Change:** `repo/*.rs` and tests.
- **Exact Edits Expected:** CRUD + list/search; integrity checks; FSM transitions invoked before writes.
- **Validation Command:** `./scripts/test-integration.sh`
- **Expected Result:** Repository integration tests pass against temp DB.
- **Recovery Instruction:** Never bypass FSM transition validation to make a test pass.

### Milestone 3 — Audit chain primitives
- **Goal:** Implement `audit_chain.rs` hashing helpers (no signing yet).
- **Files to Read:** SPEC-002 chain section.
- **Files to Change:** `audit_chain.rs` + tests.
- **Exact Edits Expected:** `prior_hash`, `payload_hash` (canonical CBOR), `entry_hash = H(prior_hash || payload_hash)`; chain verification function.
- **Validation Command:** `./scripts/test-integration.sh`
- **Expected Result:** Tamper test detects mutation; happy-path test verifies chain.
- **Recovery Instruction:** Never mask a tamper failure.

### Milestone 4 — Cryptographic erasure on purge (stub) and migration test
- **Goal:** Implement `purge` that NULLs payload columns and tombstones the row; add a roll-forward migration test.
- **Files to Read:** SPEC-002 retention rules.
- **Files to Change:** `repo/secret_repo.rs` purge path; new migration `0002_noop.rs` (test fixture).
- **Exact Edits Expected:** Purge sets state to `purged`, NULLs payload + dek_id; migration test ensures audit-chain continuity.
- **Validation Command:** `./scripts/verify.sh`
- **Expected Result:** Verify green; TRACEABILITY L6 rows advance.
- **Recovery Instruction:** If chain continuity test fails after a migration, STOP — do not "fix" the chain.

## 9. Concrete Steps
1. Migrations.
2. Repos.
3. Audit chain hashing.
4. Purge + migration continuity.
5. Advance TRACEABILITY.

## 10. Validation and Acceptance
- All persistence integration tests pass.
- Tamper-detection test fails when violating chain.
- I-1, I-3, I-8 enforcement tests pass.

## 11. Idempotence and Recovery
- Migrations are additive and idempotent.
- Tests use temp DBs; safe to rerun.

## 12. Progress
- [x] Milestone 1 complete (SQLite schema, PRAGMAs, initial and additive migrations; `./scripts/test-integration.sh` passed)
- [x] Milestone 2 complete (`SecretRepo` and `AuditRepo` typed methods; `./scripts/test-integration.sh` passed)
- [x] Milestone 3 complete (audit hash-chain append/verify/tamper tests; `./scripts/test-integration.sh` passed)
- [x] Milestone 4 complete (purge tombstones payload material, migration continuity test, L6 traceability rows; `./scripts/verify.sh` passed)

## 13. Surprises & Discoveries
- WSL's `cc` resolves to a Zig/clang shim that rejects the bundled SQLite build flags. `rusqlite` is target-scoped: Unix uses system SQLite via `pkg-config`, Windows keeps bundled SQLite for self-contained Tauri packaging.
- SPEC-002 requires canonical CBOR payload hashing. EP-003 uses deterministic CBOR serialization over fixed Rust structs for audit payloads; signing remains out of scope until EP-004.

## 14. Decision Log
- ADR-0007 (SQLite + rusqlite) locked at the end of this plan.
- Added `rusqlite`, `sha2`, `ciborium`, and test-only `tempfile` to `crates/core` for the EP-003 persistence/audit implementation.
- The `0002_noop` migration is an additive test fixture that proves roll-forward migration continuity without introducing schema drift beyond a marker table.
- ADR-0007 accepted on 2026-06-21.

## 15. Outcomes & Retrospective
- EP-003 completed locally. `./scripts/verify.sh` exits `verify: ok`.
- Implemented `crates/core::persistence` with schema PRAGMAs, additive migrations, typed secret and audit repositories, audit-chain hashing/verification, tamper detection, and purge tombstoning.
- Added integration tests for migration creation, no payload indexes, secret CRUD/list/update/purge, illegal FSM transition rejection, audit-chain happy path, audit tamper detection, and migration continuity.
- Advanced TRACEABILITY rows L6-I1, L6-I3, and L6-I8 to IMPLEMENTED.
- No AEAD encryption, SpecAnchor verification, IPC schema, UI behavior, production signing, vault-file migration of user data, or production deployment behavior was introduced.
- Remaining risks are inherited dependency-audit warnings from the Tauri/Vite scaffold; `cargo deny`, repository integration tests, and full verify pass.
