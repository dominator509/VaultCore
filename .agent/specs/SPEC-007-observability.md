# SPEC-007 Observability

- **Status:** Draft
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phase 7
- **Linked ExecPlans:** EP-008

## User-Visible Goal
Operators (here, the user themselves) can understand local app health via redacted logs, local metrics, and a UI Vault Health view. No telemetry leaves the device by default.

## Non-Goals
- Remote observability sinks (v1)
- Distributed tracing (v1)

## Terms
- **Local introspection channel:** a Unix domain socket / named pipe exposing `/health/*` and `/metrics`.

## Required Behavior

### Logging
- Structured JSON per `OBSERVABILITY.md`.
- Redaction filter at every emit site.
- Levels: `trace` (dev only), `debug`, `info`, `warn`, `error`.

### Metrics
- Local counters and histograms enumerated in `OBSERVABILITY.md`.
- Exposed only via the local introspection channel.

### Health
- `/health/builder` and `/health/verifier` reflect SpecAnchor verification, IPC link, audit append status, error counts.
- UI Vault Health view consumes both.

### Audit Visibility
- UI surfaces audit chain head hash, last append time, and a "verify now" action.

### Alerts (local, user-facing)
- SpecAnchor tamper detected.
- Audit chain anomaly.
- Repeated IPC signature failures.
- Repeated authz denials (potential misconfiguration).

## Inputs / Outputs
- Logs to platform-standard locations (and `VAULTCORE_DEV_DIR/logs/` in dev).
- Metrics over local introspection channel only.

## Error States
- Log emission failure (disk full): degrade to in-memory ring buffer; surface a warning.
- Introspection channel failure: degrade gracefully; health view shows last-known status.

## Data Rules
- Allowed log fields enumerated.
- Forbidden fields enumerated.
- Metrics carry no payload-derived data.

## Security Rules
- Redaction tests must catch synthetic markers in any log destination.
- No introspection channel reachable over network.

## Performance Rules
- Instrumentation overhead < 5 % on hot paths.

## Required Tests
- Redaction marker test on all sinks.
- Health endpoint smoke test (part of `./scripts/smoke-test.sh`).
- Metric counter unit tests.
- "No remote network call" sentinel test.

## Acceptance Criteria
- All required tests pass.
- Vault Health view present and accurate.
- No remote sink in default builds.
