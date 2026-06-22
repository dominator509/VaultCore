pub mod biometrics;
pub mod passkey;
pub mod passphrase;
pub mod platform;

use vaultcore_core::{crypto::KdfSalt, VaultError};

use crate::session::AuthProof;

/// Validate an unlock proof before asking the Verifier to issue a session.
///
/// # Errors
///
/// Returns validation or auth errors for unknown methods or invalid proofs.
pub fn verify_unlock_proof(auth: &AuthProof) -> Result<(), VaultError> {
    match auth.method.as_str() {
        "passphrase" => {
            let passphrase =
                vaultcore_core::crypto::SealedBytes::new(auth.proof.as_bytes().to_vec());
            let salt = KdfSalt::from_bytes(*b"vaultcoreunlock!");
            let _key =
                passphrase::derive_key(&passphrase, &salt, passphrase::Argon2idParams::minimum())?;
            Ok(())
        }
        "passkey" => passkey::verify_proof(&passkey::PasskeyProof::from_local_proof(&auth.proof)),
        "biometrics" => {
            biometrics::verify_proof(&biometrics::BiometricProof::from_local_proof(&auth.proof))
        }
        _ => Err(VaultError::invalid_field(
            "method",
            "unknown authentication method",
        )),
    }
}
