# Final Review Prompt

You are performing the final review for the active VaultCore ExecPlan.

Instructions:
1. Read `AGENTS.md`, `COMMANDS.md`, the active ExecPlan, relevant specs, and the affected TRACEABILITY.md rows.
2. Run full verification: `./scripts/verify.sh`.
3. Run `./scripts/production-readiness-check.sh` if this ExecPlan reaches a gate.
4. Run `git diff --name-only`.
5. Compare changed files with the active ExecPlan's expected changed files.
6. Verify acceptance criteria.
7. Confirm any touched invariant (I-1..I-8) has at least one enforcement test that fails when violated.
8. Confirm any touched threat (T-001..T-023) is mitigated with linked evidence or accepted as a residual risk.
9. Advance TRACEABILITY.md rows.
10. Update `Outcomes & Retrospective`.
11. Produce a final report with:
    - completion status,
    - changed files,
    - commands run and results,
    - acceptance criteria status,
    - decisions made (with ADR IDs),
    - assumptions confirmed or changed,
    - remaining risks,
    - whether production-readiness criteria advanced and which gate is next.
