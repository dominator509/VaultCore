use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::{
    error::{VaultError, VaultErrorCode},
    persistence::{
        audit_chain::{
            compute_entry_hash, compute_payload_hash, genesis_hash, hash_from_slice,
            AuditChainEntry,
        },
        schema::persistence_error,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendAuditEntry {
    pub ts: i64,
    pub actor: String,
    pub op: String,
    pub target_id: Option<String>,
    pub result: String,
    pub countersignature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub seq: i64,
    pub ts: i64,
    pub actor: String,
    pub op: String,
    pub target_id: Option<String>,
    pub result: String,
    pub prior_hash: Vec<u8>,
    pub payload_hash: Vec<u8>,
    pub entry_hash: Vec<u8>,
    pub countersignature: Vec<u8>,
}

pub struct AuditRepo<'a> {
    connection: &'a Connection,
    genesis_hash: [u8; 32],
}

impl<'a> AuditRepo<'a> {
    #[must_use]
    pub const fn new(connection: &'a Connection, genesis_hash: [u8; 32]) -> Self {
        Self {
            connection,
            genesis_hash,
        }
    }

    #[must_use]
    pub fn with_zero_genesis(connection: &'a Connection) -> Self {
        Self::new(connection, genesis_hash())
    }

    /// Append an audit entry and return its entry hash.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::ValidationInvalidField` for malformed audit fields,
    /// `VaultErrorCode::AuditChainAnomaly` for hash issues, or persistence errors for `SQLite`
    /// failures.
    pub fn append(&self, entry: &AppendAuditEntry) -> Result<[u8; 32], VaultError> {
        validate_append(entry)?;
        let prior_hash = self.chain_head()?;
        let payload_hash = compute_payload_hash(&AuditChainEntry {
            ts: entry.ts,
            actor: &entry.actor,
            op: &entry.op,
            target_id: entry.target_id.as_deref(),
            result: &entry.result,
        })?;
        let entry_hash = compute_entry_hash(&prior_hash, &payload_hash);

        self.connection
            .execute(
                "INSERT INTO audit_log (
                    ts, actor, op, target_id, result, prior_hash, payload_hash, entry_hash,
                    countersignature
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    entry.ts,
                    entry.actor,
                    entry.op,
                    entry.target_id,
                    entry.result,
                    prior_hash.as_slice(),
                    payload_hash.as_slice(),
                    entry_hash.as_slice(),
                    entry.countersignature,
                ],
            )
            .map_err(persistence_error)?;

        Ok(entry_hash)
    }

    /// Return all audit rows in chain order.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::PersistenceFailure` when rows cannot be read.
    pub fn list(&self) -> Result<Vec<AuditEntry>, VaultError> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT seq, ts, actor, op, target_id, result, prior_hash, payload_hash,
                    entry_hash, countersignature
                 FROM audit_log ORDER BY seq ASC",
            )
            .map_err(persistence_error)?;
        let rows = statement
            .query_map([], row_to_audit_entry)
            .map_err(persistence_error)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(persistence_error)
    }

    /// Return the current audit chain head.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::AuditChainAnomaly` when the stored head is not a 32-byte hash,
    /// or a persistence error when the database cannot be read.
    pub fn chain_head(&self) -> Result<[u8; 32], VaultError> {
        let head: Option<Vec<u8>> = self
            .connection
            .query_row(
                "SELECT entry_hash FROM audit_log ORDER BY seq DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(persistence_error)?;

        match head {
            Some(value) => hash_from_slice(&value),
            None => Ok(self.genesis_hash),
        }
    }
}

fn row_to_audit_entry(row: &Row<'_>) -> Result<AuditEntry, rusqlite::Error> {
    Ok(AuditEntry {
        seq: row.get(0)?,
        ts: row.get(1)?,
        actor: row.get(2)?,
        op: row.get(3)?,
        target_id: row.get(4)?,
        result: row.get(5)?,
        prior_hash: row.get(6)?,
        payload_hash: row.get(7)?,
        entry_hash: row.get(8)?,
        countersignature: row.get(9)?,
    })
}

fn validate_append(entry: &AppendAuditEntry) -> Result<(), VaultError> {
    if entry.actor.is_empty() {
        return Err(VaultError::invalid_field("actor", "actor is required"));
    }
    if entry.op.is_empty() {
        return Err(VaultError::invalid_field("op", "operation is required"));
    }
    if !matches!(entry.result.as_str(), "ok" | "denied" | "error") {
        return Err(VaultError::invalid_field(
            "result",
            "result must be ok, denied, or error",
        ));
    }
    if entry.countersignature.is_empty() {
        return Err(VaultError::new(
            VaultErrorCode::ValidationInvalidField,
            Some("countersignature".to_owned()),
            "countersignature is required",
        ));
    }
    Ok(())
}
