# EP-010 Production Readiness

## 1. Purpose / Big Picture
Final gate. Verify every invariant, every threat, every spec, every gate. Produce the go/no-go evidence packet.

## 2. Scope
- Full verification on all three OSes.
- Invariant suite re-run.
- Threat coverage map re-verified.
- Security review notes.
- Performance and accessibility re-checks.
- Privacy review.
- Backup/restore verification.
- Monitoring/health re-verified.
- Deployment dry run + rollback drill review.
- Documentation review.
- TRACEABILITY.md fully green.

## 3. Non-goals
- Net-new features (only launch-blocking fixes).
- Publishing without owner approval.

## 4. Context and Orientation
After EP-009. Reads PRODUCTION_READINESS.md, SPEC-008.

## 5. Files to Read First
- `PRODUCTION_READINESS.md`, all specs, runbooks, CHANGELOG.md

## 6. Files to Change
- Readiness docs/checklists.
- Narrow launch-blocking fixes only.
- `PRODUCTION_READINESS.md` final status.
- `TRACEABILITY.md` (all rows VERIFIED + GATE PASSED).
- This ExecPlan.

## 8. Milestones

### Milestone 1 — Full verification + invariant suite
- **Validation Command:** `./scripts/verify.sh && cargo nextest run --test invariants && cargo nextest run --test threats`
- **Expected Result:** Green on each OS.

### Milestone 2 — Readiness domains review (security, privacy, performance, a11y, observability, deployment, rollback, data, docs)
- **Validation Command:** `./scripts/production-readiness-check.sh`
- **Expected Result:** Exit 0 or exact remaining gaps recorded.

### Milestone 3 — Final launch packet + owner approval
- **Validation Command:** `git diff --name-only`
- **Expected Result:** Only readiness-related files changed; owner approval recorded in DECISIONS.md.

## 9. Concrete Steps
1. Run verification matrix.
2. Close any launch-blocking gap (narrow fixes only).
3. Run readiness script.
4. Produce evidence packet.

## 10. Validation and Acceptance
- All checks pass.
- All gates G1-A..G3-E passed with linked evidence.
- All invariants I-1..I-8 have passing enforcement tests.
- All threats T-001..T-023 mitigated or accepted as residual.
- Owner approval recorded.

## 11. Idempotence and Recovery
- Readiness review can be rerun.
- No destructive operations.

## 12. Progress
- [ ] Milestone 1 complete
- [ ] Milestone 2 complete
- [ ] Milestone 3 complete

## 13. Surprises & Discoveries
- None yet.

## 14. Decision Log
- Final owner approval and any accepted residual risks captured here.

## 15. Outcomes & Retrospective
- Pending execution.
