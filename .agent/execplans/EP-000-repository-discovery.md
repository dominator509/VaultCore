# EP-000 Repository Discovery

## 1. Purpose / Big Picture
Confirm the greenfield state of the VaultCore repository, inventory any preexisting tooling, and lock in the workspace plan, package managers, test runners, CI, and command surface so later ExecPlans can implement without guessing.

## 2. Scope
- Inventory repository root (manifests, lockfiles, CI workflows, env templates).
- Confirm no prior `crates/`, `app/`, or `src/` exists (greenfield assumption A-001).
- Detect platform tooling availability (`rustup`, `pnpm`, Tauri prerequisites).
- Update `COMMANDS.md`, `ARCHITECTURE.md`, `ASSUMPTIONS.md` from evidence.
- Record initial ADRs that became evidence-confirmed.

## 3. Non-goals
- No feature implementation.
- No new dependencies beyond verifying tooling presence.
- No production deployment.

## 4. Context and Orientation
Greenfield repo. Architecture, threat model, and traceability are pre-specified in uploaded docs and mirrored under `docs/`. This ExecPlan establishes ground truth before EP-001 lays foundation.

## 5. Files to Read First
- `AGENTS.md`
- `COMMANDS.md`
- `PROJECT_BRIEF.md`
- `ASSUMPTIONS.md`
- `ARCHITECTURE.md` (this repo) and uploaded `Architecture.md` reference
- `THREAT_MODEL.md` and `TRACEABILITY.md`

## 6. Files to Change
- `COMMANDS.md` (confirm/replace placeholders with verified commands)
- `ARCHITECTURE.md` (confirm repo map matches greenfield reality)
- `ASSUMPTIONS.md` (mark verified/unverified)
- `DECISIONS.md` (record ADR-0004..ADR-0010 status)
- This ExecPlan

## 7. Interfaces and Contracts
- No new user-visible behavior.
- Output: evidence-backed canonical docs.

## 8. Milestones

### Milestone 1 — Confirm greenfield state and inventory
- **Goal:** Confirm no source code yet exists; capture tooling presence.
- **Files to Read:** repo root, `Cargo.toml` (must not exist yet), `app/package.json` (must not exist yet), CI workflows under `.github/workflows/` (if any), `docs/`.
- **Files to Change:** This ExecPlan (Progress + Discoveries).
- **Exact Edits Expected:** Record `rustup` version, `pnpm` version, `node` version, OS, Tauri prereqs result.
- **Validation Command:** `./scripts/preflight.sh`
- **Expected Result:** Either passes (scripts present) or fails clearly listing missing items; nothing destructive.
- **Recovery Instruction:** If `preflight.sh` is missing, restore from this blueprint pack before proceeding.

### Milestone 2 — Verify commands and update COMMANDS.md
- **Goal:** Replace any placeholders with verified commands.
- **Files to Read:** `COMMANDS.md`, any CI workflows present.
- **Files to Change:** `COMMANDS.md`, this ExecPlan.
- **Exact Edits Expected:** Confirm `cargo`, `pnpm`, `cargo nextest`, `cargo deny`, `cargo audit`, Tauri commands; mark any that require install.
- **Validation Command:** `./scripts/preflight.sh`
- **Expected Result:** Preflight passes with required tooling available.
- **Recovery Instruction:** If a tool is missing, document and STOP only if EP-001 cannot proceed without it.

### Milestone 3 — Confirm architecture and traceability
- **Goal:** Confirm `ARCHITECTURE.md` repo map matches the greenfield plan (no preexisting code).
- **Files to Read:** `ARCHITECTURE.md`, uploaded `Architecture.md`, `TRACEABILITY.md`.
- **Files to Change:** `ARCHITECTURE.md` only if a mismatch exists, this ExecPlan.
- **Exact Edits Expected:** Confirm planned layout: `crates/{core,builder,verifier,cli,tests/invariants}`, `app/`, `docs/`.
- **Validation Command:** `git diff --name-only`
- **Expected Result:** Either no changes (architecture matches) or minimal corrections.
- **Recovery Instruction:** Do not invent layout; defer to uploaded `Architecture.md`.

### Milestone 4 — Update ASSUMPTIONS and DECISIONS
- **Goal:** Promote assumptions to verified where evidence supports; record ADR-0004..0010 status.
- **Files to Read:** `ASSUMPTIONS.md`, `DECISIONS.md`.
- **Files to Change:** `ASSUMPTIONS.md`, `DECISIONS.md`, this ExecPlan.
- **Exact Edits Expected:** Mark A-001..A-015 as Verified / Pending; promote ADRs from Proposed to Accepted where verified.
- **Validation Command:** `./scripts/verify.sh`
- **Expected Result:** Verify exits 0 in its no-op state (no source code yet); failures must be due to missing tooling only.
- **Recovery Instruction:** If verify fails for reasons other than empty workspace, STOP and document.

## 9. Concrete Steps
1. Run preflight.
2. Inventory repository.
3. Confirm tooling.
4. Update COMMANDS.md and ARCHITECTURE.md from evidence.
5. Promote assumptions and ADRs.
6. Update progress.

## 10. Validation and Acceptance
- Preflight passes.
- COMMANDS.md and ARCHITECTURE.md reflect repo reality.
- ASSUMPTIONS.md and DECISIONS.md reflect verified state.

## 11. Idempotence and Recovery
- Discovery is doc-only; safe to rerun.
- Do not modify source code in this plan.

## 12. Progress
- [ ] Milestone 1 complete
- [ ] Milestone 2 complete
- [ ] Milestone 3 complete
- [ ] Milestone 4 complete

## 13. Surprises & Discoveries
- None yet.

## 14. Decision Log
- ADR-0004 (workspace structure): pending confirmation.
- ADR-0009 (test runners): pending confirmation.

## 15. Outcomes & Retrospective
- Pending execution.
