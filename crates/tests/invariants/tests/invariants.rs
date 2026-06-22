#[test]
fn invariant_suite_is_wired() {
    assert_eq!(vaultcore_core::VERSION, env!("CARGO_PKG_VERSION"));
}
