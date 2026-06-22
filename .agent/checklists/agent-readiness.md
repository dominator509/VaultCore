# Agent Readiness Checklist (VaultCore)

- [ ] Exactly one active ExecPlan is selected.
- [ ] The ExecPlan is self-contained (no hidden context required).
- [ ] Required files to read are listed (including uploaded Architecture.md, THREAT_MODEL.md, TRACEABILITY.md references).
- [ ] Required files to change are listed.
- [ ] Exact commands are defined in `COMMANDS.md` (Rust + TS + Tauri).
- [ ] Expected command outputs/results are stated.
- [ ] Acceptance criteria are observable and objective.
- [ ] Non-goals are explicit (no cloud sync, no new roles, no new secret types, no telemetry).
- [ ] STOP conditions are explicit (from `AGENTS.md`).
- [ ] Recovery rules are explicit (bounded retry).
- [ ] Diff review rule is explicit.
- [ ] TRACEABILITY.md rows touched by the ExecPlan are identified.
- [ ] Any invariant (I-1..I-8) touched has a planned enforcement test.
- [ ] Any threat (T-001..T-023) touched has a planned mitigation or accepted residual.
