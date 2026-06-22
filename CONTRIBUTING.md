# CONTRIBUTING.md

## Setup
1. Read `AGENTS.md`.
2. Read `COMMANDS.md`.
3. Read `.agent/PLANS.md`.
4. Read the active ExecPlan.
5. Run `./scripts/preflight.sh`.
6. Run `./scripts/install.sh`.

## Branch Rules
- `main` is releasable.
- Feature work on short-lived branches named `ep-XXX-short-slug` (matches the active ExecPlan).
- One active ExecPlan per branch.

## Coding Standards
- Respect the layer and dependency rules in `ARCHITECTURE.md`.
- Prefer small, reversible changes.
- Follow existing patterns verified in the repo (do not invent crate/module boundaries).
- Do not invent crypto primitives, IPC message types, env vars, config keys, roles, secret types, or audit fields.
- Rust:
  - Pass `cargo clippy --workspace --all-targets -- -D warnings`.
  - Use `zeroize` for any plaintext or key material.
  - Make crypto types `pub(crate)` unless absolutely required.
- TypeScript:
  - Strict TS; no `any`.
  - UI never calls Rust modules directly; always via the Tauri IPC bridge.

## Test Requirements
- Add tests at every layer the change touches.
- Add or update an invariant enforcement test if the change touches I-1..I-8.
- Add a regression test for any bug fix.
- Update fixtures only as required.

## Documentation Requirements
- Update `COMMANDS.md` if commands change.
- Update `ARCHITECTURE.md` if boundaries change.
- Update `SECURITY.md` and `THREAT_MODEL.md` if threat coverage changes.
- Update `TRACEABILITY.md` rows for any change that advances a row.
- Update the active ExecPlan's Progress, Surprises & Discoveries, Decision Log.

## Commit Guidance
- Use Conventional Commits (e.g. `feat(builder): add reveal flow`).
- Include the ExecPlan ID in the commit footer (`EP-004`).
- Reference any ADR introduced.

## Pull Request Checklist
- Scope matches the active ExecPlan
- Tests added/updated at every layer touched
- Invariant tests added/updated if touched
- Threat-model coverage referenced if touched
- Docs updated
- No secrets, signing keys, or real SpecAnchors committed
- Diff limited to expected files or extras justified
- Validation results included in the PR description

## Code Review Checklist
- Architecture and dependency rules respected
- Trust boundaries validated
- Verifier never sees plaintext
- SpecAnchor verification not weakened
- Audit chain continuity preserved
- No hallucinated APIs, env vars, routes, or tables
- Tests are deterministic and at the right layers
- Error handling follows SPEC-006
- Observability and security requirements respected
- No remote network calls added

## Agent-Specific Contribution Rules
- One active ExecPlan only
- Do not implement directly from `ROADMAP.md`
- Do not ask for next steps unless a STOP condition applies
- Update Progress, Surprises & Discoveries, Decision Log as you work
- Compare `git diff --name-only` to expected changed files before claiming completion
- Advance TRACEABILITY.md rows when work lands
