# Continue Partially Completed ExecPlan Prompt

You are continuing a partially completed VaultCore ExecPlan.

Input:
- ExecPlan: `[EXECPLAN_PATH]`

Instructions:
1. Read `AGENTS.md`, `COMMANDS.md`, `.agent/PLANS.md`, and `[EXECPLAN_PATH]`.
2. Inspect:
   - `Progress`
   - `Surprises & Discoveries`
   - `Decision Log`
3. Resume at the first incomplete milestone.
4. Validate prior assumptions against current repository state before editing (especially crypto primitives, IPC schema, role set, secret types, audit fields).
5. Use only commands from `COMMANDS.md`.
6. Do not broaden scope.
7. Implement milestones in order.
8. Update the ExecPlan after each milestone.
9. Continue autonomously.
10. Do not ask for next steps.
11. Stop only under STOP conditions.

At completion:
- run `./scripts/verify.sh`,
- run `git diff --name-only`,
- advance TRACEABILITY.md rows,
- update Outcomes & Retrospective,
- report changed files, commands, results, decisions, risks, acceptance status, and the next gate.
