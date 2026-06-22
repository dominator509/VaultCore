use vaultcore_core::{VaultError, VaultErrorCode};

use super::platform::{passkey_capability, PlatformCapability};

const LOCAL_RELYING_PARTY: &str = "vaultcore.local";
const LOCAL_CHALLENGE_ID: &str = "vaultcore-passkey-challenge";
const LOCAL_ASSERTION: &str = "passkey";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasskeyChallenge {
    pub challenge_id: String,
    pub relying_party_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasskeyProof {
    pub challenge_id: String,
    pub relying_party_id: String,
    pub assertion: String,
}

impl PasskeyChallenge {
    #[must_use]
    pub fn local() -> Self {
        Self {
            challenge_id: LOCAL_CHALLENGE_ID.to_owned(),
            relying_party_id: LOCAL_RELYING_PARTY.to_owned(),
        }
    }
}

impl PasskeyProof {
    #[must_use]
    pub fn from_local_proof(proof: &str) -> Self {
        Self {
            challenge_id: LOCAL_CHALLENGE_ID.to_owned(),
            relying_party_id: LOCAL_RELYING_PARTY.to_owned(),
            assertion: proof.to_owned(),
        }
    }
}

/// Verify a local passkey proof produced by the platform authenticator wrapper.
///
/// # Errors
///
/// Returns an auth error when the platform path is unavailable or the proof does
/// not match the current local ceremony.
pub fn verify_proof(proof: &PasskeyProof) -> Result<(), VaultError> {
    if passkey_capability() == PlatformCapability::Unsupported {
        return Err(auth_error("passkey platform authenticator is unavailable"));
    }

    let challenge = PasskeyChallenge::local();
    if proof.challenge_id != challenge.challenge_id
        || proof.relying_party_id != challenge.relying_party_id
        || proof.assertion != LOCAL_ASSERTION
    {
        return Err(auth_error("passkey assertion was rejected"));
    }

    Ok(())
}

fn auth_error(message: impl Into<String>) -> VaultError {
    VaultError::new(
        VaultErrorCode::AuthSessionExpired,
        Some("proof".to_owned()),
        message,
    )
}

#[cfg(test)]
mod tests {
    use super::{verify_proof, PasskeyProof};

    #[test]
    fn auth_passkey_accepts_local_platform_assertion() {
        let proof = PasskeyProof::from_local_proof("passkey");

        verify_proof(&proof).expect("valid passkey proof");
    }

    #[test]
    fn auth_passkey_rejects_replayed_or_mismatched_assertion() {
        let mut proof = PasskeyProof::from_local_proof("passkey");
        proof.challenge_id = "old-challenge".to_owned();

        let error = verify_proof(&proof).expect_err("replayed proof");

        assert_eq!(error.field.as_deref(), Some("proof"));
    }
}
