# EP-002 Core Domain

## 1. Purpose / Big Picture
Implement VaultCore's domain in `crates/core`: secret types (eight), roles (five), lifecycle FSM, identifiers, validators, error taxonomy. Pure code, no I/O. Lock in crypto primitive ADR-0008 even though the implementation lives in EP-004.

## 2. Scope
- Types: `SecretType`, `Role`, `LifecycleState`, `SecretId` (ULID).
- FSM transitions and tests.
- Per-type metadata schemas (no payload fields).
- Validation functions per type and global validators.
- Error taxonomy types (`VaultError`, `VaultErrorCode`).
- Pure unit tests.

## 3. Non-goals
- No persistence.
- No crypto implementation (only ADR lock).
- No IPC schema (EP-004).
- No UI (EP-005).
- No auth (EP-006).

## 4. Context and Orientation
After EP-001. Use uploaded `Architecture.md` L1..L4 and SPEC-001.

## 5. Files to Read First
- `.agent/specs/SPEC-001-core-domain.md`
- `.agent/specs/SPEC-006-error-handling.md`
- `ARCHITECTURE.md`
- `DECISIONS.md` (ADR-0008)

## 6. Files to Change
- `crates/core/src/types/{secret_type.rs,role.rs,lifecycle.rs,id.rs,meta/*.rs}`
- `crates/core/src/validation/*.rs`
- `crates/core/src/error.rs`
- `crates/core/src/fsm.rs`
- `crates/core/src/lib.rs` (re-exports)
- Unit tests next to each module
- `DECISIONS.md` (lock ADR-0008)
- `TRACEABILITY.md` rows L1..L4
- This ExecPlan

## 7. Interfaces and Contracts
- Public surface of `crates/core` exposes types and validators only.
- No `pub use` of internal crypto types yet (those land in EP-004 behind sealed types).

## 8. Milestones

### Milestone 1 — Types and identifiers
- **Goal:** Define `SecretType` (8 variants), `Role` (5 variants), `LifecycleState` (8 variants), `SecretId` (ULID).
- **Files to Read:** SPEC-001.
- **Files to Change:** `crates/core/src/types/*`.
- **Exact Edits Expected:** Enums, `Display`, `Serialize/Deserialize`, exhaustive matches.
- **Validation Command:** `./scripts/typecheck.sh`
- **Expected Result:** Compiles.
- **Recovery Instruction:** If a name is ambiguous, follow SPEC-001 exactly; do not invent variants.

### Milestone 2 — Per-type metadata schemas
- **Goal:** Strict per-type `Meta` structs (e.g., `ApiKeyMeta`, `LoginMeta`, ...).
- **Files to Read:** SPEC-001 type matrix.
- **Files to Change:** `crates/core/src/types/meta/*.rs`.
- **Exact Edits Expected:** Per-type structs with required/optional fields, length limits, no free-form maps.
- **Validation Command:** `./scripts/test-unit.sh`
- **Expected Result:** Per-type validation unit tests pass.
- **Recovery Instruction:** If a type's metadata is unspecified, lock smallest-safe-choice in Decision Log and continue.

### Milestone 3 — FSM and transitions
- **Goal:** Implement `LifecycleState` FSM with all legal transitions and explicit illegal-transition errors.
- **Files to Read:** SPEC-001 FSM section.
- **Files to Change:** `crates/core/src/fsm.rs` + tests.
- **Exact Edits Expected:** `fn transition(from, to) -> Result<(), VaultError>`; transition table tests covering legal + illegal.
- **Validation Command:** `./scripts/test-unit.sh`
- **Expected Result:** All FSM tests pass.
- **Recovery Instruction:** Never add a transition not in SPEC-001.

### Milestone 4 — Error taxonomy and final hookup
- **Goal:** Implement `VaultError` with stable codes per SPEC-006; round-trip serde tests.
- **Files to Read:** SPEC-006.
- **Files to Change:** `crates/core/src/error.rs`, re-exports in `lib.rs`.
- **Exact Edits Expected:** `enum VaultErrorCode` with `VC-VAL-*`, `VC-FSM-*`, `VC-VAL-…`, ... ; `VaultError { code, field?, message }`.
- **Validation Command:** `./scripts/verify.sh`
- **Expected Result:** Verify green; TRACEABILITY rows for L1..L4 advance to IMPLEMENTED.
- **Recovery Instruction:** Bounded retry per AGENTS.md; never add a new error category without SPEC-006 update.

## 9. Concrete Steps
1. Define types and IDs.
2. Define per-type metadata.
3. Implement FSM and tests.
4. Implement error taxonomy and round-trip serde.
5. Advance TRACEABILITY rows.

## 10. Validation and Acceptance
- All unit tests pass.
- No infrastructure imports in `crates/core` (verified by `deny.toml`).
- TRACEABILITY rows for SPEC-001/SPEC-006 advance to IMPLEMENTED.

## 11. Idempotence and Recovery
- Domain code is pure; refactors are safe within plan scope.

## 12. Progress
- [x] Milestone 1 complete (`SecretType`, `Role`, `LifecycleState`, `SecretId`; `./scripts/typecheck.sh` passed)
- [x] Milestone 2 complete (strict per-type metadata schemas and validators; `./scripts/test-unit.sh` passed)
- [x] Milestone 3 complete (FSM transition table, illegal-transition errors, and exhaustive state-pair tests; `./scripts/test-unit.sh` passed)
- [x] Milestone 4 complete (`VaultError` taxonomy, ADR-0008 lock, traceability rows L1..L4; `./scripts/verify.sh` passed)

## 13. Surprises & Discoveries
- SPEC-001 requires per-type metadata but does not define exact field matrices. Implemented smallest strict searchable schemas with no payload fields and no free-form maps.
- SPEC-001 lists explicit transitions and also says `archived` and `soft_deleted` are reversible. Implemented `archived -> active` and `soft_deleted -> archived` as the minimal reversible paths.
- Serde's automatic screaming-snake conversion would serialize `OAuthApp` as `O_AUTH_APP`; variants are explicitly renamed to the SPEC-001 wire names.
- `cargo audit` still reports warning-class Tauri transitive advisories and one low esbuild dev-server advisory inherited from EP-001; `cargo deny` reports no known vulnerabilities.

## 14. Decision Log
- ADR-0008 locked at the end of this plan.
- Added `serde` for the SPEC-001 serialization contract and `ulid` for the required opaque `SecretId`; `serde_json` is test-only for round-trip tests.
- Metadata schemas use required identity fields per type plus optional searchable descriptors and shared labels; encrypted payload fields are intentionally absent from all domain metadata.
- ADR-0008 accepted on 2026-06-21; crypto implementations remain out of scope until EP-004.

## 15. Outcomes & Retrospective
- EP-002 completed locally. `./scripts/verify.sh` exits `verify: ok`.
- Implemented pure core-domain types, strict metadata schemas, global validators, lifecycle FSM, `SecretId`, and stable redaction-safe error taxonomy.
- Added TRACEABILITY rows L1..L4 with status IMPLEMENTED.
- No persistence, IPC schema, UI behavior, auth ceremony, crypto implementation, vault data access, or production deployment behavior was introduced.
- Remaining risks are the inherited dependency-audit warnings from the current Tauri/Vite scaffold; they do not block EP-002 acceptance because cargo-deny and full verify pass.
