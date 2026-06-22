# SPEC-003 IPC and Boundary Contracts

- **Status:** Draft
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phase 3
- **Linked ExecPlans:** EP-004

## User-Visible Goal
Define the stable contracts at every trust boundary: UI ↔ Builder (Tauri IPC), Builder ↔ Verifier (signed local IPC), Builder ↔ Persistence, and Verifier ↔ Audit log.

## Non-Goals
- Any HTTP/REST API (there is no server)
- Any cloud RPC

## Terms
- **Tauri command:** a named UI → Builder call.
- **Trinity message:** a Builder ↔ Verifier signed, length-prefixed message.

## Required Behavior

### UI → Builder (Tauri Commands)
Enumerated, named, validated, role-aware. Examples (full list lives in `crates/builder::api`):
- `unlock(method, proof)` → `SessionToken`
- `list(filter)` → `Vec<SecretSummary>`
- `reveal(secret_id, reason)` → `RevealResponse { ttl_ms, payload_handle }`
- `copy(secret_id, ttl_ms)` → `Ack`
- `create(secret_input)` → `SecretSummary`
- `update(secret_id, patch)` → `SecretSummary`
- `rotate(secret_id, new_payload)` → `SecretSummary`
- `soft_delete(secret_id)` → `Ack`
- `purge(secret_id, confirmation_token)` → `Ack`
- `audit_view(filter)` → `Vec<AuditEntry>`
- `verify_audit_chain()` → `ChainStatus`
- `lock()` → `Ack`

### Builder ↔ Verifier (Trinity Messages)
- Framed (length-prefixed), Ed25519-signed, replay-protected with `(session_id, monotonic_counter)`.
- Enumerated message types:
  - `AuthorizeOp { op, target_id, role, session_id }` → `Countersignature` or `Denied`
  - `AppendAudit { op, target_id, result, payload_hash }` → `Ack { entry_hash }`
  - `VerifyChain { head }` → `Status`
  - `IssueSession { auth_proof }` → `SessionToken`
  - `RevokeSession { session_id }` → `Ack`
- Verifier never receives plaintext payloads. Builder hashes payloads and sends only `payload_hash` (and metadata) to Verifier.

### Builder ↔ Persistence
- Repositories in `crates/core::persistence::repo`.
- Builder calls repository methods; repository methods do not log or audit (Builder's responsibility).

### Verifier ↔ Audit Log
- Append-only; chain continuity enforced; entries countersigned by Verifier.

### Versioning
- Tauri command schema version and Trinity message schema version are part of the SpecAnchor. Mismatch causes startup failure with a clear error.

### Validation
- Every message validates: schema, signature, freshness, session, role.
- Default-deny on unknown ops.

## Inputs / Outputs
Detailed types live in `crates/core::types` and `crates/builder::api`; this spec lists the names and rules.

## Error States
- `BadSchema`, `BadSignature`, `Replay`, `Expired`, `Unauthorized`, `NotFound`, `Conflict`, `RateLimited`, `Internal`.

## Data Rules
- No plaintext over Builder ↔ Verifier.
- Tauri IPC frames are scoped to the desktop process; not exposed over network.

## Security Rules
- Both sides verify SpecAnchor before any message.
- Replay counter is monotonic per session and reset on session change.
- Denied operations are logged and audited (with `result = denied`).

## Performance Rules
- Authorize round-trip < 5 ms in steady state.

## Observability Rules
- Log every message at Builder and Verifier with the redaction filter.
- Metric counters per message type.

## Required Tests
- Contract tests for every Tauri command.
- Contract tests for every Trinity message.
- Negative tests: bad signature, replay, expired session, unauthorized role.
- Cross-version mismatch test.

## Acceptance Criteria
- All commands and messages implemented and tested.
- Every (Role × Operation) cell has explicit allow/deny coverage.
