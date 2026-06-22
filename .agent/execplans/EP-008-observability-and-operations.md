# EP-008 Observability and Operations

## 1. Purpose / Big Picture
Add structured redacted logs, local metrics, health endpoints, the in-UI Vault Health view, and runbooks. No remote sinks.

## 2. Scope
- Logging infra in Builder, Verifier, UI with central redaction filter.
- Local metrics counters/histograms per OBSERVABILITY.md.
- `/health/builder` and `/health/verifier` over local introspection sockets.
- Vault Health view in UI.
- Runbooks for common failures.

## 3. Non-goals
- Remote telemetry (forbidden by I-7).
- Distributed tracing.

## 4. Context and Orientation
After EP-004 onward. Reads OBSERVABILITY.md, SPEC-007.

## 5. Files to Read First
- `OBSERVABILITY.md`, `.agent/specs/SPEC-007-observability.md`, `OPERATIONS.md`

## 6. Files to Change
- `crates/builder/src/obs/*`, `crates/verifier/src/obs/*`, `app/src/components/VaultHealth.tsx`
- Runbooks under `.agent/runbooks/*`
- `scripts/smoke-test.sh` (assert on health endpoints)
- `TRACEABILITY.md` rows for L7
- This ExecPlan

## 8. Milestones

### Milestone 1 — Logging + redaction
- **Validation Command:** `cargo nextest run -p vaultcore-core obs::redaction`
- **Expected Result:** Synthetic markers never appear in any log destination.

### Milestone 2 — Metrics + health endpoints
- **Validation Command:** `./scripts/smoke-test.sh`
- **Expected Result:** Smoke checks `/health/*` and metric exposure.

### Milestone 3 — Vault Health view + alerts
- **Validation Command:** `pnpm --dir app test:e2e -- vault-health.spec.ts`
- **Expected Result:** UI shows SpecAnchor status, audit head, sessions; alert states render.

### Milestone 4 — Runbooks + sentinel "no remote network call" test
- **Validation Command:** `./scripts/verify.sh && cargo nextest run --test no_network`
- **Expected Result:** Green; TRACEABILITY L7 rows VERIFIED.

## 9. Concrete Steps
1. Wire structured logs + redaction filter.
2. Add metrics + local introspection.
3. Build Vault Health view.
4. Author runbooks; add no-network sentinel test.

## 10. Validation and Acceptance
- Redaction tests green.
- Health/smoke checks green.
- No remote sink in default builds.

## 11. Idempotence and Recovery
- Observability is additive; safe to rerun.

## 12. Progress
- [ ] Milestone 1 complete
- [ ] Milestone 2 complete
- [ ] Milestone 3 complete
- [ ] Milestone 4 complete

## 13. Surprises & Discoveries
- None yet.

## 14. Decision Log
- ADR-0010 (no remote telemetry) re-affirmed.

## 15. Outcomes & Retrospective
- Pending execution.
