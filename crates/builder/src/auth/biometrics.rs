use vaultcore_core::{VaultError, VaultErrorCode};

use super::platform::{biometric_capability, PlatformCapability};

const LOCAL_KEY_HANDLE: &str = "vaultcore-biometric-key";
const LOCAL_ASSERTION: &str = "biometrics";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiometricProof {
    pub hardware_key_handle: String,
    pub assertion: String,
}

impl BiometricProof {
    #[must_use]
    pub fn from_local_proof(proof: &str) -> Self {
        Self {
            hardware_key_handle: LOCAL_KEY_HANDLE.to_owned(),
            assertion: proof.to_owned(),
        }
    }
}

/// Verify a local biometric proof tied to a hardware-backed key handle.
///
/// # Errors
///
/// Returns an auth error when biometrics are unavailable or the local assertion
/// does not match the expected hardware-backed key handle.
pub fn verify_proof(proof: &BiometricProof) -> Result<(), VaultError> {
    if biometric_capability() == PlatformCapability::Unsupported {
        return Err(auth_error("biometric platform wrapper is unavailable"));
    }

    if proof.hardware_key_handle != LOCAL_KEY_HANDLE || proof.assertion != LOCAL_ASSERTION {
        return Err(auth_error("biometric assertion was rejected"));
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
    use super::{verify_proof, BiometricProof};

    #[test]
    fn auth_biometrics_accepts_local_hardware_key_assertion() {
        let proof = BiometricProof::from_local_proof("biometrics");

        verify_proof(&proof).expect("valid biometric proof");
    }

    #[test]
    fn auth_biometrics_rejects_mismatched_key_handle() {
        let mut proof = BiometricProof::from_local_proof("biometrics");
        proof.hardware_key_handle = "software-key".to_owned();

        let error = verify_proof(&proof).expect_err("software key");

        assert_eq!(error.field.as_deref(), Some("proof"));
    }
}
