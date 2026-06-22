# ARCHITECTURE.md

## Purpose
Translate the uploaded `Architecture.md` reference into enforceable, repository-local rules that lower-tier coding agents can follow. Every rule in this file is a constraint a CI check, a test, or a script can verify.

## System Overview
VaultCore is a local-first, security-first secrets vault. Two Rust binaries (Builder and Verifier) enforce a Trinity contract with a signed read-only SpecAnchor. A TypeScript + React UI inside a Tauri shell is the only user surface. All data is local. No remote network calls in default builds.

## Repository Map
```
/
├── crates/
│   ├── core/            # L1..L4 shared domain types, FSM, errors (no I/O)
│   ├── builder/         # Builder binary: brief plaintext, crypto ops, IPC client to Verifier
│   ├── verifier/        # Verifier binary: policy, RBAC, audit, SpecAnchor verification, never sees plaintext
│   ├── cli/             # Offline admin: migrate, SpecAnchor generate/verify
│   └── tests/
│       └── invariants/  # Cross-crate invariant enforcement tests (I-1..I-8)
├── app/                 # TypeScript + React UI inside Tauri shell
│   ├── src/
│   ├── src-tauri/       # Tauri Rust glue (depends on crates/builder API)
│   └── tests/
├── scripts/             # Shell wrappers (preflight, verify, etc.)
├── .agent/              # Blueprint pack control plane (specs, ExecPlans, checklists, templates)
├── docs/                # ARCHITECTURE.md (this file), THREAT_MODEL.md, TRACEABILITY.md (mirrors of upload)
├── deny.toml            # cargo-deny config
├── Cargo.toml           # workspace
└── pnpm-workspace.yaml  # TS workspace (app + tooling)
```

## Layer Responsibilities (six layers from Architecture.md)
- **L1 Experience (UI):** TypeScript + React inside Tauri. Lives in `app/`. Owns flows, states, accessibility. Never holds plaintext beyond the moment of display; uses the auto-clear timer for clipboard/reveal.
- **L2 Session / Device:** Session tokens, idle timeout, device binding. Implemented in `crates/builder` and `crates/verifier`.
- **L3 Identity & AuthN/AuthZ:** Passkey, biometrics, passphrase fallback; RBAC over five roles. Implemented in `crates/verifier` (policy) and `crates/builder` (auth ceremony).
- **L4 Secret Lifecycle:** FSM (`draft → active → expiring_soon → expired → rotating → archived → soft_deleted → purged`), rotation, retention. Implemented in `crates/core` (pure FSM) and `crates/builder` (effects).
- **L5 Crypto:** XChaCha20-Poly1305 AEAD, HKDF-SHA-512, Argon2id, Ed25519. Implemented in `crates/builder` (with sealed key handles); pure primitives live in `crates/core::crypto`.
- **L6 Persistence / Audit / Backup:** SQLite vault, hash-chained audit log, encrypted backups. Implemented in `crates/core::persistence` and `crates/builder` (writer) and `crates/verifier` (audit append authority).

## Trinity Contract (must hold in code and tests)
- **SpecAnchor:** Signed, read-only configuration file. Carries policy version, crypto suite IDs, RBAC matrix, IPC schema version, and Ed25519 verification key for Builder ↔ Verifier messages. Loaded at startup; both Builder and Verifier verify its signature before any other action (I-6).
- **Builder:** May briefly hold plaintext to perform a user-initiated operation. Must zeroize on Drop. Cannot write to persistence without a Verifier countersignature (I-5).
- **Verifier:** Never receives plaintext payloads. Enforces RBAC, policy, audit append, and signs writes. Cannot decrypt.

## Dependency Rules (one-way, enforced by code review and a cargo-deny / static check)
- `crates/core` may not depend on `crates/builder`, `crates/verifier`, `crates/cli`, or `app/`.
- `crates/builder` may depend on `crates/core` and on its own crypto/IPC modules. It must not depend on `crates/verifier`.
- `crates/verifier` may depend on `crates/core` and on its own policy/audit modules. It must not depend on `crates/builder`.
- `crates/cli` may depend on `crates/core` only.
- `app/src-tauri` may depend on `crates/builder` (Tauri-side glue) and `crates/core` only. It must not depend on `crates/verifier`.
- TypeScript code in `app/src` may not directly import Rust modules; it must go through the Tauri IPC bridge.

## Import Rules
- No circular imports between crates or between TS modules.
- Within a crate, public surface is the smallest set required by callers; everything else is `pub(crate)`.
- Crypto primitives are exposed only through sealed types that zeroize on Drop.

## Runtime Flow
- **Startup:** UI launches Tauri shell → Tauri shell spawns Verifier → Verifier verifies SpecAnchor signature → Verifier opens audit log → Tauri shell spawns Builder → Builder verifies SpecAnchor signature → Builder establishes signed IPC channel with Verifier → UI receives "locked" state.
- **Unlock:** UI initiates auth ceremony → Builder runs WebAuthn / biometrics / passphrase KDF → Builder derives session key via HKDF-SHA-512 → Verifier validates auth proof → Verifier issues a signed session token with finite lifetime.
- **Reveal:** UI requests reveal(secret_id) → Builder loads ciphertext from persistence → Builder asks Verifier to authorize reveal → Verifier checks role + policy + audit append → Verifier signs authorization → Builder decrypts payload → Builder returns plaintext to UI via the single audited reveal channel → Plaintext is zeroized after the auto-clear timer fires.
- **Write:** Builder prepares envelope → Builder asks Verifier to countersign → Verifier checks role + policy + audit append → Verifier signs → Builder writes envelope to SQLite → Builder emits audit entry containing prior_hash + payload_hash + countersignature.

## Data Flow
- Inputs cross trust boundaries only at: UI → Builder (Tauri IPC), Builder → Verifier (signed IPC), Builder → Persistence (SQLite), Verifier → Audit log (append-only).
- Metadata fields (enumerated in SPEC-002) are plaintext columns and may be searched.
- Payload columns hold AEAD ciphertext only. No payload field is ever indexed in plaintext.

## Request / Command Flow
- Every external operation has:
  - a named entry point in `crates/builder`,
  - a validated input schema,
  - a Verifier authorization step,
  - a success output contract,
  - an error contract (taxonomy in SPEC-006),
  - logging and audit hooks,
  - tests at unit, integration, and (if user-visible) E2E layers.

## State Management Rules
- Durable state: SQLite vault file. Single source of truth.
- Session state: held in Builder and Verifier in memory; never persisted.
- UI state: derived from Builder via Tauri IPC; never authoritative on its own.

## Persistence Boundaries
- All schema and migrations live in `crates/core::persistence::migrations`.
- Domain invariants are enforced before SQL writes; SQL constraints are a defense-in-depth layer.
- Migrations are additive by default; destructive migrations require an ADR and rollback path.

## External Integration Boundaries
- Platform authenticators are accessed only via OS-provided APIs wrapped in adapter modules under `crates/builder::auth::platform`.
- OS keychain access is wrapped in an adapter under `crates/builder::keystore`.
- Clipboard access is wrapped in `crates/builder::clipboard`.

## Security Boundaries
- Trust boundaries: UI ↔ Builder, Builder ↔ Verifier, Builder ↔ Persistence, Verifier ↔ Audit log, Builder ↔ Platform authenticator.
- All boundaries validate input and enforce least privilege.
- Redaction is applied at every log emission point.

## Validation Boundaries
- UI-side validation is for UX only.
- Builder-side validation is authoritative.
- Verifier-side validation is the final policy gate.

## Error Handling Boundaries
- Error taxonomy is defined in `crates/core::error` and SPEC-006.
- Internal errors are mapped to stable user-facing errors at the Builder boundary.
- Errors never leak plaintext, signing keys, or hash-chain pre-images.

## Observability Boundaries
- Logs are emitted at Builder and Verifier boundaries with the redaction filter applied.
- Metrics are counters and histograms exposed only to a local introspection channel.
- No remote sink by default.

## Architectural Invariants (must each have an enforcement test)
- I-1: No plaintext at rest.
- I-2: Just-in-time decryption; plaintext is zeroized after use.
- I-3: Metadata is searchable; payloads are opaque ciphertext.
- I-4: Trinity process boundary; Verifier never sees plaintext.
- I-5: No Builder write without Verifier countersignature.
- I-6: SpecAnchor signature verified at Builder and Verifier startup.
- I-7: No vendor backdoor; no remote key escrow; no remote unlock path.
- I-8: Every action emits an append-only, hash-chained audit entry.

## Forbidden Changes
- Direct database access from UI.
- Plaintext payloads in Verifier.
- Plaintext payloads logged anywhere.
- Bypassing Verifier countersignature for any write.
- Disabling SpecAnchor signature verification (even in dev).
- Adding remote network calls in default builds.
- Adding new roles or new secret types in v1.

## How to Add a New Feature
1. Add or update the relevant spec (`.agent/specs/`).
2. Add or update an ExecPlan.
3. Update `ARCHITECTURE.md` if boundaries change.
4. Update `THREAT_MODEL.md` if a new threat is enabled.
5. Implement smallest reversible path; keep crate boundaries clean.
6. Add tests at every layer the feature touches.
7. Advance affected TRACEABILITY.md rows.
8. Validate; review diff against expected changed files.

## How to Add a New Dependency
1. Confirm necessity.
2. Confirm no existing crate/package can do the job.
3. For crypto crates, prefer the audited allowlist; otherwise an ADR is required.
4. Update `Cargo.toml` / `package.json` and lockfiles; update `deny.toml` if needed.
5. Record reason in the Decision Log and add an ADR if the choice affects architecture.

## How to Modify Data Schema
1. Update SPEC-002.
2. Add an additive migration under `crates/core/src/persistence/migrations/`.
3. Document forward and rollback paths in `ROLLBACK.md`.
4. Add integration tests and an audit-chain continuity test if applicable.
5. Update `DEPLOYMENT.md` and `OPERATIONS.md`.

## How to Add a New Integration
1. Update PROJECT_BRIEF.md and the relevant spec.
2. Create an adapter under the appropriate crate boundary.
3. Document config/secrets in `ENVIRONMENT.md`.
4. Define failure handling, timeouts, and retries.
5. Add observability and smoke tests.

## Architecture Review Checklist
- Layer responsibilities respected
- One-way dependency rules respected
- Trust boundaries validated
- Verifier never sees plaintext
- SpecAnchor verified at startup
- Audit chain continuity preserved
- No new remote network calls
- Redaction applied at every log site
- Invariant enforcement tests added for any touched invariant
- TRACEABILITY.md advanced
