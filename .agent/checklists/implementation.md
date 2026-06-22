# Implementation Checklist (VaultCore)

- [ ] Read `AGENTS.md`, `COMMANDS.md`, `.agent/PLANS.md`, active ExecPlan, relevant specs.
- [ ] Read uploaded `Architecture.md`, `THREAT_MODEL.md`, `TRACEABILITY.md` references.
- [ ] Read existing crate/module patterns before editing.
- [ ] Implement one milestone at a time.
- [ ] Do not broaden scope (no new roles, no new secret types, no new IPC schemas).
- [ ] Update ExecPlan Progress after each milestone.
- [ ] Update Surprises & Discoveries when reality differs.
- [ ] Update Decision Log for meaningful choices (link ADR IDs).
- [ ] Validate each milestone with the exact command listed.
- [ ] Advance TRACEABILITY.md rows as work lands.
- [ ] Never weaken signatures, SpecAnchor verification, or audit chain to make a test pass.
- [ ] Continue unless a STOP condition applies.
