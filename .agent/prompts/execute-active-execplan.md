# Execute Active ExecPlan Prompt

You are executing a VaultCore ExecPlan.

Inputs:
- ExecPlan: `[EXECPLAN_PATH]`
- Optional user request: `[OPTIONAL_USER_REQUEST]`

Instructions:
1. Read `AGENTS.md`.
2. Read `COMMANDS.md`.
3. Read `.agent/PLANS.md`.
4. Read `[EXECPLAN_PATH]` fully.
5. Read the uploaded `Architecture.md`, `THREAT_MODEL.md`, `TRACEABILITY.md` references.
6. Run `./scripts/preflight.sh`.
7. Implement milestones in order.
8. Before editing, inspect the files listed under "Files to Read First" and the current milestone's "Files to Read".
9. Use only commands from `COMMANDS.md`.
10. Do not invent crypto primitives, IPC schemas, env vars, roles, secret types, or audit fields.
11. Update Progress, Surprises & Discoveries, and Decision Log after each milestone.
12. Validate each milestone with the milestone's validation command.
13. Continue autonomously to the next milestone.
14. Do not ask for next steps.
15. Stop only under STOP conditions in `AGENTS.md`.
16. At completion, run `./scripts/verify.sh`, run `git diff --name-only`, compare changed files to the ExecPlan, advance TRACEABILITY.md, update Outcomes & Retrospective, and produce the final report.

Output report must include:
- ExecPlan completed
- Changed files
- Commands run and results
- Acceptance criteria status
- Decisions made (with ADR IDs)
- Assumptions confirmed or changed
- Remaining risks (including residual risks if applicable)
- TRACEABILITY.md row status changes
- Next gate
