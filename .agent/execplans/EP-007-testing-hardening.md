# EP-007 Testing Hardening

## 1. Purpose / Big Picture
Close coverage gaps, add regression tests for any bug found in earlier ExecPlans, and ensure every invariant I-1..I-8 has at least one failing-when-violated test. Stabilize CI.

## 2. Scope
- Invariant enforcement test suite completeness.
- Threat-coverage tests linking T-001..T-023 to code/tests.
- Regression tests for issues found in EP-002..EP-006.
- Test data fixtures generator script.
- Flaky-test policy and CI stabilization.

## 3. Non-goals
- New features.
- Performance test framework changes beyond budgets enforcement.

## 4. Context and Orientation
After EP-005 and EP-006. Reads TESTING.md and THREAT_MODEL.md.

## 5. Files to Read First
- `TESTING.md`
- `THREAT_MODEL.md`
- `TRACEABILITY.md`
- existing tests across crates and `app/`

## 6. Files to Change
- `crates/tests/invariants/*`
- `tests/fixtures/*` and `scripts/generate-test-vaults.sh`
- Test files across crates and `app/`
- `TRACEABILITY.md` (advance rows)
- This ExecPlan

## 8. Milestones

### Milestone 1 — Invariant suite completeness for I-1..I-8
- **Validation Command:** `cargo nextest run --test invariants`
- **Expected Result:** Every invariant has at least one explicit test that fails when violated; suite is green now.

### Milestone 2 — Threat-coverage tests
- **Validation Command:** `./scripts/security-check.sh && cargo nextest run --test threats`
- **Expected Result:** Each threat row references at least one test (or accepted residual risk).

### Milestone 3 — Fixtures generator + regression tests
- **Validation Command:** `./scripts/generate-test-vaults.sh && ./scripts/verify.sh`
- **Expected Result:** Deterministic fixtures generated; regression tests pass.

### Milestone 4 — CI parity + flaky policy
- **Validation Command:** `./scripts/verify.sh` (locally) and CI green
- **Expected Result:** Stable, repeatable verification matrix.

## 9. Concrete Steps
1. Audit invariant test coverage; fill gaps.
2. Link threats to tests.
3. Generate fixtures; add regressions.
4. Stabilize CI; document flaky policy.

## 10. Validation and Acceptance
- Invariant suite green.
- Threat coverage map complete.
- CI stable across platforms.

## 11. Idempotence and Recovery
- Tests additive; fixtures deterministic.

## 12. Progress
- [ ] Milestone 1 complete
- [ ] Milestone 2 complete
- [ ] Milestone 3 complete
- [ ] Milestone 4 complete

## 13. Surprises & Discoveries
- None yet.

## 14. Decision Log
- Flaky-test policy captured here.

## 15. Outcomes & Retrospective
- Pending execution.
