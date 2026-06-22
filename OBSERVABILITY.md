# OBSERVABILITY.md

## Logging Strategy
- Structured JSON lines.
- Emitted only at trust boundaries and at critical lifecycle events.
- Redaction filter applied at every emission point.
- Log destinations:
  - Dev: `VAULTCORE_DEV_DIR/logs/builder.log`, `verifier.log`, `ui.log`
  - Production: platform standard log directory; user-accessible
- No remote sink in default builds.

## Structured Log Fields
Allowed fields:
- `ts` (ISO 8601)
- `level` (`trace`/`debug`/`info`/`warn`/`error`)
- `component` (`builder` / `verifier` / `ui`)
- `op` (operation name, enumerated)
- `session_id` (opaque)
- `secret_id` (opaque)
- `role` (Owner/Admin/Editor/Viewer/Auditor)
- `status` (`ok`/`denied`/`error`)
- `duration_ms`
- `audit_seq` (monotonic)
- `err_code` (taxonomy in SPEC-006)

Forbidden fields: payloads, signing keys, derived keys, nonces, passphrases, biometric templates, raw audit pre-images, file paths to user vaults.

## Redaction Rules
- Apply a deny-list at every emit site; a unit test injects synthetic markers and asserts they never appear in any log destination.
- Redact unknown fields by default if the redaction filter is uncertain.

## Metrics
Local counters/histograms exposed only over the local introspection channel:
- `builder.unlock.success_total`
- `builder.unlock.failure_total`
- `builder.reveal.success_total`
- `builder.reveal.duration_ms` (histogram)
- `builder.write.success_total`
- `verifier.countersign.success_total`
- `verifier.countersign.denied_total{reason}`
- `verifier.audit.append_total`
- `ipc.signature_failures_total`
- `ipc.replay_rejections_total`

No remote metrics export in default builds.

## Traces
- Not in v1. (Tracing within a single device is low-value; cross-process correlation IDs are already in logs and metrics.)

## Health Checks
- `/health/builder`: SpecAnchor verified, IPC up, last audit append ok, error count.
- `/health/verifier`: SpecAnchor verified, audit tail hash, session count.
- Local-only sockets/pipes; not network.

## Uptime Checks
- Not applicable (local app).

## Dashboards
- A built-in "Vault Health" view in the UI surfaces:
  - SpecAnchor verification status
  - Last audit append timestamp
  - Audit chain head hash
  - Session and role
  - Counts and recent denials (without payload references)

## Alerts
- Local user-facing alerts:
  - SpecAnchor tamper detected.
  - Audit chain break detected.
  - Repeated IPC signature failures (T-014..T-016).
  - Repeated authz denials (potential misconfiguration).

## SLIs / SLOs
- Unlock success rate (passkey path): ≥ 99 % under normal conditions.
- Reveal latency: 95th percentile < 100 ms after countersignature.
- Search latency: 95th percentile < 200 ms on 10,000-record fixture.

## Debugging Production Issues
1. Confirm current version and SpecAnchor.
2. Check health views.
3. Inspect local logs with `session_id` / `secret_id` correlation.
4. Recreate locally with a fixture vault.
5. If suspected Sev-1 (I-1/I-4/I-5/I-7), stop and escalate per `OPERATIONS.md`.

## Observability Acceptance Criteria
- Every invariant has at least one observable signal (log line, metric, or health field) that lights up when it would be violated.
- Redaction tests pass.
- Smoke test asserts on health endpoints.
- No log destination is a remote sink in default builds.
