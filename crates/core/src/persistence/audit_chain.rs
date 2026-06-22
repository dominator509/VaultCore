use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    error::{VaultError, VaultErrorCode},
    persistence::repo::AuditEntry,
};

pub const HASH_BYTES: usize = 32;

#[must_use]
pub fn genesis_hash() -> [u8; HASH_BYTES] {
    [0; HASH_BYTES]
}

/// Compute a payload hash over deterministic CBOR bytes for an audited payload.
///
/// # Errors
///
/// Returns `VaultErrorCode::AuditChainAnomaly` when the payload cannot be serialized.
pub fn compute_payload_hash<T: Serialize>(payload: &T) -> Result<[u8; HASH_BYTES], VaultError> {
    let mut encoded = Vec::new();
    ciborium::into_writer(payload, &mut encoded).map_err(|_| audit_error("payload hash failed"))?;
    Ok(hash_bytes(&encoded))
}

#[must_use]
pub fn compute_entry_hash(
    prior_hash: &[u8; HASH_BYTES],
    payload_hash: &[u8; HASH_BYTES],
) -> [u8; HASH_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(prior_hash);
    hasher.update(payload_hash);
    hasher.finalize().into()
}

/// Verify an ordered audit chain.
///
/// # Errors
///
/// Returns `VaultErrorCode::AuditChainAnomaly` when any link has an unexpected prior,
/// payload, or entry hash.
pub fn verify_chain(
    entries: &[AuditEntry],
    genesis: [u8; HASH_BYTES],
) -> Result<VerifiedChain, VaultError> {
    let mut expected_prior = genesis;

    for entry in entries {
        let prior_hash = hash_from_slice(&entry.prior_hash)?;
        if prior_hash != expected_prior {
            return Err(audit_error("audit prior hash mismatch"));
        }

        let expected_payload_hash = compute_payload_hash(&AuditChainEntry::from(entry))?;
        let payload_hash = hash_from_slice(&entry.payload_hash)?;
        if payload_hash != expected_payload_hash {
            return Err(audit_error("audit payload hash mismatch"));
        }

        let expected_entry_hash = compute_entry_hash(&prior_hash, &payload_hash);
        let entry_hash = hash_from_slice(&entry.entry_hash)?;
        if entry_hash != expected_entry_hash {
            return Err(audit_error("audit entry hash mismatch"));
        }

        expected_prior = entry_hash;
    }

    Ok(VerifiedChain {
        entries: entries.len(),
        head: expected_prior,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedChain {
    pub entries: usize,
    pub head: [u8; HASH_BYTES],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditChainEntry<'a> {
    pub ts: i64,
    pub actor: &'a str,
    pub op: &'a str,
    pub target_id: Option<&'a str>,
    pub result: &'a str,
}

impl<'a> From<&'a AuditEntry> for AuditChainEntry<'a> {
    fn from(entry: &'a AuditEntry) -> Self {
        Self {
            ts: entry.ts,
            actor: &entry.actor,
            op: &entry.op,
            target_id: entry.target_id.as_deref(),
            result: &entry.result,
        }
    }
}

pub(crate) fn hash_from_slice(value: &[u8]) -> Result<[u8; HASH_BYTES], VaultError> {
    value
        .try_into()
        .map_err(|_| audit_error("audit hash must be 32 bytes"))
}

pub(crate) fn audit_error(message: impl Into<String>) -> VaultError {
    VaultError::new(VaultErrorCode::AuditChainAnomaly, None, message)
}

fn hash_bytes(value: &[u8]) -> [u8; HASH_BYTES] {
    Sha256::digest(value).into()
}

#[cfg(test)]
mod tests {
    use super::{compute_entry_hash, compute_payload_hash, genesis_hash, AuditChainEntry};

    #[test]
    fn entry_hash_uses_prior_and_payload_hash() {
        let payload = AuditChainEntry {
            ts: 1,
            actor: "Owner:session",
            op: "create_secret",
            target_id: None,
            result: "ok",
        };
        let payload_hash = compute_payload_hash(&payload).expect("hash payload");
        let entry_hash = compute_entry_hash(&genesis_hash(), &payload_hash);

        assert_ne!(entry_hash, genesis_hash());
        assert_ne!(entry_hash, payload_hash);
    }
}
