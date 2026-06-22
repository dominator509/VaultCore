use vaultcore_builder::ipc::BuilderIpcSigner;
use vaultcore_core::{
    encode_frame, AuditResult, Role, SigningKeypair, TrinityRequest, VaultErrorCode,
};
use vaultcore_verifier::ipc::VerifierIpc;

#[test]
fn trinity_ipc_accepts_signed_authorize_round_trip() {
    let signing_key = SigningKeypair::from_bytes([31; 32]);
    let trusted_key = signing_key.verification_key();
    let mut builder = BuilderIpcSigner::new(signing_key, "session-1");
    let mut verifier = VerifierIpc::new(trusted_key);

    let request = TrinityRequest::AuthorizeOp {
        op: "list".to_owned(),
        target_id: None,
        role: Role::Viewer,
        session_id: "session-1".to_owned(),
    };
    let frame = builder.sign_request(request.clone()).expect("signed frame");

    let accepted = verifier.receive(&frame).expect("verified request");
    assert_eq!(accepted, request);
}

#[test]
fn trinity_ipc_rejects_replayed_counter() {
    let signing_key = SigningKeypair::from_bytes([32; 32]);
    let trusted_key = signing_key.verification_key();
    let mut builder = BuilderIpcSigner::new(signing_key, "session-1");
    let mut verifier = VerifierIpc::new(trusted_key);

    let frame = builder
        .sign_request(TrinityRequest::AppendAudit {
            op: "create".to_owned(),
            target_id: Some("01J00000000000000000000000".to_owned()),
            result: AuditResult::Allowed,
            payload_hash: "payload-hash".to_owned(),
        })
        .expect("signed frame");

    verifier.receive(&frame).expect("first receive");
    let error = verifier.receive(&frame).expect_err("replay rejected");
    assert_eq!(error.code, VaultErrorCode::IpcFailure);
}

#[test]
fn trinity_ipc_rejects_tampered_payload() {
    let signing_key = SigningKeypair::from_bytes([33; 32]);
    let trusted_key = signing_key.verification_key();
    let mut builder = BuilderIpcSigner::new(signing_key, "session-1");
    let mut verifier = VerifierIpc::new(trusted_key);

    let mut frame = builder
        .sign_frame(TrinityRequest::VerifyChain {
            head: "head-a".to_owned(),
        })
        .expect("signed frame");
    frame.payload = TrinityRequest::VerifyChain {
        head: "head-b".to_owned(),
    };
    let bytes = encode_frame(&frame).expect("encode tampered frame");

    let error = verifier
        .receive(&bytes)
        .expect_err("bad signature rejected");
    assert_eq!(error.code, VaultErrorCode::IpcFailure);
}

#[test]
fn trinity_ipc_rejects_untrusted_builder_key() {
    let signing_key = SigningKeypair::from_bytes([34; 32]);
    let untrusted_key = SigningKeypair::from_bytes([35; 32]).verification_key();
    let mut builder = BuilderIpcSigner::new(signing_key, "session-1");
    let mut verifier = VerifierIpc::new(untrusted_key);

    let frame = builder
        .sign_request(TrinityRequest::RevokeSession {
            session_id: "session-1".to_owned(),
        })
        .expect("signed frame");

    let error = verifier
        .receive(&frame)
        .expect_err("untrusted key rejected");
    assert_eq!(error.code, VaultErrorCode::IpcFailure);
}
