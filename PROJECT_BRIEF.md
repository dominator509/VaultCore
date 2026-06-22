# Project Brief

## Project Name
VaultCore

## Problem Statement
Individuals and small teams who handle high-value secrets (API keys, OAuth app credentials, SSH keys, wallet keys, certificates, sensitive notes and blobs) need a local-first secrets vault that is provably safe-by-construction. Existing solutions either trust a single monolithic process with plaintext, depend on opaque cloud providers, or leave audit/integrity guarantees unverifiable. VaultCore solves this by enforcing a Trinity contract (SpecAnchor / Builder / Verifier) that splits cryptographic capability from policy and audit, so no single process can both decrypt secrets and silently mutate state.

## Product Goal
Deliver a local-first, security-first secrets vault whose architectural invariants (I-1 through I-8) are enforced in code and verified by tests at every layer, with no plaintext at rest, just-in-time decryption, signed configuration, countersigned writes, and a fully auditable history.

## Target Users
- Solo developers and security-aware individuals managing personal credentials
- Small engineering and ops teams sharing a tightly scoped set of secrets
- Auditors who need to inspect an append-only history without ever seeing plaintext
- Power users with hardware tokens (Touch ID, Windows Hello, FIDO2) who refuse cloud-only vaults

## Primary User Outcomes
- Store, retrieve, rotate, and audit secrets locally with zero plaintext at rest
- Authenticate via passkey, biometrics, or master passphrase fallback
- Search and filter secrets by metadata without exposing payloads
- Receive provable evidence (audit chain + signed SpecAnchor) that no silent mutation occurred
- Recover from device loss via verified, encrypted backup
- Operate fully offline; never required to call a remote service

## Business Goals
- Ship a defensible v1 that an external security reviewer can audit against documented invariants
- Keep the threat model (23 in-scope threats T-001..T-023) verifiable from code and tests
- Maintain a traceability matrix from every requirement to source, test, and release gate
- Avoid feature drift: only Owner/Admin/Editor/Viewer/Auditor roles and only the eight defined secret types in v1

## Technical Goals
- Enforce six-layer architecture (L1 Experience, L2 Session/Device, L3 Identity & AuthN/AuthZ, L4 Secret Lifecycle, L5 Crypto, L6 Persistence/Audit/Backup) with one-way import rules
- Maintain Trinity process boundary: Builder (Rust, brief plaintext) and Verifier (Rust, never sees plaintext) communicate only via signed messages and a signed SpecAnchor
- Use vetted crypto only: XChaCha20-Poly1305 or AES-256-GCM-SIV for payloads, HKDF-SHA-512 for derivation, Argon2id for passphrase KDF, Ed25519 for signatures
- All UI in TypeScript, all crypto and policy in Rust; no plaintext ever crosses the TS/Rust boundary except via a single, audited "reveal" channel
- Every write requires Verifier countersignature (invariant I-5)
- Every action emits an append-only, hash-chained audit entry (invariant I-8)

## Existing Repository Status
Greenfield repository. Architecture, threat model, and traceability matrix are pre-specified (Architecture.md, THREAT_MODEL.md, TRACEABILITY.md). All source code, tests, scripts, and CI must be created from scratch following this blueprint.

## Preferred Tech Stack
- Frontend / UI: TypeScript + React (Tauri shell for desktop integration)
- Backend / Engine: Rust 2021 edition (Builder process, Verifier process, shared core crates)
- Database / Persistence: SQLite (encrypted payloads as opaque blobs; searchable metadata in plaintext columns only as defined by SPEC-002)
- Authentication: WebAuthn passkeys (preferred), platform biometrics (Touch ID / Windows Hello), Argon2id-derived master passphrase fallback
- Hosting / Deployment: Locally installed desktop application (Tauri bundle). No server component in v1.
- Testing: Rust `cargo test` + `cargo nextest` for Builder/Verifier; MSTest-equivalent Vitest for TypeScript; Playwright for E2E acceptance
- Package Manager: `cargo` (Rust), `pnpm` (TypeScript)
- CI/CD: GitHub Actions with reproducible builds, signed artifacts, gate-based promotion
- Observability: Structured logs (redacted), local metrics counters, no remote telemetry by default

## Constraints

### Business Constraints
- v1 must be auditable by an external reviewer against Architecture.md and THREAT_MODEL.md
- No telemetry that leaves the device by default
- Open verification: every release gate (G1-A..G3-E) must be reproducible from the repository

### Technical Constraints
- Plaintext secret material may exist only inside the Builder process and only for the minimal duration required for a user-initiated operation (I-1, I-2)
- The Verifier process must never receive plaintext payloads (I-4)
- All cross-process messages must be signed and replay-protected
- SpecAnchor must be signed and verified on every Builder/Verifier startup (I-6)
- No vendor backdoor key, no escrow, no remote unlock (I-7)

### Security / Compliance Constraints
- Must mitigate or document residual risk for all 23 in-scope threats T-001..T-023
- Must respect all five documented residual risks (R-1..R-5) with explicit user-facing acknowledgement where applicable
- No deviation from the approved crypto algorithm set without an ADR

### Performance Requirements
- Cold start to lock screen: under 1.5 seconds on a mid-range laptop
- Unlock (passkey path): under 500 ms after user gesture
- Search across 10,000 metadata records: under 200 ms
- Decrypt and reveal a single secret payload: under 100 ms after Verifier countersignature

### Accessibility Requirements
- Full keyboard navigation for every primary flow (unlock, search, reveal, copy, rotate, audit view)
- Visible focus, semantic landmarks, ARIA where required
- No color-only state communication
- Minimum WCAG 2.1 AA for v1

### Data / Privacy Requirements
- Plaintext payloads never persisted (I-1)
- Metadata stored unencrypted is restricted to fields enumerated in SPEC-002 (name, type, labels, lifecycle state, timestamps, hash-chained audit pointers)
- User can export an encrypted backup; user can purge any secret to the cryptographic erasure boundary
- No analytics, no crash reporting that leaves the device by default

### Integrations
- Platform authenticators (WebAuthn, Touch ID, Windows Hello) — local only
- OS keychain for ephemeral unlock token storage only (never long-term plaintext)
- Optional clipboard integration with auto-clear timer

## Out of Scope / Known Non-Goals
- Cloud sync, server-side storage, multi-device live sync in v1
- Browser extension in v1
- Mobile applications in v1
- Team-wide remote sharing protocol in v1 (single-device, local-export-based sharing only)
- Any vendor key escrow or "recover from us" path (I-7)
- Any secret types beyond the eight defined in SPEC-001
- Any role beyond Owner/Admin/Editor/Viewer/Auditor

## Timeline / Milestones
Driven by gates, not dates:
- G1-A..G1-F: Foundation, crypto, persistence, audit chain
- G2-A..G2-C: Identity, session, secret lifecycle
- G3-A..G3-E: UI, E2E acceptance, accessibility, threat-model verification, production readiness

## Deployment Target
Locally installed Tauri desktop application for macOS, Windows, and Linux. No server. No remote auto-update channel that is not signed and verifiable.

## Success Metrics
- 100% of threats T-001..T-023 either mitigated with linked test evidence or accepted as a documented residual risk
- 100% of TRACEABILITY.md rows reach status `IMPLEMENTED + VERIFIED + GATE PASSED`
- All invariants I-1..I-8 have at least one automated test that fails when the invariant is violated
- All eight scripts (`./scripts/verify.sh` chain) pass on a clean checkout
- `./scripts/production-readiness-check.sh` passes for the v1 release tag

## Definition of Production Readiness
VaultCore v1 is production-ready only when:
- All required specs SPEC-000..SPEC-008 are implemented and test-backed
- All ExecPlans EP-000..EP-010 are complete with updated Outcomes & Retrospective
- All eight invariants I-1..I-8 have passing enforcement tests
- All 23 in-scope threats are addressed with linked mitigations
- `./scripts/production-readiness-check.sh` exits 0
- Deployment, rollback, observability, and incident-response runbooks exist
- Residual risks R-1..R-5 are documented in user-visible form where applicable
