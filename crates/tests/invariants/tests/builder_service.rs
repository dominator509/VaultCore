use vaultcore_builder::{
    api::{AuditFilter, SecretInput, SecretListFilter},
    service::BuilderService,
};
use vaultcore_core::{Role, SecretType, VaultErrorCode};

#[test]
fn builder_service_authorizes_reads_through_verifier_boundary() {
    let mut service = BuilderService::default();

    service
        .list(&SecretListFilter {
            role: Role::Viewer,
            query: None,
        })
        .expect("viewer can list");
}

#[test]
fn builder_service_default_denies_unknown_or_insufficient_roles() {
    let mut service = BuilderService::default();

    let error = service
        .create(SecretInput {
            role: Role::Viewer,
            secret_type: SecretType::Note,
            name: "example".to_owned(),
            payload_handle: "payload://local".to_owned(),
        })
        .expect_err("viewer cannot create");

    assert_eq!(error.code, VaultErrorCode::AuthorizationDenied);
}

#[test]
fn builder_service_never_reveals_plaintext_payload() {
    let mut service = BuilderService::default();

    let response = service
        .reveal("local-1", "maintenance", Role::Viewer)
        .expect("viewer can reveal handle");

    assert!(response.payload_handle.starts_with("payload://"));
}

#[test]
fn builder_service_keeps_auditor_role_parallel_to_hierarchy() {
    let mut service = BuilderService::default();

    service
        .audit_view(&AuditFilter {
            role: Role::Auditor,
        })
        .expect("auditor can view audit");

    let error = service
        .list(&SecretListFilter {
            role: Role::Auditor,
            query: None,
        })
        .expect_err("auditor cannot list secrets");
    assert_eq!(error.code, VaultErrorCode::AuthorizationDenied);
}
