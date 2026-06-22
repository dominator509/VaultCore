# SPEC-000 Product Scope

- **Status:** Draft until owner-approved
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phase 0 and all phases
- **Linked ExecPlans:** EP-000 through EP-010

## User-Visible Goal
Deliver VaultCore as a local-first, security-first secrets vault enabling target users (solo developers, small ops teams, auditors, power users) to store, retrieve, rotate, audit, and recover the eight defined secret types under five roles, without any plaintext at rest and without any remote service dependency.

## Non-Goals
- Cloud sync, server-side storage, multi-device live sync (v1)
- Browser extension (v1)
- Mobile applications (v1)
- Team-wide remote sharing protocol (v1)
- Any vendor key escrow or remote unlock path (forbidden by I-7)
- Any secret type beyond the eight enumerated
- Any role beyond Owner/Admin/Editor/Viewer/Auditor
- Telemetry that leaves the device by default

## Terms
- **Trinity:** SpecAnchor (signed config) + Builder (brief plaintext) + Verifier (no plaintext, signs writes, owns audit).
- **Invariant (I-n):** an architectural rule that has at least one enforcement test.
- **Gate (G-n):** a release gate verifiable from the repository.
- **Residual risk (R-n):** an accepted, documented limitation.

## Required Behavior
- The system must implement the six-layer architecture and the Trinity contract as defined in `ARCHITECTURE.md`.
- The system must enforce invariants I-1..I-8 in code, not just docs.
- The system must mitigate threats T-001..T-023 or explicitly accept them as residual risks R-1..R-5.
- The system must remain local-only by default and contain no remote network calls.

## Inputs
- Project brief fields
- The uploaded `Architecture.md`, `THREAT_MODEL.md`, `TRACEABILITY.md`
- Specs SPEC-001..SPEC-008

## Outputs
- Working v1 implementation
- Signed Tauri bundles per platform
- Reproducible verification evidence
- Updated TRACEABILITY.md

## Error States
- A required signing key, hardware authenticator, or platform code-signing identity is unavailable.
- An ExecPlan would introduce behavior that violates an invariant or a non-goal.
- Repository state conflicts materially with assumptions.

## Data Rules
- Data requirements derive from SPEC-002 (vault schema, audit chain).
- No data types, retention rules, or telemetry beyond what SPEC-000..SPEC-008 specify.

## Security Rules
- Follow `SECURITY.md` and `THREAT_MODEL.md`.
- STOP rather than guess on any security-sensitive undefined behavior.

## Accessibility Rules
- WCAG 2.1 AA on primary flows; full keyboard navigation; no color-only state.

## Performance Rules
- Cold start < 1.5 s, unlock < 500 ms, search < 200 ms on 10k records, reveal < 100 ms after countersignature.

## Observability Rules
- Logs, metrics, health endpoints local-only, redacted; surface audit chain head in UI.

## Required Tests
- Acceptance tests covering every primary flow.
- Invariant enforcement tests for I-1..I-8.
- Threat coverage references for T-001..T-023.

## Acceptance Criteria
- All ExecPlans EP-000..EP-010 complete.
- All gates G1-A..G3-E passed with linked evidence.
- TRACEABILITY.md rows all VERIFIED + GATE PASSED.
