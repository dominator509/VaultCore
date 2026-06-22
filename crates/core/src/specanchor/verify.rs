use crate::{
    crypto::{sign_message, verify_message, SigningKeypair},
    error::{VaultError, VaultErrorCode},
    specanchor::schema::{SignedSpecAnchor, SpecAnchor},
};

/// Sign a `SpecAnchor` payload with an offline signing key.
///
/// # Errors
///
/// Returns `VaultErrorCode::SpecAnchorFailure` if the payload cannot be canonicalized.
pub fn sign_specanchor(
    payload: SpecAnchor,
    signing_key: &SigningKeypair,
) -> Result<SignedSpecAnchor, VaultError> {
    let bytes = canonical_payload_bytes(&payload)?;
    Ok(SignedSpecAnchor {
        payload,
        signing_key: signing_key.verification_key(),
        signature: sign_message(signing_key, &bytes),
    })
}

/// Verify a signed `SpecAnchor` envelope.
///
/// # Errors
///
/// Returns `VaultErrorCode::SpecAnchorFailure` if canonicalization or signature verification
/// fails.
pub fn verify_signed_specanchor(anchor: &SignedSpecAnchor) -> Result<(), VaultError> {
    let bytes = canonical_payload_bytes(&anchor.payload)?;
    verify_message(anchor.signing_key, &bytes, anchor.signature).map_err(|_| {
        VaultError::new(
            VaultErrorCode::SpecAnchorFailure,
            None,
            "SpecAnchor signature verification failed",
        )
    })
}

/// Encode a signed `SpecAnchor` as redaction-safe JSON bytes.
///
/// # Errors
///
/// Returns `VaultErrorCode::SpecAnchorFailure` when JSON serialization fails.
pub fn encode_signed_specanchor(anchor: &SignedSpecAnchor) -> Result<Vec<u8>, VaultError> {
    serde_json::to_vec_pretty(anchor).map_err(|_| specanchor_error("SpecAnchor encoding failed"))
}

/// Decode and verify a signed `SpecAnchor` from JSON bytes.
///
/// # Errors
///
/// Returns `VaultErrorCode::SpecAnchorFailure` when decoding or signature verification fails.
pub fn decode_signed_specanchor(bytes: &[u8]) -> Result<SignedSpecAnchor, VaultError> {
    let anchor: SignedSpecAnchor = serde_json::from_slice(bytes)
        .map_err(|_| specanchor_error("SpecAnchor decoding failed"))?;
    verify_signed_specanchor(&anchor)?;
    Ok(anchor)
}

fn canonical_payload_bytes(payload: &SpecAnchor) -> Result<Vec<u8>, VaultError> {
    let mut bytes = Vec::new();
    ciborium::into_writer(payload, &mut bytes)
        .map_err(|_| specanchor_error("SpecAnchor canonicalization failed"))?;
    Ok(bytes)
}

fn specanchor_error(message: impl Into<String>) -> VaultError {
    VaultError::new(VaultErrorCode::SpecAnchorFailure, None, message)
}

#[cfg(test)]
mod tests {
    use crate::{
        crypto::{SigningKeypair, VerificationKey},
        specanchor::{
            decode_signed_specanchor, encode_signed_specanchor, sign_specanchor,
            verify_signed_specanchor, SpecAnchor,
        },
    };

    fn fixture() -> (SigningKeypair, SpecAnchor) {
        let signing_key = SigningKeypair::from_bytes([11; 32]);
        let builder_key = VerificationKey::from_bytes([22; 32]).expect("verification key");
        (signing_key, SpecAnchor::development_default(builder_key))
    }

    #[test]
    fn signed_specanchor_round_trips() {
        let (signing_key, payload) = fixture();
        let signed = sign_specanchor(payload, &signing_key).expect("sign");
        verify_signed_specanchor(&signed).expect("verify");
        let encoded = encode_signed_specanchor(&signed).expect("encode");
        let decoded = decode_signed_specanchor(&encoded).expect("decode");
        assert_eq!(decoded, signed);
    }

    #[test]
    fn signed_specanchor_rejects_tampering() {
        let (signing_key, payload) = fixture();
        let mut signed = sign_specanchor(payload, &signing_key).expect("sign");
        signed.payload.policy_version = "tampered".to_owned();
        let error = verify_signed_specanchor(&signed).expect_err("tamper rejected");
        assert_eq!(error.code.as_str(), "VC-SPEC-001");
    }
}
