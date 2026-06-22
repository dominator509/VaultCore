use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use rand_core::{OsRng, RngCore};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    crypto::SealedBytes,
    error::{VaultError, VaultErrorCode},
};

pub const AEAD_KEY_BYTES: usize = 32;
pub const AEAD_NONCE_BYTES: usize = 24;

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct AeadKey([u8; AEAD_KEY_BYTES]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AeadNonce([u8; AEAD_NONCE_BYTES]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ciphertext {
    pub nonce: AeadNonce,
    pub bytes: Vec<u8>,
}

impl AeadKey {
    #[must_use]
    pub fn generate() -> Self {
        let mut key = [0; AEAD_KEY_BYTES];
        OsRng.fill_bytes(&mut key);
        Self(key)
    }

    #[must_use]
    pub const fn from_bytes(bytes: [u8; AEAD_KEY_BYTES]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn expose_for_builder_only(&self) -> &[u8; AEAD_KEY_BYTES] {
        &self.0
    }
}

impl AeadNonce {
    #[must_use]
    pub fn generate() -> Self {
        let mut nonce = [0; AEAD_NONCE_BYTES];
        OsRng.fill_bytes(&mut nonce);
        Self(nonce)
    }

    #[must_use]
    pub const fn from_bytes(bytes: [u8; AEAD_NONCE_BYTES]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(self) -> [u8; AEAD_NONCE_BYTES] {
        self.0
    }
}

/// Encrypt plaintext payload bytes with XChaCha20-Poly1305.
///
/// # Errors
///
/// Returns `VaultErrorCode::CryptoFailure` if the AEAD implementation rejects encryption.
pub fn encrypt_payload(
    key: &AeadKey,
    plaintext: &SealedBytes,
    aad: &[u8],
) -> Result<Ciphertext, VaultError> {
    let nonce = AeadNonce::generate();
    encrypt_payload_with_nonce(key, nonce, plaintext, aad)
}

/// Encrypt plaintext with a caller-provided nonce for deterministic test vectors.
///
/// # Errors
///
/// Returns `VaultErrorCode::CryptoFailure` if the AEAD implementation rejects encryption.
pub fn encrypt_payload_with_nonce(
    key: &AeadKey,
    nonce: AeadNonce,
    plaintext: &SealedBytes,
    aad: &[u8],
) -> Result<Ciphertext, VaultError> {
    let cipher = XChaCha20Poly1305::new(key.expose_for_builder_only().into());
    let bytes = cipher
        .encrypt(
            XNonce::from_slice(&nonce.0),
            chacha20poly1305::aead::Payload {
                msg: plaintext.expose_for_builder_only(),
                aad,
            },
        )
        .map_err(|_| crypto_error("AEAD encryption failed"))?;
    Ok(Ciphertext { nonce, bytes })
}

/// Decrypt ciphertext payload bytes with XChaCha20-Poly1305.
///
/// # Errors
///
/// Returns `VaultErrorCode::CryptoFailure` if authentication or decryption fails.
pub fn decrypt_payload(
    key: &AeadKey,
    ciphertext: &Ciphertext,
    aad: &[u8],
) -> Result<SealedBytes, VaultError> {
    let cipher = XChaCha20Poly1305::new(key.expose_for_builder_only().into());
    let plaintext = cipher
        .decrypt(
            XNonce::from_slice(&ciphertext.nonce.0),
            chacha20poly1305::aead::Payload {
                msg: &ciphertext.bytes,
                aad,
            },
        )
        .map_err(|_| crypto_error("AEAD authentication failed"))?;
    Ok(SealedBytes::new(plaintext))
}

pub(crate) fn crypto_error(message: impl Into<String>) -> VaultError {
    VaultError::new(VaultErrorCode::CryptoFailure, None, message)
}

impl std::fmt::Debug for AeadKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("AeadKey([redacted])")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        decrypt_payload, encrypt_payload_with_nonce, AeadKey, AeadNonce, AEAD_KEY_BYTES,
        AEAD_NONCE_BYTES,
    };
    use crate::crypto::SealedBytes;

    #[test]
    fn aead_round_trips_with_deterministic_vector() {
        let key = AeadKey::from_bytes([7; AEAD_KEY_BYTES]);
        let nonce = AeadNonce::from_bytes([9; AEAD_NONCE_BYTES]);
        let plaintext = SealedBytes::new(b"payload".to_vec());
        let ciphertext =
            encrypt_payload_with_nonce(&key, nonce, &plaintext, b"secret-id").expect("encrypt");
        assert_ne!(ciphertext.bytes, b"payload");

        let decrypted = decrypt_payload(&key, &ciphertext, b"secret-id").expect("decrypt");
        assert_eq!(decrypted.expose_for_builder_only(), b"payload");
    }

    #[test]
    fn aead_rejects_bad_aad() {
        let key = AeadKey::from_bytes([7; AEAD_KEY_BYTES]);
        let nonce = AeadNonce::from_bytes([9; AEAD_NONCE_BYTES]);
        let plaintext = SealedBytes::new(b"payload".to_vec());
        let ciphertext =
            encrypt_payload_with_nonce(&key, nonce, &plaintext, b"secret-id").expect("encrypt");

        let error = decrypt_payload(&key, &ciphertext, b"other-id").expect_err("bad aad");
        assert_eq!(error.code.as_str(), "VC-CRYPTO-001");
    }
}
