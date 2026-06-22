# SPEC-002 Data Model

- **Status:** Draft
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phase 2
- **Linked ExecPlans:** EP-003

## User-Visible Goal
Define the SQLite schema, the hash-chained audit log, retention rules, and migrations such that I-1 (no plaintext at rest), I-3 (metadata searchable, payloads opaque), and I-8 (every action audited) are enforced by the schema itself.

## Non-Goals
- Backup format details beyond "the vault file is the backup" (v1)
- Multi-device sync schema (v1)

## Terms
- **Vault file:** a single SQLite file containing secrets, audit log, and metadata.
- **Envelope:** the AEAD-encrypted payload structure stored in payload columns.

## Required Behavior

### Tables (v1)
- `secrets`
  - `id TEXT PRIMARY KEY` (ULID)
  - `type TEXT NOT NULL` (one of the eight)
  - `name TEXT NOT NULL`
  - `labels TEXT NOT NULL` (JSON array)
  - `state TEXT NOT NULL` (FSM state)
  - `created_at INTEGER NOT NULL` (UNIX ms)
  - `updated_at INTEGER NOT NULL`
  - `expires_at INTEGER NULL`
  - `payload_envelope BLOB NULL` (ciphertext; AEAD)
  - `payload_dek_id TEXT NULL` (per-secret data-encryption-key handle)
  - `meta TEXT NOT NULL` (JSON, type-specific structured metadata; no sensitive payload)
- `audit_log` (append-only)
  - `seq INTEGER PRIMARY KEY AUTOINCREMENT`
  - `ts INTEGER NOT NULL`
  - `actor TEXT NOT NULL` (role + session_id)
  - `op TEXT NOT NULL` (operation name, enumerated)
  - `target_id TEXT NULL` (secret_id; opaque)
  - `result TEXT NOT NULL` (`ok`/`denied`/`error`)
  - `prior_hash BLOB NOT NULL`
  - `payload_hash BLOB NOT NULL` (hash over canonical CBOR of the audited fields, excluding any plaintext)
  - `entry_hash BLOB NOT NULL` (= H(prior_hash || payload_hash))
  - `countersignature BLOB NOT NULL` (Verifier Ed25519 signature over entry_hash)
- `specanchor_meta`
  - `version TEXT PRIMARY KEY`
  - `loaded_at INTEGER NOT NULL`
- `migrations`
  - `version INTEGER PRIMARY KEY`
  - `applied_at INTEGER NOT NULL`

### Constraints
- `payload_envelope` may contain only AEAD ciphertext; never plaintext, never JSON of a secret.
- `meta` JSON schema per type is strict; no free-form fields beyond the documented set.
- Indices: `(type)`, `(state)`, `(name)`, `(expires_at)`. No index on payload columns.

### Audit Chain
- `entry_hash[n] = H(prior_hash[n] || payload_hash[n])`, where `prior_hash[n] = entry_hash[n-1]`, `prior_hash[0]` is the genesis hash recorded in the SpecAnchor.
- The audit chain head is the latest `entry_hash`; surfaced in the UI Vault Health view.
- Tamper detection: re-verify the chain at startup; any mismatch puts the app in tamper-evident read-only mode.

### Migrations
- Additive by default.
- Destructive migrations require an ADR + rollback path.
- Migrations preserve audit-chain continuity (the migration itself is audited).

### Retention
- `soft_deleted`: kept indefinitely until purge.
- `purged`: payload_envelope and payload_dek_id are NULLed; the data-encryption-key is destroyed; the row is retained as a tombstone (ID, type, timestamps, state=`purged`).

### Backup / Restore
- Backup is a copy of the vault file. Restore is replacing the vault file. Pre-migration backups are automatic (7-day retention).

## Inputs / Outputs
- Repositories in `crates/core::persistence::repo` expose typed methods returning domain types.

## Error States
- Constraint violation
- Foreign data integrity mismatch (e.g. `payload_dek_id` referenced but missing)
- Migration failure (auto-rolled-back to pre-migration backup)

## Data Rules
- Plaintext payloads never written.
- Indexed columns never contain payload data.
- `meta` JSON validated against per-type schema before write.

## Security Rules
- All writes are countersigned (audit_log.countersignature is non-NULL).
- DB connection pragmas: `foreign_keys=ON`, `journal_mode=WAL`, `synchronous=NORMAL` or stricter; documented.

## Performance Rules
- Search by `(type, state, name LIKE ?)` sub-200 ms at 10k rows.

## Observability Rules
- Repository ops emit logs at the boundary with `op`, `target_id`, `result`, `duration_ms`.

## Required Tests
- Integration tests per repository method.
- Audit-chain verification test (happy + tamper case).
- Migration tests (forward + rollback) with audit-chain continuity.
- Cryptographic-erasure test on purge.

## Acceptance Criteria
- Schema implemented and migrations work.
- All required tests pass.
- I-1, I-3, I-8 enforcement tests pass.
