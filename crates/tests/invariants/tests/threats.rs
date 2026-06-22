use std::{collections::BTreeSet, fs, path::Path};

#[test]
fn threat_model_covers_all_in_scope_threats() {
    let text = fs::read_to_string(repo_root().join("THREAT_MODEL.md")).expect("read threat model");

    for id in 1..=23 {
        let threat_id = format!("T-{id:03}");
        assert!(text.contains(&threat_id), "missing {threat_id}");
    }
}

#[test]
fn threat_rows_have_evidence_or_accepted_residual_risk() {
    let text = fs::read_to_string(repo_root().join("THREAT_MODEL.md")).expect("read threat model");
    let mut seen = BTreeSet::new();

    for line in text.lines().filter(|line| line.starts_with("| T-")) {
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        assert_eq!(cells.len(), 6, "malformed row: {line}");
        let threat_id = cells[1];
        let evidence = cells[4];
        assert!(seen.insert(threat_id.to_owned()), "duplicate {threat_id}");
        assert!(
            evidence.contains('`') || evidence.contains("R-"),
            "{threat_id} must cite a test/source path or accepted residual risk"
        );
    }

    assert_eq!(seen.len(), 23);
}

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
}
