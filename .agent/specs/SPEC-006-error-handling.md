# SPEC-006 Error Handling

- **Status:** Draft
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phases 1-7
- **Linked ExecPlans:** EP-002, EP-003, EP-004, EP-005, EP-006, EP-007, EP-008

## User-Visible Goal
A stable error taxonomy that lets the UI surface predictable, redaction-safe messages; that lets Builder and Verifier map internal failures consistently; and that supports bounded retry behavior.

## Non-Goals
- Stack traces in user-facing messages.
- Silent failures.
- Infinite retries.

## Terms
- **Error code:** stable string from the taxonomy below.
- **Recoverable error:** can be retried without data loss.
- **Fatal error:** requires user action or escalation.

## Required Behavior

### Taxonomy (initial; additive only)
| Code | Category | Recoverable | Notes |
|---|---|---|---|
| `VC-VAL-001..` | Validation | Yes | User input rejected; surface field. |
| `VC-AUTH-001..` | Auth/Session | Yes/No | `VC-AUTH-001 SessionExpired` is recoverable by relock. |
| `VC-AUTHZ-001..` | Authorization | No | Default-deny path. |
| `VC-FSM-001..` | Lifecycle | No | Illegal transition. |
| `VC-CRYPTO-001..` | Crypto failure | No | AEAD verify failure; treat as suspected tampering. |
| `VC-IPC-001..` | IPC | Yes (with backoff) | Bad signature, replay, framing error. |
| `VC-PERSIST-001..` | Persistence | Sometimes | Constraint, IO, migration. |
| `VC-AUDIT-001..` | Audit chain | No | Chain anomaly → tamper-evident mode. |
| `VC-SPEC-001..` | SpecAnchor | No | Signature failure → refuse to start. |
| `VC-INTERNAL-001..` | Internal | No | Generic; surface a safe message. |

### Mapping
- Builder maps internal errors to taxonomy codes before sending to UI.
- Verifier returns taxonomy codes (no internal details).
- UI surfaces a human-readable message + the code (for support).

### Retry Rules
- IPC errors: bounded retry with exponential backoff (1, 2, 4 attempts max).
- Crypto verify failures: never retried; surfaces immediately.
- Audit chain anomalies: never retried; enters tamper-evident mode.

### Redaction
- Error payloads must not include plaintext, signing keys, nonces, hash pre-images, or file paths to user vaults.

## Inputs / Outputs
- Builder: returns `Result<T, VaultError>` where `VaultError` has `code`, `category`, `recoverable`, optional `field`.
- UI: maps `VaultError` to a string from `app/src/i18n/en.json`.

## Error States
Enumerated above.

## Data Rules
- Errors are typed (Rust `enum` + TS discriminated union); no stringly-typed errors.

## Security Rules
- Crypto and audit chain errors surface as fatal; never silently retried.
- Authz denials are audited with the `denied` reason code.

## Accessibility Rules
- UI shows error code + message; does not rely on color alone.

## Performance Rules
- Error mapping is O(1).

## Observability Rules
- Every error logged with `err_code`, `op`, `component`, `session_id`, `duration_ms`.

## Required Tests
- Per-category mapping tests.
- Redaction tests for error payloads.
- Bounded retry test for IPC errors.
- Audit anomaly test (no retry, enters tamper-evident).

## Acceptance Criteria
- Taxonomy implemented and used everywhere.
- Bounded retry rules implemented and tested.
- Redaction verified.
