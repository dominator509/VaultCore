use rusqlite::{params, Connection};
use tempfile::TempDir;
use vaultcore_core::{
    genesis_hash, migrate, verify_chain, ApiKeyMeta, AppendAuditEntry, AuditRepo, LifecycleState,
    ListSecretsFilter, NewSecret, SecretId, SecretMeta, SecretRepo, SecretType, UpdateSecret,
};

fn temp_connection() -> (TempDir, Connection) {
    let directory = TempDir::new().expect("temporary db directory");
    let db_path = directory.path().join("vaultcore.sqlite3");
    let mut connection = Connection::open(db_path).expect("open sqlite db");
    migrate(&mut connection).expect("run migrations");
    (directory, connection)
}

fn api_key_meta() -> SecretMeta {
    SecretMeta::ApiKey(ApiKeyMeta {
        service: "github".to_owned(),
        key_name: "ci".to_owned(),
        environment: Some("prod".to_owned()),
        labels: vec!["prod".to_owned()],
    })
}

fn new_secret() -> NewSecret {
    NewSecret {
        id: SecretId::generate(),
        secret_type: SecretType::ApiKey,
        name: "github ci".to_owned(),
        labels: vec!["prod".to_owned()],
        created_at: 1_800_000_000_000,
        expires_at: None,
        payload_envelope: Some(vec![0, 159, 92, 221, 17, 3]),
        payload_dek_id: Some("dek_01".to_owned()),
        meta: api_key_meta(),
    }
}

#[test]
fn migration_creates_spec_tables_and_no_payload_indexes() {
    let (_directory, connection) = temp_connection();

    for table in ["secrets", "audit_log", "specanchor_meta", "migrations"] {
        let exists: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                params![table],
                |row| row.get(0),
            )
            .expect("query sqlite_master");
        assert_eq!(exists, 1, "{table} table should exist");
    }

    let indexed_columns: Vec<String> = {
        let mut statement = connection
            .prepare(
                "SELECT m.name
                 FROM sqlite_master m
                 WHERE m.type = 'index' AND m.tbl_name = 'secrets'
                 ORDER BY m.name",
            )
            .expect("prepare index query");
        statement
            .query_map([], |row| row.get(0))
            .expect("query indexes")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect indexes")
    };
    assert!(indexed_columns.iter().any(|name| name.contains("type")));
    assert!(indexed_columns.iter().any(|name| name.contains("state")));
    assert!(indexed_columns.iter().any(|name| name.contains("name")));
    assert!(indexed_columns
        .iter()
        .any(|name| name.contains("expires_at")));
    assert!(!indexed_columns.iter().any(|name| name.contains("payload")));
}

#[test]
fn secret_repo_crud_list_and_purge_tombstones_payload() {
    let (_directory, connection) = temp_connection();
    let repo = SecretRepo::new(&connection);
    let secret = new_secret();

    let created = repo.create(&secret).expect("create secret");
    assert_eq!(created.state, LifecycleState::Draft);
    assert_eq!(
        created.payload_envelope.as_deref(),
        secret.payload_envelope.as_deref()
    );

    let active = repo
        .update(
            secret.id,
            &UpdateSecret {
                state: Some(LifecycleState::Active),
                updated_at: secret.created_at + 1,
                ..UpdateSecret::default()
            },
        )
        .expect("activate secret");
    assert_eq!(active.state, LifecycleState::Active);

    let listed = repo
        .list(&ListSecretsFilter {
            secret_type: Some(SecretType::ApiKey),
            state: Some(LifecycleState::Active),
            name_contains: Some("github".to_owned()),
        })
        .expect("list secrets");
    assert_eq!(listed.len(), 1);

    repo.update(
        secret.id,
        &UpdateSecret {
            state: Some(LifecycleState::Archived),
            updated_at: secret.created_at + 2,
            ..UpdateSecret::default()
        },
    )
    .expect("archive secret");
    repo.update(
        secret.id,
        &UpdateSecret {
            state: Some(LifecycleState::SoftDeleted),
            updated_at: secret.created_at + 3,
            ..UpdateSecret::default()
        },
    )
    .expect("soft delete secret");

    let purged = repo
        .purge(secret.id, secret.created_at + 4)
        .expect("purge secret");
    assert_eq!(purged.state, LifecycleState::Purged);
    assert!(purged.payload_envelope.is_none());
    assert!(purged.payload_dek_id.is_none());
}

#[test]
fn secret_repo_rejects_illegal_fsm_transition() {
    let (_directory, connection) = temp_connection();
    let repo = SecretRepo::new(&connection);
    let secret = new_secret();
    repo.create(&secret).expect("create secret");

    let error = repo
        .update(
            secret.id,
            &UpdateSecret {
                state: Some(LifecycleState::Purged),
                updated_at: secret.created_at + 1,
                ..UpdateSecret::default()
            },
        )
        .expect_err("draft cannot purge directly");

    assert_eq!(error.code.as_str(), "VC-FSM-001");
}

#[test]
fn audit_repo_appends_and_verifies_hash_chain() {
    let (_directory, connection) = temp_connection();
    let repo = AuditRepo::with_zero_genesis(&connection);

    let first = repo
        .append(&AppendAuditEntry {
            ts: 1,
            actor: "Owner:session_1".to_owned(),
            op: "create_secret".to_owned(),
            target_id: None,
            result: "ok".to_owned(),
            countersignature: vec![1, 2, 3],
        })
        .expect("append first audit");
    let second = repo
        .append(&AppendAuditEntry {
            ts: 2,
            actor: "Owner:session_1".to_owned(),
            op: "update_secret".to_owned(),
            target_id: Some("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_owned()),
            result: "ok".to_owned(),
            countersignature: vec![4, 5, 6],
        })
        .expect("append second audit");

    assert_ne!(first, genesis_hash());
    assert_ne!(first, second);

    let entries = repo.list().expect("list audit");
    let verified = verify_chain(&entries, genesis_hash()).expect("verify chain");
    assert_eq!(verified.entries, 2);
    assert_eq!(verified.head, second);
}

#[test]
fn audit_chain_detects_tampering() {
    let (_directory, connection) = temp_connection();
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
    assert_eq!(error.code.as_str(), "VC-AUDIT-001");
}

#[test]
fn migration_continuity_preserves_audit_chain() {
    let (_directory, connection) = temp_connection();
    let repo = AuditRepo::with_zero_genesis(&connection);
    let head = repo
        .append(&AppendAuditEntry {
            ts: 1,
            actor: "Owner:session_1".to_owned(),
            op: "migration_0002".to_owned(),
            target_id: None,
            result: "ok".to_owned(),
            countersignature: vec![7, 8, 9],
        })
        .expect("append migration audit");

    let entries = repo.list().expect("list audit");
    let verified = verify_chain(&entries, genesis_hash()).expect("verify chain");
    assert_eq!(verified.head, head);

    let marker: String = connection
        .query_row(
            "SELECT note FROM migration_continuity_marker WHERE version = 2",
            [],
            |row| row.get(0),
        )
        .expect("read continuity marker");
    assert_eq!(marker, "additive migration continuity fixture");
}
