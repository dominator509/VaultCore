use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use rusqlite::{params, Connection};
use tempfile::TempDir;
use vaultcore_builder::{
    api::SecretInput,
    service::{BuilderService, VerifierGateway},
};
use vaultcore_core::{
    decode_frame, encode_frame, genesis_hash, migrate, sign_specanchor, verify_chain,
    verify_signed_specanchor, ApiKeyMeta, AppendAuditEntry, AuditRepo, AuditResult, NewSecret,
    Role, SecretId, SecretMeta, SecretRepo, SecretType, SigningKeypair, SpecAnchor, TrinityRequest,
    VaultError, VaultErrorCode,
};
use vaultcore_verifier::policy;

#[test]
fn invariant_suite_is_wired() {
    assert_eq!(vaultcore_core::VERSION, env!("CARGO_PKG_VERSION"));
}

#[test]
fn i1_no_plaintext_payload_marker_at_rest() {
    let (_directory, connection, db_path) = temp_connection();
    let repo = SecretRepo::new(&connection);
    let secret = new_secret(b"ciphertext-envelope".to_vec());

    repo.create(&secret).expect("create secret");
    drop(connection);

    let bytes = fs::read(db_path).expect("read sqlite file");
    assert!(!bytes
        .windows(b"plaintext-secret-marker".len())
        .any(|window| window == b"plaintext-secret-marker"));
}

#[test]
fn i2_builder_reveal_returns_jit_payload_handle_not_plaintext() {
    let mut service = BuilderService::default();

    let response = service
        .reveal("local-1", "breakglass review", Role::Viewer)
        .expect("reveal handle");

    assert_eq!(response.ttl_ms, 30_000);
    assert!(response.payload_handle.starts_with("payload://"));
    assert!(!format!("{response:?}").contains("plaintext-secret-marker"));
}

#[test]
fn i3_metadata_indexes_do_not_include_payload_columns() {
    let (_directory, connection, _db_path) = temp_connection();

    let indexes = sqlite_secret_indexes(&connection);

    assert!(indexes.iter().any(|name| name.contains("type")));
    assert!(indexes.iter().any(|name| name.contains("state")));
    assert!(indexes.iter().any(|name| name.contains("name")));
    assert!(!indexes.iter().any(|name| name.contains("payload")));
}

#[test]
fn i4_trinity_frames_do_not_encode_plaintext_payloads_for_verifier() {
    let signing_key = SigningKeypair::from_bytes([41; 32]);
    let request = TrinityRequest::AppendAudit {
        op: "reveal".to_owned(),
        target_id: Some("local-1".to_owned()),
        result: AuditResult::Allowed,
        payload_hash: "payload-hash-only".to_owned(),
    };
    let frame = vaultcore_core::sign_trinity_frame(&signing_key, "session-1", 7, request)
        .expect("sign frame");

    let encoded = encode_frame(&frame).expect("encode frame");
    let decoded = decode_frame(&encoded).expect("decode frame");

    assert_eq!(decoded.counter, 7);
    assert!(!String::from_utf8_lossy(&encoded).contains("plaintext-secret-marker"));
}

#[test]
fn i5_builder_write_requires_verifier_authorization() {
    let mut service = BuilderService::default();

    let error = service
        .create(SecretInput {
            role: Role::Viewer,
            secret_type: SecretType::Note,
            name: "viewer write attempt".to_owned(),
            payload_handle: "payload://local".to_owned(),
        })
        .expect_err("viewer write denied");

    assert_eq!(error.code, VaultErrorCode::AuthorizationDenied);
}

#[test]
fn i6_specanchor_signature_rejects_tampering() {
    let signing_key = SigningKeypair::from_bytes([42; 32]);
    let builder_key = SigningKeypair::from_bytes([43; 32]).verification_key();
    let mut signed = sign_specanchor(SpecAnchor::development_default(builder_key), &signing_key)
        .expect("sign specanchor");

    signed.payload.policy_version = "tampered".to_owned();
    let error = verify_signed_specanchor(&signed).expect_err("tamper rejected");

    assert_eq!(error.code, VaultErrorCode::SpecAnchorFailure);
}

#[test]
fn i7_runtime_sources_do_not_contain_remote_unlock_or_escrow_calls() {
    let root = repo_root();
    let mut scanned = 0usize;
    for relative in [
        "crates/builder/src",
        "crates/verifier/src",
        "crates/core/src",
        "app/src",
        "app/src-tauri/src",
    ] {
        for file in source_files(&root.join(relative)) {
            let text = fs::read_to_string(&file).expect("read source file");
            scanned += 1;
            for forbidden in [
                "reqwest::",
                "ureq::",
                "fetch(",
                "WebSocket",
                "key escrow",
                "remote recovery",
            ] {
                assert!(
                    !text.contains(forbidden),
                    "{} contains forbidden remote/backdoor marker {forbidden}",
                    file.display()
                );
            }
        }
    }
    assert!(scanned > 0);
}

#[test]
fn i8_write_path_emits_audit_entry_after_authorization() {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let gateway = RecordingGateway {
        requests: Rc::clone(&requests),
    };
    let mut service = BuilderService::new(gateway, "session-1");

    service
        .create(SecretInput {
            role: Role::Owner,
            secret_type: SecretType::Note,
            name: "audited write".to_owned(),
            payload_handle: "payload://local".to_owned(),
        })
        .expect("owner write");

    let requests = requests.borrow();
    assert!(requests.iter().any(|request| matches!(
        request,
        TrinityRequest::AuthorizeOp { op, .. } if op == "create"
    )));
    assert!(requests.iter().any(|request| matches!(
        request,
        TrinityRequest::AppendAudit { op, result, .. }
            if op == "create" && *result == AuditResult::Allowed
    )));
}

#[test]
fn i8_audit_chain_detects_missing_or_tampered_entries() {
    let (_directory, connection, _db_path) = temp_connection();
    let repo = AuditRepo::with_zero_genesis(&connection);

    repo.append(&AppendAuditEntry {
        ts: 1,
        actor: "Owner:session_1".to_owned(),
        op: "create_secret".to_owned(),
        target_id: None,
        result: "ok".to_owned(),
        countersignature: vec![1, 2, 3],
    })
    .expect("append audit");

    connection
        .execute("UPDATE audit_log SET op = 'tampered' WHERE seq = 1", [])
        .expect("tamper audit");

    let entries = repo.list().expect("list audit");
    let error = verify_chain(&entries, genesis_hash()).expect_err("tamper detected");
    assert_eq!(error.code, VaultErrorCode::AuditChainAnomaly);
}

#[test]
fn verifier_policy_default_denies_unknown_operations() {
    assert!(!policy::allows(Role::Owner, "remote_recovery"));
    assert!(!policy::allows(Role::Auditor, "reveal"));
}

#[derive(Clone)]
struct RecordingGateway {
    requests: Rc<RefCell<Vec<TrinityRequest>>>,
}

impl VerifierGateway for RecordingGateway {
    fn authorize(&mut self, request: TrinityRequest) -> Result<(), VaultError> {
        self.requests.borrow_mut().push(request);
        Ok(())
    }

    fn append_audit(&mut self, request: TrinityRequest) -> Result<(), VaultError> {
        self.requests.borrow_mut().push(request);
        Ok(())
    }
}

fn temp_connection() -> (TempDir, Connection, PathBuf) {
    let directory = TempDir::new().expect("temporary db directory");
    let db_path = directory.path().join("vaultcore.sqlite3");
    let mut connection = Connection::open(&db_path).expect("open sqlite db");
    migrate(&mut connection).expect("run migrations");
    (directory, connection, db_path)
}

fn new_secret(payload_envelope: Vec<u8>) -> NewSecret {
    NewSecret {
        id: SecretId::generate(),
        secret_type: SecretType::ApiKey,
        name: "github ci".to_owned(),
        labels: vec!["prod".to_owned()],
        created_at: 1_800_000_000_000,
        expires_at: None,
        payload_envelope: Some(payload_envelope),
        payload_dek_id: Some("dek_01".to_owned()),
        meta: SecretMeta::ApiKey(ApiKeyMeta {
            service: "github".to_owned(),
            key_name: "ci".to_owned(),
            environment: Some("prod".to_owned()),
            labels: vec!["prod".to_owned()],
        }),
    }
}

fn sqlite_secret_indexes(connection: &Connection) -> Vec<String> {
    let mut statement = connection
        .prepare(
            "SELECT m.name
             FROM sqlite_master m
             WHERE m.type = 'index' AND m.tbl_name = 'secrets'
             ORDER BY m.name",
        )
        .expect("prepare index query");
    statement
        .query_map(params![], |row| row.get(0))
        .expect("query indexes")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect indexes")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
        .to_path_buf()
}

fn source_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_source_files(directory, &mut files);
    files
}

fn collect_source_files(directory: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).expect("read source directory") {
        let path = entry.expect("read directory entry").path();
        if path.is_dir() {
            collect_source_files(&path, files);
        } else if matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("rs" | "ts" | "tsx")
        ) {
            files.push(path);
        }
    }
}
