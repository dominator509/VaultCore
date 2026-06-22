# .agent/PLANS.md

An ExecPlan is a self-contained implementation document for one VaultCore feature or system change. A new coding agent with no prior conversation must be able to continue from the ExecPlan alone.

## Required Sections for Every ExecPlan
1. Purpose / Big Picture
2. Scope
3. Non-goals
4. Context and Orientation
5. Files to Read First
6. Files to Change
7. Interfaces and Contracts
8. Milestones
9. Concrete Steps
10. Validation and Acceptance
11. Idempotence and Recovery
12. Progress
13. Surprises & Discoveries
14. Decision Log
15. Outcomes & Retrospective

## Execution Rules
- Only one ExecPlan may be active at a time.
- Read `AGENTS.md`, `COMMANDS.md`, this file, and the ExecPlan before editing.
- Run `./scripts/preflight.sh` before milestone work.
- Complete milestones in order.
- Validate after every milestone with the exact command listed.
- Continue autonomously unless a STOP condition in `AGENTS.md` applies.

## Milestone Rules
Each milestone defines:
- goal,
- files to read,
- files to change,
- exact edits expected,
- validation command,
- expected result,
- recovery instruction.

## Validation Rules
- Use only commands from `COMMANDS.md`.
- If a command is missing or stale, update `COMMANDS.md` first with repository evidence.
- Validation is narrow during milestones and broad at plan completion (`./scripts/verify.sh`).

## Acceptance Rules
An ExecPlan is accepted only when:
- every in-scope milestone is complete,
- acceptance criteria pass,
- required validation commands pass,
- only expected files changed (extras justified in Decision Log),
- TRACEABILITY.md rows advanced,
- Progress, Decision Log, and Outcomes & Retrospective updated.

## Idempotence Rules
- Steps must be safe to rerun.
- Use additive changes by default; never overwrite the audit chain or the SpecAnchor in place.
- Document any non-idempotent step with explicit pre/post conditions.

## Recovery Rules
- First failure: smallest targeted fix.
- Second same-root failure: narrower diagnostic.
- Third same-root failure: abandon approach, record failed hypotheses, choose simpler path; STOP if blocked.
- Stop only under `AGENTS.md` STOP conditions.

## Progress Update Rules
- Update checkboxes after each milestone.
- Record what was completed, what remains, and any blockers.
- Keep the plan accurate enough for a new agent to resume.

## Decision Log Rules
For each meaningful decision record:
- date,
- decision,
- reason,
- alternatives considered,
- impact on files/tests/docs,
- linked ADR ID (if applicable).

## Completion Rules
Before closing an ExecPlan:
- run `./scripts/verify.sh` and required readiness checks,
- review diff against expected changed files,
- update Outcomes & Retrospective and residual risks,
- advance TRACEABILITY.md rows.
