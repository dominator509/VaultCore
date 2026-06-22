use argon2::Argon2;
use hkdf::Hkdf;
use rand_core::{OsRng, RngCore};
use sha2::Sha512;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    crypto::{aead::crypto_error, AeadKey, SealedBytes},
    error::VaultError,
};

pub const KDF_SALT_BYTES: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KdfSalt([u8; KDF_SALT_BYTES]);

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
struct DerivedBytes([u8; 32]);

impl KdfSalt {
    #[must_use]
    pub fn generate() -> Self {
        let mut salt = [0; KDF_SALT_BYTES];
        OsRng.fill_bytes(&mut salt);
        Self(salt)
    }

    #[must_use]
    pub const fn from_bytes(bytes: [u8; KDF_SALT_BYTES]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(self) -> [u8; KDF_SALT_BYTES] {
        self.0
    }
}

/// Derive an AEAD key using HKDF-SHA-512.
///
/// # Errors
///
/// Returns `VaultErrorCode::CryptoFailure` when HKDF expansion fails.
pub fn derive_hkdf_sha512(
    input_key_material: &SealedBytes,
    salt: &KdfSalt,
    info: &[u8],
) -> Result<AeadKey, VaultError> {
    let hkdf = Hkdf::<Sha512>::new(Some(&salt.0), input_key_material.expose_for_builder_only());
    let mut output = DerivedBytes([0; 32]);
    hkdf.expand(info, &mut output.0)
        .map_err(|_| crypto_error("HKDF-SHA-512 expansion failed"))?;
    let key = AeadKey::from_bytes(output.0);
    Ok(key)
}

/// Derive an AEAD key using Argon2id.
///
/// # Errors
///
/// Returns `VaultErrorCode::CryptoFailure` when Argon2id derivation fails.
pub fn derive_argon2id_key(
    passphrase: &SealedBytes,
    salt: &KdfSalt,
) -> Result<AeadKey, VaultError> {
    let mut output = DerivedBytes([0; 32]);
    Argon2::default()
        .hash_password_into(passphrase.expose_for_builder_only(), &salt.0, &mut output.0)
        .map_err(|_| crypto_error("Argon2id derivation failed"))?;
    Ok(AeadKey::from_bytes(output.0))
}

#[cfg(test)]
mod tests {
    use super::{derive_argon2id_key, derive_hkdf_sha512, KdfSalt};
    use crate::crypto::SealedBytes;

    #[test]
    fn hkdf_is_deterministic_for_same_inputs() {
        let ikm = SealedBytes::new(b"input key material".to_vec());
        let salt = KdfSalt::from_bytes([1; 16]);
        let first = derive_hkdf_sha512(&ikm, &salt, b"vaultcore").expect("derive");
        let second = derive_hkdf_sha512(&ikm, &salt, b"vaultcore").expect("derive");
        assert_eq!(
            first.expose_for_builder_only(),
            second.expose_for_builder_only()
        );
    }

    #[test]
    fn argon2id_is_deterministic_for_same_inputs() {
        let passphrase = SealedBytes::new(b"correct horse".to_vec());
        let salt = KdfSalt::from_bytes([2; 16]);
        let first = derive_argon2id_key(&passphrase, &salt).expect("derive");
        let second = derive_argon2id_key(&passphrase, &salt).expect("derive");
        assert_eq!(
            first.expose_for_builder_only(),
            second.expose_for_builder_only()
        );
    }
}
