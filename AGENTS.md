# AGENTS.md

## 1) Mission
Implement VaultCore from a greenfield repository to v1 production readiness using one active ExecPlan at a time. Enforce the six-layer architecture and the eight invariants I-1 through I-8 in code, tests, and CI. Never compromise the Trinity process boundary between Builder and Verifier. Continue autonomously; stop only under explicit STOP conditions.

## 2) Source-of-Truth Priority
When instructions conflict, use this priority order:
1. Current user instruction
2. `AGENTS.md`
3. Active ExecPlan in `.agent/execplans/`
4. Existing repository code and tests
5. `ARCHITECTURE.md` (this repo) and the uploaded `Architecture.md` reference
6. Relevant spec in `.agent/specs/`
7. `ROADMAP.md`

The uploaded `THREAT_MODEL.md` and `TRACEABILITY.md` are authoritative for threat coverage and requirement traceability. If a lower-priority artifact conflicts with a higher-priority one, follow the higher and record the conflict in the active ExecPlan Decision Log.

## 3) Required Workflow
1. Read `AGENTS.md`.
2. Read `COMMANDS.md`.
3. Read `.agent/PLANS.md`.
4. Read the active ExecPlan.
5. Run `./scripts/preflight.sh`.
6. Complete milestones in order.
7. Validate each milestone with the exact command in the ExecPlan and `COMMANDS.md`.
8. Update the active ExecPlan Progress, Surprises & Discoveries, and Decision Log after each milestone.
9. Continue autonomously to the next milestone.
10. Stop only if a STOP condition applies.

**Do not ask the user for next steps. Proceed autonomously through the active ExecPlan unless a STOP condition applies.**

## 4) STOP Conditions
Stop only if one of the following applies:
- Missing required secret, signing key, hardware authenticator, paid service, or external account.
- Any action that may destroy user vault data, production audit chain, or the SpecAnchor.
- Legal, security, privacy, or financial judgment is required and is not already specified.
- A materially different user-visible behavior choice is not resolved by spec or architecture (for example, changing the role set or the secret-type set).
- Required tests cannot run after documented recovery attempts and narrower diagnostics.
- Production code path would introduce a remote network call, a key escrow, or any path that violates invariant I-7 (no vendor backdoor).
- Production code path would weaken or bypass the Trinity boundary (invariants I-4, I-5).
- Production deployment, irreversible audit migration, or a destructive vault operation would occur without explicit permission.
- Repository reality conflicts so strongly with the active ExecPlan that continuing would create incorrect work.

When stopping, provide:
- exact blocker,
- evidence (file path and/or terminal output),
- smallest decision needed,
- recommended default.

## 5) Anti-Drift Rules
- Work on one active ExecPlan only.
- Do not implement directly from `ROADMAP.md`.
- Do not broaden scope: only the five roles and eight secret types defined in SPEC-001 may exist.
- Do not perform broad refactors, file reorganizations, dependency swaps, styling rewrites, or unrelated cleanup unless required by the active ExecPlan.
- Only modify files listed under "Files to Change" in the active ExecPlan. Any extra file must be recorded in the Decision Log and justified.
- Respect explicit Non-goals in specs and ExecPlans (no cloud sync, no telemetry by default, no escrow).

## 6) Anti-Hallucination Rules
- Do not invent crypto primitives. Use only XChaCha20-Poly1305 (or AES-256-GCM-SIV where mandated), HKDF-SHA-512, Argon2id, Ed25519 — as locked in ADR-0008.
- Do not invent IPC message types or formats. Confirm against the Builder/Verifier message schema in the repository.
- Do not invent command names. Use commands from `COMMANDS.md`.
- Do not invent environment variables, config keys, database columns, audit fields, or route names.
- Do not invent role names beyond Owner/Admin/Editor/Viewer/Auditor.
- Do not invent secret types beyond the eight enumerated.
- Confirm names by reading repository files before use.
- If a command is missing or stale, update `COMMANDS.md` first using evidence from the repository.
- Record assumptions in `ASSUMPTIONS.md` or the active ExecPlan Decision Log.

## 7) Anti-Fixation Rules
Bounded retry budget for the same root failure:
1. First failure: inspect the exact error, form one hypothesis, make the smallest targeted fix, rerun the narrowest relevant command.
2. Second same-root failure: create or run a narrower diagnostic, isolate the failure, avoid broad rewrites.
3. Third same-root failure: stop that approach, record failed hypotheses in Surprises & Discoveries, choose a simpler implementation path, continue if safe.
4. Never patch blindly around the same error indefinitely.

For crypto and IPC failures specifically: if a signature or AEAD verification fails three times, STOP and surface the failure; do not adjust algorithm parameters to make it pass.

## 8) Dependency Rules
- Prefer crates and packages already present in the repository.
- Before adding a dependency:
  1. Confirm it is necessary.
  2. Verify existing tooling cannot provide the capability.
  3. For crypto crates, prefer audited options (`ring`, `rustcrypto/aead`, `dalek-cryptography`); record source and audit status in the Decision Log.
  4. Update install/build docs accordingly.
- Do not swap foundational dependencies (UI framework, IPC layer, crypto crate, database driver) without an ADR.

## 9) File Creation Rules
- Place files only in the layer directories defined in `ARCHITECTURE.md`.
- Builder code goes under `crates/builder/`, Verifier code under `crates/verifier/`, shared types under `crates/core/`, UI under `app/`, scripts under `scripts/`, plans under `.agent/`.
- Do not introduce new top-level directories without updating `ARCHITECTURE.md` and the active ExecPlan.
- Use the smallest reversible creation needed to satisfy the milestone.

## 10) Testing Rules
- Every feature change requires tests at the closest appropriate layer.
- Minimum expectation per feature:
  - Rust unit tests for domain and crypto logic (`cargo test` or `cargo nextest`)
  - Rust integration tests across Builder/Verifier IPC where applicable
  - TS unit tests (Vitest) for UI logic
  - Playwright acceptance tests for user-visible flows
- Every invariant I-1..I-8 must have at least one enforcement test that fails when the invariant is violated.
- Do not claim completion unless validation commands pass or an explicit STOP condition is documented.

## 11) Documentation Update Rules
Update docs whenever the implementation changes:
- commands,
- architecture boundaries,
- IPC message schema,
- envelope formats,
- environment variables,
- deployment steps,
- runbooks,
- observability behavior,
- specs or acceptance criteria,
- threat-model coverage,
- TRACEABILITY.md status,
- assumptions or decisions.

## 12) Security Rules
- Never commit secrets, signing keys, or test vault data containing real credentials.
- Never log secret payloads. Logs may include metadata fields enumerated in SPEC-002 only, and must redact anything tagged sensitive.
- Validate every input at every trust boundary (UI→Builder, Builder→Verifier, Verifier→Persistence).
- Apply least privilege to every role check; default-deny.
- Never weaken Builder/Verifier message signing or replay protection to make a test pass.
- Treat any migration of the audit chain or the SpecAnchor as security-sensitive.
- Stop if a security-sensitive action is not already specified.

## 13) Production / Vault Data Rules
- Do not touch a user's vault file without explicit permission.
- Never use a user's vault data for tests; use fixtures created in EP-007.
- Destructive vault changes (purge, rekey, migration) require an ExecPlan-level approval gate and a documented rollback.
- The SpecAnchor is immutable in production runtime; modifying it requires offline regeneration, signing, and a new release.

## 14) Definition of Done
A task or ExecPlan is done only when all are true:
- All acceptance criteria pass.
- Required validation commands pass.
- ExecPlan Progress is updated.
- Final diff is reviewed.
- Only expected files changed (or extras are justified in Decision Log).
- TRACEABILITY.md rows touched by the ExecPlan are advanced (PLANNED → IMPLEMENTED, IMPLEMENTED → VERIFIED, VERIFIED → GATE PASSED).
- Remaining risks are documented.
- Relevant docs (ARCHITECTURE.md, SECURITY.md, COMMANDS.md, runbooks) are updated.
- No unresolved STOP conditions remain.

## 15) Final Response Requirements
At the end of an ExecPlan, provide:
- ExecPlan completed.
- Changed files.
- Commands run with results.
- Acceptance criteria status.
- Decisions made and ADR IDs.
- Assumptions confirmed or revised.
- Remaining risks (including residual risks R-1..R-5 if touched).
- TRACEABILITY.md row status changes.
- Whether production-readiness criteria advanced and which gate (G1-A..G3-E) is next.
