use std::{
    fs,
    path::{Path, PathBuf},
};

#[test]
fn default_runtime_sources_do_not_introduce_remote_network_calls() {
    let root = repo_root();
    let scan_roots = [
        "crates/builder/src",
        "crates/verifier/src",
        "crates/core/src",
        "app/src",
        "app/src-tauri/src",
    ];
    let forbidden = [
        "reqwest::",
        "ureq::",
        "hyper::",
        "tonic::",
        "TcpStream",
        "TcpListener",
        "UdpSocket",
        "fetch(",
        "WebSocket",
        "sendBeacon",
        "remote unlock",
        "remote recovery",
        "key escrow",
        "telemetry sink",
    ];
    let mut scanned = 0usize;

    for relative in scan_roots {
        for file in source_files(&root.join(relative)) {
            let text = fs::read_to_string(&file).expect("read runtime source");
            scanned += 1;
            for marker in forbidden {
                assert!(
                    !text.contains(marker),
                    "{} contains forbidden network/backdoor marker {marker}",
                    file.display()
                );
            }
        }
    }

    assert!(scanned > 0);
}

#[test]
fn workspace_manifests_do_not_add_direct_network_clients() {
    let root = repo_root();
    let manifests = [
        "crates/core/Cargo.toml",
        "crates/builder/Cargo.toml",
        "crates/verifier/Cargo.toml",
        "crates/cli/Cargo.toml",
        "crates/tests/invariants/Cargo.toml",
        "app/package.json",
    ];
    let forbidden = ["reqwest", "ureq", "hyper", "tonic", "axios", "socket.io"];

    for relative in manifests {
        let file = root.join(relative);
        let text = fs::read_to_string(&file).expect("read manifest");
        for marker in forbidden {
            assert!(
                !text.contains(marker),
                "{} contains direct network client dependency {marker}",
                file.display()
            );
        }
    }
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
