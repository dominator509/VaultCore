use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use zeroize::ZeroizeOnDrop;

use crate::{
    crypto::aead::crypto_error,
    error::{VaultError, VaultErrorCode},
};

pub const SIGNATURE_BYTES: usize = 64;
pub const VERIFICATION_KEY_BYTES: usize = 32;

#[derive(Clone, ZeroizeOnDrop)]
pub struct SigningKeypair(SigningKey);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationKey([u8; VERIFICATION_KEY_BYTES]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignatureBytes([u8; SIGNATURE_BYTES]);

impl SigningKeypair {
    #[must_use]
    pub fn generate() -> Self {
        Self(SigningKey::generate(&mut OsRng))
    }

    #[must_use]
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(SigningKey::from_bytes(&bytes))
    }

    #[must_use]
    pub fn verification_key(&self) -> VerificationKey {
        VerificationKey(self.0.verifying_key().to_bytes())
    }
}

impl VerificationKey {
    /// Build a verification key from raw Ed25519 bytes.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::CryptoFailure` when the bytes are not a valid key.
    pub fn from_bytes(bytes: [u8; VERIFICATION_KEY_BYTES]) -> Result<Self, VaultError> {
        VerifyingKey::from_bytes(&bytes)
            .map(|_| Self(bytes))
            .map_err(|_| crypto_error("invalid Ed25519 verification key"))
    }

    #[must_use]
    pub const fn as_bytes(self) -> [u8; VERIFICATION_KEY_BYTES] {
        self.0
    }
}

impl SignatureBytes {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; SIGNATURE_BYTES]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(self) -> [u8; SIGNATURE_BYTES] {
        self.0
    }
}

impl Serialize for SignatureBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for SignatureBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        let bytes: [u8; SIGNATURE_BYTES] = bytes
            .try_into()
            .map_err(|_| D::Error::custom("signature must be 64 bytes"))?;
        Ok(Self(bytes))
    }
}

#[must_use]
pub fn sign_message(signing_key: &SigningKeypair, message: &[u8]) -> SignatureBytes {
    SignatureBytes(signing_key.0.sign(message).to_bytes())
}

/// Verify an Ed25519 signature.
///
/// # Errors
///
/// Returns `VaultErrorCode::CryptoFailure` when the signature bytes are invalid or the
/// signature does not verify for the message/key pair.
pub fn verify_message(
    verification_key: VerificationKey,
    message: &[u8],
    signature: SignatureBytes,
) -> Result<(), VaultError> {
    let key = VerifyingKey::from_bytes(&verification_key.0)
        .map_err(|_| crypto_error("invalid Ed25519 verification key"))?;
    let signature = Signature::from_bytes(&signature.0);
    key.verify(message, &signature).map_err(|_| {
        VaultError::new(
            VaultErrorCode::CryptoFailure,
            None,
            "Ed25519 signature verification failed",
        )
    })
}

impl std::fmt::Debug for SigningKeypair {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SigningKeypair([redacted])")
    }
}

#[cfg(test)]
mod tests {
    use super::{sign_message, verify_message, SigningKeypair};

    #[test]
    fn ed25519_signature_round_trips() {
        let signing_key = SigningKeypair::from_bytes([3; 32]);
        let verification_key = signing_key.verification_key();
        let signature = sign_message(&signing_key, b"message");

        verify_message(verification_key, b"message", signature).expect("verify");
    }

    #[test]
    fn ed25519_rejects_modified_message() {
        let signing_key = SigningKeypair::from_bytes([3; 32]);
        let verification_key = signing_key.verification_key();
        let signature = sign_message(&signing_key, b"message");

        let error = verify_message(verification_key, b"modified", signature).expect_err("reject");
        assert_eq!(error.code.as_str(), "VC-CRYPTO-001");
    }
}
