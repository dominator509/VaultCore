use serde::{Deserialize, Serialize};
use vaultcore_core::{LifecycleState, Role, SecretType};

use crate::session::AuthProof;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ack {
    pub ok: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretSummary {
    pub id: String,
    pub secret_type: SecretType,
    pub name: String,
    pub state: LifecycleState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretListFilter {
    pub role: Role,
    pub query: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretInput {
    pub role: Role,
    pub secret_type: SecretType,
    pub name: String,
    pub payload_handle: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretPatch {
    pub role: Role,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevealResponse {
    pub ttl_ms: u64,
    pub payload_handle: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditFilter {
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditViewEntry {
    pub op: String,
    pub target_id: Option<String>,
    pub result: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainStatus {
    pub valid: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnlockRequest {
    pub auth: AuthProof,
}
