use rusqlite::{params, Connection, OptionalExtension, Row};
use std::str::FromStr;

use crate::{
    error::{VaultError, VaultErrorCode},
    fsm::transition,
    persistence::schema::persistence_error,
    types::{meta::ValidateMeta, LifecycleState, SecretId, SecretMeta, SecretType},
    validation::{validate_labels, validate_name},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretRecord {
    pub id: SecretId,
    pub secret_type: SecretType,
    pub name: String,
    pub labels: Vec<String>,
    pub state: LifecycleState,
    pub created_at: i64,
    pub updated_at: i64,
    pub expires_at: Option<i64>,
    pub payload_envelope: Option<Vec<u8>>,
    pub payload_dek_id: Option<String>,
    pub meta: SecretMeta,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSecret {
    pub id: SecretId,
    pub secret_type: SecretType,
    pub name: String,
    pub labels: Vec<String>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub payload_envelope: Option<Vec<u8>>,
    pub payload_dek_id: Option<String>,
    pub meta: SecretMeta,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UpdateSecret {
    pub name: Option<String>,
    pub labels: Option<Vec<String>>,
    pub state: Option<LifecycleState>,
    pub updated_at: i64,
    pub expires_at: Option<Option<i64>>,
    pub payload_envelope: Option<Option<Vec<u8>>>,
    pub payload_dek_id: Option<Option<String>>,
    pub meta: Option<SecretMeta>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ListSecretsFilter {
    pub secret_type: Option<SecretType>,
    pub state: Option<LifecycleState>,
    pub name_contains: Option<String>,
}

pub struct SecretRepo<'a> {
    connection: &'a Connection,
}

impl<'a> SecretRepo<'a> {
    #[must_use]
    pub const fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    /// Create a new draft secret row with opaque payload bytes.
    ///
    /// # Errors
    ///
    /// Returns validation errors for invalid domain fields or persistence errors for `SQLite`
    /// failures.
    pub fn create(&self, secret: &NewSecret) -> Result<SecretRecord, VaultError> {
        validate_new_secret(secret)?;
        let labels = serde_json::to_string(&secret.labels).map_err(json_error)?;
        let meta = serde_json::to_string(&secret.meta).map_err(json_error)?;

        self.connection
            .execute(
                "INSERT INTO secrets (
                    id, type, name, labels, state, created_at, updated_at, expires_at,
                    payload_envelope, payload_dek_id, meta
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6, ?7, ?8, ?9, ?10)",
                params![
                    secret.id.to_string(),
                    secret.secret_type.to_string(),
                    secret.name,
                    labels,
                    LifecycleState::Draft.to_string(),
                    secret.created_at,
                    secret.expires_at,
                    secret.payload_envelope,
                    secret.payload_dek_id,
                    meta,
                ],
            )
            .map_err(persistence_error)?;

        self.get(secret.id)?
            .ok_or_else(|| VaultError::not_found(secret.id))
    }

    /// Fetch a secret by id.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::PersistenceFailure` when the row cannot be read or decoded.
    pub fn get(&self, id: SecretId) -> Result<Option<SecretRecord>, VaultError> {
        self.connection
            .query_row(
                "SELECT id, type, name, labels, state, created_at, updated_at, expires_at,
                    payload_envelope, payload_dek_id, meta
                 FROM secrets WHERE id = ?1",
                params![id.to_string()],
                |row| Ok(row_to_secret(row)),
            )
            .optional()
            .map_err(persistence_error)?
            .transpose()
    }

    /// List secrets using indexed metadata filters.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::PersistenceFailure` when any matching row cannot be read or
    /// decoded.
    pub fn list(&self, filter: &ListSecretsFilter) -> Result<Vec<SecretRecord>, VaultError> {
        let secret_type = filter.secret_type.map(|value| value.to_string());
        let state = filter.state.map(|value| value.to_string());
        let name_like = filter
            .name_contains
            .as_ref()
            .map(|value| format!("%{value}%"));

        let mut statement = self
            .connection
            .prepare(
                "SELECT id, type, name, labels, state, created_at, updated_at, expires_at,
                    payload_envelope, payload_dek_id, meta
                 FROM secrets
                 WHERE (?1 IS NULL OR type = ?1)
                   AND (?2 IS NULL OR state = ?2)
                   AND (?3 IS NULL OR name LIKE ?3)
                 ORDER BY updated_at DESC, id ASC",
            )
            .map_err(persistence_error)?;

        let mut rows = statement
            .query(params![secret_type, state, name_like])
            .map_err(persistence_error)?;

        let mut secrets = Vec::new();
        while let Some(row) = rows.next().map_err(persistence_error)? {
            secrets.push(row_to_secret(row)?);
        }

        Ok(secrets)
    }

    /// Update a secret, validating lifecycle state transitions before writing.
    ///
    /// # Errors
    ///
    /// Returns validation/FSM errors for invalid domain changes or persistence errors for `SQLite`
    /// failures.
    pub fn update(&self, id: SecretId, update: &UpdateSecret) -> Result<SecretRecord, VaultError> {
        let current = self.get(id)?.ok_or_else(|| VaultError::not_found(id))?;
        let next = merge_update(current, update)?;
        validate_record(&next)?;

        let labels = serde_json::to_string(&next.labels).map_err(json_error)?;
        let meta = serde_json::to_string(&next.meta).map_err(json_error)?;

        self.connection
            .execute(
                "UPDATE secrets
                 SET name = ?2, labels = ?3, state = ?4, updated_at = ?5, expires_at = ?6,
                     payload_envelope = ?7, payload_dek_id = ?8, meta = ?9
                 WHERE id = ?1",
                params![
                    id.to_string(),
                    next.name,
                    labels,
                    next.state.to_string(),
                    next.updated_at,
                    next.expires_at,
                    next.payload_envelope,
                    next.payload_dek_id,
                    meta,
                ],
            )
            .map_err(persistence_error)?;

        self.get(id)?.ok_or_else(|| VaultError::not_found(id))
    }

    /// Cryptographically purge a soft-deleted secret by tombstoning payload columns.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::FsmInvalidTransition` if the current state cannot transition to
    /// `purged`, or persistence errors for `SQLite` failures.
    pub fn purge(&self, id: SecretId, updated_at: i64) -> Result<SecretRecord, VaultError> {
        let current = self.get(id)?.ok_or_else(|| VaultError::not_found(id))?;
        transition(current.state, LifecycleState::Purged)?;

        self.connection
            .execute(
                "UPDATE secrets
                 SET state = 'purged', updated_at = ?2, payload_envelope = NULL, payload_dek_id = NULL
                 WHERE id = ?1",
                params![id.to_string(), updated_at],
            )
            .map_err(persistence_error)?;

        self.get(id)?.ok_or_else(|| VaultError::not_found(id))
    }
}

fn validate_new_secret(secret: &NewSecret) -> Result<(), VaultError> {
    validate_name(&secret.name)?;
    validate_labels(&secret.labels)?;
    secret.meta.validate()?;
    validate_payload_pair(
        secret.payload_envelope.as_deref(),
        secret.payload_dek_id.as_deref(),
    )
}

fn validate_record(secret: &SecretRecord) -> Result<(), VaultError> {
    validate_name(&secret.name)?;
    validate_labels(&secret.labels)?;
    secret.meta.validate()?;
    validate_payload_pair(
        secret.payload_envelope.as_deref(),
        secret.payload_dek_id.as_deref(),
    )
}

fn validate_payload_pair(
    payload_envelope: Option<&[u8]>,
    payload_dek_id: Option<&str>,
) -> Result<(), VaultError> {
    match (payload_envelope, payload_dek_id) {
        (Some(payload), Some(dek_id)) if !payload.is_empty() && !dek_id.is_empty() => Ok(()),
        (None, None) => Ok(()),
        _ => Err(VaultError::invalid_field(
            "payload_envelope",
            "payload envelope and DEK id must be present together",
        )),
    }
}

fn merge_update(current: SecretRecord, update: &UpdateSecret) -> Result<SecretRecord, VaultError> {
    let state = update.state.unwrap_or(current.state);
    if state != current.state {
        transition(current.state, state)?;
    }

    Ok(SecretRecord {
        id: current.id,
        secret_type: current.secret_type,
        name: update.name.clone().unwrap_or(current.name),
        labels: update.labels.clone().unwrap_or(current.labels),
        state,
        created_at: current.created_at,
        updated_at: update.updated_at,
        expires_at: update.expires_at.unwrap_or(current.expires_at),
        payload_envelope: update
            .payload_envelope
            .clone()
            .unwrap_or(current.payload_envelope),
        payload_dek_id: update
            .payload_dek_id
            .clone()
            .unwrap_or(current.payload_dek_id),
        meta: update.meta.clone().unwrap_or(current.meta),
    })
}

fn row_to_secret(row: &Row<'_>) -> Result<SecretRecord, VaultError> {
    let id_raw: String = row.get(0).map_err(persistence_error)?;
    let type_raw: String = row.get(1).map_err(persistence_error)?;
    let labels_raw: String = row.get(3).map_err(persistence_error)?;
    let state_raw: String = row.get(4).map_err(persistence_error)?;
    let meta_raw: String = row.get(10).map_err(persistence_error)?;

    Ok(SecretRecord {
        id: SecretId::from_str(&id_raw)?,
        secret_type: parse_secret_type(&type_raw)?,
        name: row.get(2).map_err(persistence_error)?,
        labels: serde_json::from_str(&labels_raw).map_err(json_error)?,
        state: parse_lifecycle_state(&state_raw)?,
        created_at: row.get(5).map_err(persistence_error)?,
        updated_at: row.get(6).map_err(persistence_error)?,
        expires_at: row.get(7).map_err(persistence_error)?,
        payload_envelope: row.get(8).map_err(persistence_error)?,
        payload_dek_id: row.get(9).map_err(persistence_error)?,
        meta: serde_json::from_str(&meta_raw).map_err(json_error)?,
    })
}

fn parse_secret_type(value: &str) -> Result<SecretType, VaultError> {
    match value {
        "API_KEY" => Ok(SecretType::ApiKey),
        "LOGIN" => Ok(SecretType::Login),
        "OAUTH_APP" => Ok(SecretType::OAuthApp),
        "SSH_KEY" => Ok(SecretType::SshKey),
        "WALLET_KEY" => Ok(SecretType::WalletKey),
        "CERT" => Ok(SecretType::Cert),
        "NOTE" => Ok(SecretType::Note),
        "BLOB" => Ok(SecretType::Blob),
        _ => Err(VaultError::new(
            VaultErrorCode::PersistenceFailure,
            Some("type".to_owned()),
            "stored secret type is invalid",
        )),
    }
}

fn parse_lifecycle_state(value: &str) -> Result<LifecycleState, VaultError> {
    match value {
        "draft" => Ok(LifecycleState::Draft),
        "active" => Ok(LifecycleState::Active),
        "expiring_soon" => Ok(LifecycleState::ExpiringSoon),
        "expired" => Ok(LifecycleState::Expired),
        "rotating" => Ok(LifecycleState::Rotating),
        "archived" => Ok(LifecycleState::Archived),
        "soft_deleted" => Ok(LifecycleState::SoftDeleted),
        "purged" => Ok(LifecycleState::Purged),
        _ => Err(VaultError::new(
            VaultErrorCode::PersistenceFailure,
            Some("state".to_owned()),
            "stored lifecycle state is invalid",
        )),
    }
}

fn json_error(error: serde_json::Error) -> VaultError {
    let message = error.to_string();
    drop(error);
    VaultError::new(
        VaultErrorCode::PersistenceFailure,
        None,
        format!("JSON persistence encoding failed: {message}"),
    )
}
