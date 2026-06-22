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
- [x] Milestone 1 complete (`cargo nextest run -p vaultcore-core obs::redaction` passed)
- [x] Milestone 2 complete (`./scripts/smoke-test.sh` passed)
- [x] Milestone 3 complete (`pnpm --dir app test:e2e -- vault-health.spec.ts` passed)
- [x] Milestone 4 complete (`./scripts/verify.sh && cargo nextest run --test no_network` passed)

## 13. Surprises & Discoveries
- Milestone 1 validation targets `vaultcore-core`, so the shared redaction filter belongs in `crates/core/src/obs` and builder/verifier emitters should consume it rather than each defining their own deny-list.
- The pre-EP-008 smoke script skipped when no smoke binary existed. Milestone 2 replaced the skip with local-only Builder and Verifier observability contract tests.
- The existing `audit-health.spec.ts` selector became ambiguous after adding a SpecAnchor alert message. Updated the test to target the SpecAnchor heading explicitly.

## 14. Decision Log
- ADR-0010 (no remote telemetry) re-affirmed.
- Added `crates/core/src/obs/*` and `crates/core/src/lib.rs` beyond the literal file list because EP-008 requires a central redaction filter and the Milestone 1 command validates the core crate.
- Added `serde_json` to Builder and Verifier so their log emitters can call the shared core redaction filter without introducing a second logging format.
- Added `crates/tests/invariants/tests/no_network.rs` beyond the literal file list because Milestone 4 requires the named `cargo nextest run --test no_network` sentinel.
- Updated `app/tests/e2e/audit-health.spec.ts` beyond the literal file list to keep the existing EP-005/EP-007 health acceptance coverage valid after the new Vault Health alert text landed.

## 15. Outcomes & Retrospective
- EP-008 completed locally.
- Structured log redaction, local health/metrics contract tests, Vault Health UI alerts, runbooks, and no-network sentinel coverage are implemented.
- `./scripts/verify.sh && cargo nextest run --test no_network` exits successfully.
- Known residual tooling note: Windows Tauri MSI bundling still reports `Access is denied. (os error 5)` during `build.sh`, and the script treats it as a non-fatal platform prerequisite issue before reporting `build: ok`.
