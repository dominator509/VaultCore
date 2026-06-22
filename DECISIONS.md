# DECISIONS.md

## Decision Table

| ADR ID | Title | Status | Date | Owner | Summary |
|---|---|---|---|---|---|
| ADR-0001 | Repository-local blueprint governance | Accepted | TBD at EP-000 | [OWNER_TO_SET] | Control-plane docs and ExecPlans live in-repo. |
| ADR-0002 | Smallest reversible choice default for unresolved details | Accepted | TBD | [OWNER_TO_SET] | Bounded autonomy; choose smallest reversible option and document. |
| ADR-0003 | One active ExecPlan rule | Accepted | TBD | [OWNER_TO_SET] | Prevents drift and incomplete validation. |
| ADR-0004 | Greenfield workspace structure | Accepted | TBD | [OWNER_TO_SET] | Rust workspace (`crates/*`) + TS app (`app/`) + Tauri shell. |
| ADR-0005 | Builder ↔ Verifier IPC mechanism | Proposed | TBD at EP-004 | [OWNER_TO_SET] | Unix domain sockets / Windows named pipes with Ed25519-signed framed messages and per-session nonce + monotonic counter for replay protection. |
| ADR-0006 | UI shell choice | Proposed | TBD at EP-001 | [OWNER_TO_SET] | Tauri (Rust core + system webview) chosen over Electron for smaller attack surface and tighter Rust integration. |
| ADR-0007 | Persistence engine | Accepted | 2026-06-21 | [OWNER_TO_SET] | SQLite with `rusqlite` (bundled sqlcipher off; payloads are app-encrypted blobs). |
| ADR-0008 | Crypto primitive set | Accepted | 2026-06-21 | [OWNER_TO_SET] | XChaCha20-Poly1305 AEAD, HKDF-SHA-512 derivation, Argon2id KDF, Ed25519 signatures. |
| ADR-0009 | Test runners | Proposed | TBD at EP-001 | [OWNER_TO_SET] | Rust: `cargo nextest`; TS: Vitest; E2E: Playwright. |
| ADR-0010 | No remote telemetry by default | Accepted | TBD | [OWNER_TO_SET] | Enforced by invariant I-7 and AGENTS.md STOP condition. |

## ADR Index
- `ADR-0001`: Repository-local governance
- `ADR-0002`: Smallest reversible default
- `ADR-0003`: One active ExecPlan
- `ADR-0004`: Workspace structure
- `ADR-0005`: Trinity IPC
- `ADR-0006`: UI shell
- `ADR-0007`: Persistence
- `ADR-0008`: Crypto primitives
- `ADR-0009`: Test runners
- `ADR-0010`: No remote telemetry

## Initial ADR Entries

### EP-010: Production Launch Approval Status
- **Context:** EP-010 local readiness validation passed, and GitHub Actions run `27940112544` passed the multi-OS verify gate for commit `2adc3051d8a7a511ee28804a7ff5e0b54afd8abd`; production launch still requires owner approval, production signing credentials, release SpecAnchor signing, and signed release artifact evidence.
- **Decision:** No production publish or updater-channel activation is approved in this thread. Local readiness and multi-OS verify evidence may be committed and pushed for review; launch remains NO-GO until the owner explicitly approves release-candidate entry and provides the approved release signing path.
- **Alternatives Considered:** Treat local green checks as release approval (rejected: violates EP-010 owner-approval gate and AGENTS.md production deployment STOP conditions).
- **Consequences:** The repository can carry an honest readiness packet while preventing accidental publication, signing-key leakage, or updater-channel changes.
- **Status:** Accepted for EP-010 gatekeeping

### ADR-0004: Greenfield Workspace Structure
- **Context:** Architecture.md prescribes a six-layer architecture with a clear Trinity boundary. Greenfield repo must reflect this from day one.
- **Decision:** Use a Cargo workspace at the root with `crates/core` (domain + shared types), `crates/builder` (binary, brief plaintext), `crates/verifier` (binary, no plaintext), `crates/cli` (admin/offline operations); UI under `app/` with Tauri shell.
- **Alternatives Considered:** Single binary with internal modules (rejected: violates I-4); multi-repo split (rejected: hurts traceability and CI).
- **Consequences:** Stronger process isolation; clearer dependency rules; slightly more build complexity.
- **Status:** Accepted

### ADR-0008: Crypto Primitive Set
- **Context:** Architecture.md envelope formats reference XChaCha20-Poly1305 / AES-256-GCM-SIV, HKDF-SHA-512, Argon2id, Ed25519.
- **Decision:** Default to XChaCha20-Poly1305 for payload AEAD, HKDF-SHA-512 for key derivation, Argon2id for passphrase KDF, Ed25519 for signatures.
- **Alternatives Considered:** AES-GCM (rejected: nonce-misuse risk in fast iteration); ChaCha20 without Poly1305 (rejected: needs authentication).
- **Consequences:** Locks the threat-model assumptions; any change requires a new ADR and re-verification of T-009..T-013.
- **Status:** Accepted

### ADR-0007: Persistence Engine
- **Context:** SPEC-002 defines a single local vault file with searchable metadata, opaque encrypted payload blobs, additive migrations, and a hash-chained audit log.
- **Decision:** Use SQLite via `rusqlite`; Unix builds link system SQLite through `pkg-config`, while Windows builds use bundled SQLite for self-contained Tauri packaging.
- **Alternatives Considered:** JSON files (rejected: weak constraints and query/index support); embedded key-value store (rejected: more custom schema/audit logic); SQLCipher as the primary security boundary (rejected for v1 because payloads are app-encrypted envelopes).
- **Consequences:** Strong local transactional semantics and simple backup/restore; platform build configuration must keep SQLite available without introducing remote services.
- **Status:** Accepted

### ADR-0010: No Remote Telemetry by Default
- **Context:** Invariant I-7 forbids vendor backdoors; project goal forbids cloud calls in v1.
- **Decision:** No remote network calls in default builds. Any opt-in telemetry must be off by default, locally inspectable, and gated by an explicit ADR.
- **Alternatives Considered:** Opt-out crash reports (rejected: still leaks data).
- **Consequences:** Simpler threat model; weaker remote diagnostics (user must export local logs manually).
- **Status:** Accepted

## Rules for Adding New Decisions
1. Add a new ADR for any change to architecture boundaries, crypto primitives, IPC schema, persistence engine, role/secret-type set, deployment target, or signing key handling.
2. Use the template in `.agent/templates/adr-template.md`.
3. Assign a unique ADR ID.
4. Update the decision table and ADR index.
5. Cross-link affected specs, ExecPlans, threats, and invariants.
6. Do not create ADRs for trivial local code choices.

## ADR Template Reference
Use `.agent/templates/adr-template.md`.
