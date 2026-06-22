use std::{fs, path::Path};

#[test]
fn generated_fixture_is_synthetic_and_deterministic() {
    let root = repo_root();
    let fixture = fs::read_to_string(root.join("tests/fixtures/synthetic-vault-v1.json"))
        .expect("read generated fixture");
    let hash = fs::read_to_string(root.join("tests/fixtures/synthetic-vault-v1.sha256"))
        .expect("read generated fixture hash");

    assert!(fixture.contains("\"real_user_data\": false"));
    assert!(fixture.contains("vaultcore-ep-007-fixture-seed"));
    assert!(fixture.contains("payload_envelope_hex"));
    assert!(!fixture.contains("BEGIN PRIVATE KEY"));
    assert!(!fixture.contains("real-password"));
    assert_eq!(hash.trim().len(), 64);
    assert!(hash.trim().chars().all(|value| value.is_ascii_hexdigit()));
}

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
}
