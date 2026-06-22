# Final Review Checklist (VaultCore)

- [ ] Acceptance criteria from the active ExecPlan are satisfied.
- [ ] `./scripts/verify.sh` passed (local + CI matrix).
- [ ] `./scripts/production-readiness-check.sh` passed if applicable.
- [ ] `cargo nextest run --test invariants` passed.
- [ ] `git diff --name-only` reviewed against expected changed files.
- [ ] Relevant docs updated (ARCHITECTURE, SECURITY, COMMANDS, OBSERVABILITY, RUNBOOKS).
- [ ] No secrets, signing keys, or real SpecAnchors committed.
- [ ] No production user data touched.
- [ ] Residual risks (R-1..R-5) noted if touched.
- [ ] TRACEABILITY.md rows advanced.
- [ ] Threat-coverage map updated for any touched threat.
- [ ] Final response includes files, commands, results, decisions, assumptions, acceptance status, next gate.
