#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

use vaultcore_builder::{
    api::{
        Ack, AuditFilter, AuditViewEntry, ChainStatus, RevealResponse, SecretInput,
        SecretListFilter, SecretPatch, SecretSummary,
    },
    service::BuilderService,
    session::{AuthProof, SessionToken},
};
use vaultcore_core::{Role, VaultError};

#[tauri::command]
fn unlock(method: String, proof: String) -> Result<SessionToken, VaultError> {
    BuilderService::default().unlock(&AuthProof { method, proof })
}

#[tauri::command]
fn list(filter: SecretListFilter) -> Result<Vec<SecretSummary>, VaultError> {
    BuilderService::default().list(&filter)
}

#[tauri::command]
fn reveal(secret_id: String, reason: String, role: Role) -> Result<RevealResponse, VaultError> {
    BuilderService::default().reveal(&secret_id, &reason, role)
}

#[tauri::command]
fn copy(secret_id: String, ttl_ms: u64, role: Role) -> Result<Ack, VaultError> {
    BuilderService::default().copy(&secret_id, ttl_ms, role)
}

#[tauri::command]
fn create(secret_input: SecretInput) -> Result<SecretSummary, VaultError> {
    BuilderService::default().create(secret_input)
}

#[tauri::command]
fn update(secret_id: String, patch: SecretPatch) -> Result<SecretSummary, VaultError> {
    BuilderService::default().update(&secret_id, patch)
}

#[tauri::command]
fn rotate(
    secret_id: String,
    new_payload_handle: String,
    role: Role,
) -> Result<SecretSummary, VaultError> {
    BuilderService::default().rotate(&secret_id, &new_payload_handle, role)
}

#[tauri::command]
fn soft_delete(secret_id: String, role: Role) -> Result<Ack, VaultError> {
    BuilderService::default().soft_delete(&secret_id, role)
}

#[tauri::command]
fn purge(secret_id: String, confirmation_token: String, role: Role) -> Result<Ack, VaultError> {
    BuilderService::default().purge(&secret_id, &confirmation_token, role)
}

#[tauri::command]
fn audit_view(filter: AuditFilter) -> Result<Vec<AuditViewEntry>, VaultError> {
    BuilderService::default().audit_view(&filter)
}

#[tauri::command]
fn verify_audit_chain() -> Result<ChainStatus, VaultError> {
    BuilderService::default().verify_audit_chain()
}

#[tauri::command]
fn lock() -> Result<Ack, VaultError> {
    BuilderService::default().lock()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            unlock,
            list,
            reveal,
            copy,
            create,
            update,
            rotate,
            soft_delete,
            purge,
            audit_view,
            verify_audit_chain,
            lock
        ])
        .run(tauri::generate_context!())
        .expect("failed to run VaultCore Tauri shell");
}
