# Debug Validation Failure Prompt

You are debugging a failing validation command for the active VaultCore ExecPlan.

Instructions:
1. Read `AGENTS.md`, `COMMANDS.md`, `.agent/PLANS.md`, and the active ExecPlan.
2. Do not rewrite unrelated code.
3. Capture the exact failing command.
4. Capture the exact error output.
5. Form one hypothesis about the root cause.
6. Make the smallest targeted fix.
7. Rerun the narrowest relevant command first.
8. If the same root cause fails again, run a narrower diagnostic and isolate the issue.
9. After three same-root failures, use the anti-fixation rule:
   - stop that approach,
   - record failed hypotheses in `Surprises & Discoveries`,
   - choose a simpler implementation path if safe.
10. Never tune crypto parameters, weaken signatures, or relax SpecAnchor verification to make a test pass. STOP instead.
11. Update the ExecPlan with:
    - failing command,
    - error summary,
    - hypothesis,
    - fix attempted,
    - result.

Do not ask for next steps unless a STOP condition applies.
