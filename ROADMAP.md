# ROADMAP.md

> **Warning:** Do not implement directly from this file. Implementation must happen through an ExecPlan.

## Purpose
Sequence VaultCore from a greenfield repository to v1 production readiness, gated by the eight invariants I-1..I-8 and the threat coverage in `THREAT_MODEL.md`. Every phase ends at a release gate that must be verifiable from the repository.

## Phase 0: Repository Discovery and Foundation
- **Purpose:** Stand up workspace structure, tooling, scripts, and CI for a Rust + TypeScript + Tauri repository.
- **Dependencies:** None.
- **Exit Criteria:** `./scripts/verify.sh` runs end-to-end against the empty workspace; `COMMANDS.md`, `ARCHITECTURE.md`, `ASSUMPTIONS.md` reflect repo reality.
- **Gate:** G1-A (Foundation green)
- **Linked Specs:** SPEC-000
- **Linked ExecPlans:** EP-000, EP-001

## Phase 1: Core Domain
- **Purpose:** Implement domain types (Secret, SecretType enum, Role enum, LifecycleState FSM) and pure business rules in `crates/core`.
- **Dependencies:** Phase 0.
- **Exit Criteria:** Domain entities defined, FSM transitions test-backed, no infrastructure imports in `crates/core`.
- **Gate:** G1-B (Core domain frozen)
- **Linked Specs:** SPEC-001, SPEC-006
- **Linked ExecPlans:** EP-002

## Phase 2: Data and Persistence
- **Purpose:** Implement SQLite schema, additive migrations, repositories, and the hash-chained audit log.
- **Dependencies:** Phase 1.
- **Exit Criteria:** Repositories pass integration tests; audit chain test verifies tamper detection; payload columns are ciphertext-only (I-1, I-3).
- **Gate:** G1-C (Persistence + audit chain)
- **Linked Specs:** SPEC-002, SPEC-006, SPEC-007
- **Linked ExecPlans:** EP-003

## Phase 3: Crypto and Trinity IPC (Builder + Verifier + SpecAnchor)
- **Purpose:** Implement the L5 crypto layer and the Trinity boundary. Builder process (brief plaintext), Verifier process (no plaintext), SpecAnchor signed config.
- **Dependencies:** Phase 1 and Phase 2.
- **Exit Criteria:** Builder ↔ Verifier IPC uses signed, replay-protected messages; every write is countersigned (I-5); SpecAnchor verified at startup (I-6); enforcement tests for I-1, I-2, I-4, I-5, I-6, I-7 pass.
- **Gate:** G1-D (Crypto + Trinity) and G1-E (SpecAnchor) and G1-F (Audit immutability)
- **Linked Specs:** SPEC-001, SPEC-006
- **Linked ExecPlans:** EP-004

## Phase 4: UI / Tauri Client Layer
- **Purpose:** Implement the TypeScript + React UI inside Tauri. Critical flows: unlock, list/search, reveal, copy with auto-clear, create, rotate, audit view.
- **Dependencies:** Phase 3.
- **Exit Criteria:** Playwright E2E covers critical flows; loading/empty/error states present; accessibility baseline met (WCAG 2.1 AA).
- **Gate:** G3-A (UI acceptance)
- **Linked Specs:** SPEC-004
- **Linked ExecPlans:** EP-005

## Phase 5: Auth, Permissions, and Security Controls
- **Purpose:** WebAuthn passkey path, biometrics, Argon2id passphrase fallback. RBAC over the five roles. Trust-boundary validation.
- **Dependencies:** Phases 2–4.
- **Exit Criteria:** All auth paths test-backed; role allow/deny tests cover every protected operation; T-007 (phishing-resistant unlock) mitigated.
- **Gate:** G2-A (Identity) and G2-B (Session)
- **Linked Specs:** SPEC-005, SPEC-006
- **Linked ExecPlans:** EP-006

## Phase 6: Testing Hardening
- **Purpose:** Close coverage gaps for invariants, add regression tests for every closed bug, stabilize CI.
- **Dependencies:** Earlier phases.
- **Exit Criteria:** Every invariant I-1..I-8 has at least one failing-when-violated test; verification matrix is reliable in CI.
- **Gate:** G2-C (Secret lifecycle hardening)
- **Linked Specs:** SPEC-006, SPEC-008
- **Linked ExecPlans:** EP-007

## Phase 7: Observability and Operations
- **Purpose:** Structured local logs with redaction, local metrics, health checks for Builder/Verifier, runbooks.
- **Dependencies:** Phase 3 onward.
- **Exit Criteria:** No secret material in logs; smoke test verifies health channel; runbooks for common failures and incidents.
- **Gate:** G3-B (Observability)
- **Linked Specs:** SPEC-007
- **Linked ExecPlans:** EP-008

## Phase 8: Deployment and Release
- **Purpose:** Reproducible Tauri build, code signing per platform, signed release artifacts, rollback path.
- **Dependencies:** Phases 0–7.
- **Exit Criteria:** Build artifacts are reproducible and signed; release checklist passes; rollback drill documented.
- **Gate:** G3-C (Release)
- **Linked Specs:** SPEC-008
- **Linked ExecPlans:** EP-009

## Phase 9: Production Readiness
- **Purpose:** Final launch gate: verify all invariants, all threats, all residual risks acknowledged, all docs current.
- **Dependencies:** All prior phases.
- **Exit Criteria:** `./scripts/production-readiness-check.sh` exits 0; `THREAT_MODEL.md` coverage map is complete; `TRACEABILITY.md` rows are all VERIFIED + GATE PASSED.
- **Gate:** G3-D (Threat model verification) and G3-E (Launch)
- **Linked Specs:** SPEC-008
- **Linked ExecPlans:** EP-010

## Production Readiness Milestone
Achieved only when:
- Every invariant I-1..I-8 has passing enforcement tests.
- Every threat T-001..T-023 is mitigated with linked test/code evidence or accepted as a residual risk R-1..R-5 with documented user-facing acknowledgement where applicable.
- All ExecPlans EP-000..EP-010 are complete with Outcomes & Retrospective updated.
- Release artifacts are signed and reproducible.
- Rollback path is documented and rehearsed.
