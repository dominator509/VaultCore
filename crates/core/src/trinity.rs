use serde::{Deserialize, Serialize};

use crate::{
    crypto::{sign_message, verify_message, SignatureBytes, SigningKeypair, VerificationKey},
    error::{VaultError, VaultErrorCode},
    types::Role,
};

pub const TRINITY_SCHEMA_VERSION: u16 = 1;
const MAX_FRAME_BYTES: usize = 1_048_576;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditResult {
    Allowed,
    Denied,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum TrinityRequest {
    AuthorizeOp {
        op: String,
        target_id: Option<String>,
        role: Role,
        session_id: String,
    },
    AppendAudit {
        op: String,
        target_id: Option<String>,
        result: AuditResult,
        payload_hash: String,
    },
    VerifyChain {
        head: String,
    },
    IssueSession {
        auth_proof: String,
    },
    RevokeSession {
        session_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum TrinityResponse {
    Countersignature { value: Countersignature },
    Denied { reason: String },
    Ack { entry_hash: Option<String> },
    Status { value: TrinityStatus },
    SessionToken { session_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Countersignature {
    pub signer: VerificationKey,
    pub signature: SignatureBytes,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrinityStatus {
    Valid,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTrinityFrame {
    pub schema_version: u16,
    pub session_id: String,
    pub counter: u64,
    pub signing_key: VerificationKey,
    pub payload: TrinityRequest,
    pub signature: SignatureBytes,
}

#[derive(Serialize)]
struct TrinitySigningPayload<'a> {
    schema_version: u16,
    session_id: &'a str,
    counter: u64,
    payload: &'a TrinityRequest,
}

/// Build a signed Trinity frame for Builder to Verifier IPC.
///
/// # Errors
///
/// Returns `VaultErrorCode::IpcFailure` if the signing payload cannot be canonicalized.
pub fn sign_trinity_frame(
    signing_key: &SigningKeypair,
    session_id: impl Into<String>,
    counter: u64,
    payload: TrinityRequest,
) -> Result<SignedTrinityFrame, VaultError> {
    let session_id = session_id.into();
    let signing_payload = encode_signing_payload(&session_id, counter, &payload)?;
    let signature = sign_message(signing_key, &signing_payload);
    Ok(SignedTrinityFrame {
        schema_version: TRINITY_SCHEMA_VERSION,
        session_id,
        counter,
        signing_key: signing_key.verification_key(),
        payload,
        signature,
    })
}

/// Verify the signature and trusted Builder key on a Trinity frame.
///
/// # Errors
///
/// Returns `VaultErrorCode::IpcFailure` when the schema version, signing key, or
/// signature is invalid.
pub fn verify_trinity_frame(
    frame: &SignedTrinityFrame,
    trusted_builder_key: VerificationKey,
) -> Result<(), VaultError> {
    if frame.schema_version != TRINITY_SCHEMA_VERSION {
        return Err(ipc_error("unsupported Trinity schema version"));
    }
    if frame.signing_key != trusted_builder_key {
        return Err(ipc_error("untrusted Trinity signing key"));
    }
    let signing_payload = encode_signing_payload(&frame.session_id, frame.counter, &frame.payload)?;
    verify_message(frame.signing_key, &signing_payload, frame.signature)
        .map_err(|_| ipc_error("Trinity signature verification failed"))
}

/// Encode a signed Trinity frame as a big-endian u32 length prefix followed by JSON.
///
/// # Errors
///
/// Returns `VaultErrorCode::IpcFailure` if the frame cannot be serialized or is
/// larger than the configured maximum frame size.
pub fn encode_frame(frame: &SignedTrinityFrame) -> Result<Vec<u8>, VaultError> {
    let body = serde_json::to_vec(frame).map_err(|_| ipc_error("failed to serialize frame"))?;
    if body.len() > MAX_FRAME_BYTES {
        return Err(ipc_error("Trinity frame exceeds maximum size"));
    }
    let body_len = u32::try_from(body.len()).map_err(|_| ipc_error("frame too large"))?;
    let mut encoded = Vec::with_capacity(4 + body.len());
    encoded.extend_from_slice(&body_len.to_be_bytes());
    encoded.extend_from_slice(&body);
    Ok(encoded)
}

/// Decode a length-prefixed Trinity frame.
///
/// # Errors
///
/// Returns `VaultErrorCode::IpcFailure` if the frame length is malformed or the
/// JSON payload does not match the schema.
pub fn decode_frame(bytes: &[u8]) -> Result<SignedTrinityFrame, VaultError> {
    if bytes.len() < 4 {
        return Err(ipc_error("Trinity frame missing length prefix"));
    }
    let body_len = u32::from_be_bytes(
        bytes[0..4]
            .try_into()
            .map_err(|_| ipc_error("invalid length prefix"))?,
    ) as usize;
    if body_len > MAX_FRAME_BYTES {
        return Err(ipc_error("Trinity frame exceeds maximum size"));
    }
    if bytes.len() != body_len + 4 {
        return Err(ipc_error("Trinity frame length mismatch"));
    }
    serde_json::from_slice(&bytes[4..]).map_err(|_| ipc_error("invalid Trinity frame schema"))
}

fn encode_signing_payload(
    session_id: &str,
    counter: u64,
    payload: &TrinityRequest,
) -> Result<Vec<u8>, VaultError> {
    let signing_payload = TrinitySigningPayload {
        schema_version: TRINITY_SCHEMA_VERSION,
        session_id,
        counter,
        payload,
    };
    let mut encoded = Vec::new();
    ciborium::ser::into_writer(&signing_payload, &mut encoded)
        .map_err(|_| ipc_error("failed to encode signing payload"))?;
    Ok(encoded)
}

fn ipc_error(message: impl Into<String>) -> VaultError {
    VaultError::new(VaultErrorCode::IpcFailure, None, message)
}

#[cfg(test)]
mod tests {
    use super::{
        decode_frame, encode_frame, sign_trinity_frame, verify_trinity_frame, TrinityRequest,
    };
    use crate::{Role, SigningKeypair};

    #[test]
    fn signed_frame_round_trips() {
        let signing_key = SigningKeypair::from_bytes([9; 32]);
        let request = TrinityRequest::AuthorizeOp {
            op: "list".to_owned(),
            target_id: None,
            role: Role::Viewer,
            session_id: "session-a".to_owned(),
        };
        let frame = sign_trinity_frame(&signing_key, "session-a", 1, request).expect("sign");
        let encoded = encode_frame(&frame).expect("encode");
        let decoded = decode_frame(&encoded).expect("decode");

        verify_trinity_frame(&decoded, signing_key.verification_key()).expect("verify");
    }

    #[test]
    fn modified_frame_rejects_signature() {
        let signing_key = SigningKeypair::from_bytes([9; 32]);
        let mut frame = sign_trinity_frame(
            &signing_key,
            "session-a",
            1,
            TrinityRequest::VerifyChain {
                head: "head-a".to_owned(),
            },
        )
        .expect("sign");
        frame.payload = TrinityRequest::VerifyChain {
            head: "head-b".to_owned(),
        };

        let error =
            verify_trinity_frame(&frame, signing_key.verification_key()).expect_err("reject");
        assert_eq!(error.code.as_str(), "VC-IPC-001");
    }
}
