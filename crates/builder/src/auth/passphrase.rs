use argon2::{Algorithm, Argon2, Params, Version};
use vaultcore_core::{
    crypto::{AeadKey, KdfSalt, SealedBytes, AEAD_KEY_BYTES},
    VaultError, VaultErrorCode,
};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const MIN_MEMORY_KIB: u32 = 64 * 1024;
pub const MIN_ITERATIONS: u32 = 3;
pub const MIN_PARALLELISM: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Argon2idParams {
    pub memory_kib: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
struct DerivedKeyBytes([u8; AEAD_KEY_BYTES]);

impl Argon2idParams {
    #[must_use]
    pub const fn minimum() -> Self {
        Self {
            memory_kib: MIN_MEMORY_KIB,
            iterations: MIN_ITERATIONS,
            parallelism: MIN_PARALLELISM,
        }
    }

    #[must_use]
    pub const fn tuned_for_install(memory_kib: u32, iterations: u32, parallelism: u32) -> Self {
        Self {
            memory_kib: floor(memory_kib, MIN_MEMORY_KIB),
            iterations: floor(iterations, MIN_ITERATIONS),
            parallelism: floor(parallelism, MIN_PARALLELISM),
        }
    }

    #[must_use]
    pub const fn meets_minimums(self) -> bool {
        self.memory_kib >= MIN_MEMORY_KIB
            && self.iterations >= MIN_ITERATIONS
            && self.parallelism >= MIN_PARALLELISM
    }

    fn to_argon2_params(self) -> Result<Params, VaultError> {
        if !self.meets_minimums() {
            return Err(VaultError::invalid_field(
                "argon2id_params",
                "Argon2id parameters are below SPEC-005 minimums",
            ));
        }

        Params::new(
            self.memory_kib,
            self.iterations,
            self.parallelism,
            Some(AEAD_KEY_BYTES),
        )
        .map_err(|_| {
            VaultError::new(
                VaultErrorCode::CryptoFailure,
                Some("argon2id_params".to_owned()),
                "Argon2id parameters are invalid",
            )
        })
    }
}

/// Derive a local master-key wrapping key from the passphrase fallback path.
///
/// # Errors
///
/// Returns a validation error when the passphrase is empty or a crypto error when
/// the Argon2id implementation rejects the configured parameters.
pub fn derive_key(
    passphrase: &SealedBytes,
    salt: &KdfSalt,
    params: Argon2idParams,
) -> Result<AeadKey, VaultError> {
    if passphrase.is_empty() {
        return Err(VaultError::invalid_field(
            "passphrase",
            "passphrase is required",
        ));
    }

    let argon2_params = params.to_argon2_params()?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params);
    let mut output = DerivedKeyBytes([0; AEAD_KEY_BYTES]);
    argon2
        .hash_password_into(
            passphrase.expose_for_builder_only(),
            &salt.as_bytes(),
            &mut output.0,
        )
        .map_err(|_| {
            VaultError::new(
                VaultErrorCode::CryptoFailure,
                None,
                "Argon2id passphrase derivation failed",
            )
        })?;

    Ok(AeadKey::from_bytes(output.0))
}

const fn floor(value: u32, minimum: u32) -> u32 {
    if value < minimum {
        minimum
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::{derive_key, Argon2idParams, MIN_ITERATIONS, MIN_MEMORY_KIB, MIN_PARALLELISM};
    use vaultcore_core::crypto::{KdfSalt, SealedBytes};

    #[test]
    fn auth_passphrase_install_params_never_drop_below_spec_floor() {
        let params = Argon2idParams::tuned_for_install(8 * 1024, 1, 0);

        assert_eq!(params.memory_kib, MIN_MEMORY_KIB);
        assert_eq!(params.iterations, MIN_ITERATIONS);
        assert_eq!(params.parallelism, MIN_PARALLELISM);
        assert!(params.meets_minimums());
    }

    #[test]
    fn auth_passphrase_rejects_below_minimum_direct_params() {
        let passphrase = SealedBytes::new(b"correct horse battery staple".to_vec());
        let salt = KdfSalt::from_bytes([3; 16]);
        let params = Argon2idParams {
            memory_kib: 1024,
            iterations: MIN_ITERATIONS,
            parallelism: MIN_PARALLELISM,
        };

        let error = derive_key(&passphrase, &salt, params).expect_err("below minimum");

        assert_eq!(error.field.as_deref(), Some("argon2id_params"));
    }

    #[test]
    fn auth_passphrase_rejects_empty_passphrase() {
        let passphrase = SealedBytes::new(Vec::new());
        let salt = KdfSalt::from_bytes([3; 16]);

        let error =
            derive_key(&passphrase, &salt, Argon2idParams::minimum()).expect_err("empty proof");

        assert_eq!(error.field.as_deref(), Some("passphrase"));
    }

    #[test]
    fn auth_passphrase_known_answer_vector_uses_minimum_params() {
        let passphrase = SealedBytes::new(b"vaultcore-passphrase-kat".to_vec());
        let salt = KdfSalt::from_bytes(*b"vaultcore-kdf-06");

        let key = derive_key(&passphrase, &salt, Argon2idParams::minimum()).expect("derive");

        assert_eq!(
            key.expose_for_builder_only(),
            &[
                34u8, 195, 54, 107, 225, 47, 235, 95, 32, 34, 170, 244, 45, 38, 165, 190, 196, 171,
                237, 206, 112, 37, 176, 248, 147, 99, 37, 98, 239, 4, 27, 50
            ]
        );
    }
}
