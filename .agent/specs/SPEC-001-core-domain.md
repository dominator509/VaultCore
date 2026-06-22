# SPEC-001 Core Domain

- **Status:** Draft
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phase 1
- **Linked ExecPlans:** EP-002

## User-Visible Goal
The domain layer in `crates/core` captures VaultCore's business rules — secret types, roles, lifecycle FSM, identifiers, and error taxonomy — without any I/O or framework leakage.

## Non-Goals
- Persistence details (SPEC-002)
- IPC details (SPEC-003 / Architecture.md)
- UI rendering (SPEC-004)
- Auth ceremonies (SPEC-005)

## Terms
- **Secret:** a typed, named, lifecycle-stateful record owned by a user, with metadata and an encrypted payload.
- **Role:** one of Owner, Admin, Editor, Viewer, Auditor.
- **LifecycleState:** one of `draft`, `active`, `expiring_soon`, `expired`, `rotating`, `archived`, `soft_deleted`, `purged`.

## Required Behavior

### Secret Types (exhaustive)
`API_KEY`, `LOGIN`, `OAUTH_APP`, `SSH_KEY`, `WALLET_KEY`, `CERT`, `NOTE`, `BLOB`.
- Each type has its own structured-metadata shape.
- The payload is always opaque to persistence (ciphertext) and to Verifier (never seen).

### Roles (exhaustive)
Owner > Admin > Editor > Viewer; Auditor is a parallel role with read-only access to metadata and audit only.
- Operations declare a minimum role.
- Auditor never has access to payloads.

### Lifecycle FSM (exhaustive transitions)
- `draft → active`
- `active → expiring_soon` (time-based)
- `expiring_soon → expired` (time-based)
- `expired → rotating` (user-initiated)
- `rotating → active` (rotation completes)
- `active → archived`
- `archived → soft_deleted`
- `soft_deleted → purged` (cryptographic erasure)
- `purged` is terminal; `archived` and `soft_deleted` are reversible.

### Identifiers
- `SecretId` is an opaque ULID. Never reuse identifiers; purged IDs are tombstoned.

### Validation
- Names: non-empty, ≤ 255 UTF-8 bytes, no control characters.
- Labels: list of ≤ 32 entries, each ≤ 64 UTF-8 bytes.
- Type-specific validation per secret type (defined per-type in `crates/core::types::*`).

### Error Taxonomy (initial)
- `Validation { field, code }`
- `Unauthorized { role, op }`
- `NotFound { id }`
- `Conflict { id, expected_state, actual_state }`
- `FsmInvalid { from, to }`
- `Internal { code }`

## Inputs
- Pure values from caller; no I/O.

## Outputs
- Deterministic domain results and stable domain errors.

## Error States
- Invalid type-specific field
- FSM transition violation
- Authorization decision: deny (mapped at boundary)

## Data Rules
- Domain types must be `serde`-serializable for persistence and IPC, but must not depend on database or IPC crates.
- Plaintext payloads are not domain types; they appear only as sealed handles passed through Builder.

## Security Rules
- Domain code must not import logging, persistence, or networking crates.
- Domain code must not contain string literals matching any sensitive marker.

## Accessibility Rules
Not applicable at domain layer.

## Performance Rules
- FSM transitions and validation are O(1) or O(n) over small lists.

## Observability Rules
- Domain code emits no logs; calling layers log at boundaries.

## Required Tests
- FSM transition table tests (every legal transition + every illegal transition).
- Validation tests per secret type (valid + invalid cases).
- Error taxonomy exhaustiveness tests.
- Round-trip serde tests.

## Acceptance Criteria
- Five roles enforced.
- Eight secret types defined.
- All FSM transitions tested.
- No infrastructure imports in `crates/core` (enforced by `deny.toml` or static check).
- Unit tests pass: `cargo nextest run -p vaultcore-core`.
