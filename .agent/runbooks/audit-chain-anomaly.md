# Audit Chain Anomaly Runbook

## Trigger
Vault Health reports `Audit chain anomaly`, audit verification returns invalid, or a smoke test finds an unexpected audit head.

## Immediate Actions
1. Stop writes and keep the vault in read-only investigation mode.
2. Do not purge, compact, rekey, or migrate the vault.
3. Record the audit head hash, last append timestamp, session id, and role shown in Vault Health.
4. Run `./scripts/smoke-test.sh` on a fixture vault to separate product behavior from local vault state.

## Recovery
1. Restore from the most recent encrypted backup only after owner approval.
2. Verify restored audit continuity before allowing writes.
3. Add a regression test for the discovered break before release promotion.

## Escalation
Treat as Sev-1 because it can affect invariant I-8. Follow `OPERATIONS.md` incident triage and `ROLLBACK.md` release rollback guidance.
