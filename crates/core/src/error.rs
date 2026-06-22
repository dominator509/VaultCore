use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

use crate::types::{LifecycleState, Role, SecretId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VaultErrorCategory {
    Validation,
    Auth,
    Authorization,
    Lifecycle,
    Crypto,
    Ipc,
    Persistence,
    Audit,
    SpecAnchor,
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VaultErrorCode {
    ValidationInvalidField,
    ValidationInvalidName,
    ValidationInvalidLabel,
    AuthSessionExpired,
    AuthorizationDenied,
    FsmInvalidTransition,
    CryptoFailure,
    IpcFailure,
    PersistenceFailure,
    AuditChainAnomaly,
    SpecAnchorFailure,
    NotFound,
    Conflict,
    InternalFailure,
}

impl VaultErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidationInvalidField => "VC-VAL-001",
            Self::ValidationInvalidName => "VC-VAL-002",
            Self::ValidationInvalidLabel => "VC-VAL-003",
            Self::AuthSessionExpired => "VC-AUTH-001",
            Self::AuthorizationDenied => "VC-AUTHZ-001",
            Self::FsmInvalidTransition => "VC-FSM-001",
            Self::CryptoFailure => "VC-CRYPTO-001",
            Self::IpcFailure => "VC-IPC-001",
            Self::PersistenceFailure => "VC-PERSIST-001",
            Self::AuditChainAnomaly => "VC-AUDIT-001",
            Self::SpecAnchorFailure => "VC-SPEC-001",
            Self::NotFound => "VC-VAL-004",
            Self::Conflict => "VC-FSM-002",
            Self::InternalFailure => "VC-INTERNAL-001",
        }
    }

    #[must_use]
    pub const fn category(self) -> VaultErrorCategory {
        match self {
            Self::ValidationInvalidField
            | Self::ValidationInvalidName
            | Self::ValidationInvalidLabel
            | Self::NotFound => VaultErrorCategory::Validation,
            Self::AuthSessionExpired => VaultErrorCategory::Auth,
            Self::AuthorizationDenied => VaultErrorCategory::Authorization,
            Self::FsmInvalidTransition | Self::Conflict => VaultErrorCategory::Lifecycle,
            Self::CryptoFailure => VaultErrorCategory::Crypto,
            Self::IpcFailure => VaultErrorCategory::Ipc,
            Self::PersistenceFailure => VaultErrorCategory::Persistence,
            Self::AuditChainAnomaly => VaultErrorCategory::Audit,
            Self::SpecAnchorFailure => VaultErrorCategory::SpecAnchor,
            Self::InternalFailure => VaultErrorCategory::Internal,
        }
    }

    #[must_use]
    pub const fn is_recoverable(self) -> bool {
        matches!(
            self,
            Self::ValidationInvalidField
                | Self::ValidationInvalidName
                | Self::ValidationInvalidLabel
                | Self::AuthSessionExpired
                | Self::IpcFailure
                | Self::PersistenceFailure
        )
    }
}

impl fmt::Display for VaultErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultError {
    pub code: VaultErrorCode,
    pub category: VaultErrorCategory,
    pub recoverable: bool,
    pub field: Option<String>,
    pub message: String,
}

impl VaultError {
    #[must_use]
    pub fn new(code: VaultErrorCode, field: Option<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            category: code.category(),
            recoverable: code.is_recoverable(),
            field,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn invalid_field(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(
            VaultErrorCode::ValidationInvalidField,
            Some(field.into()),
            message,
        )
    }

    #[must_use]
    pub fn invalid_name(message: impl Into<String>) -> Self {
        Self::new(
            VaultErrorCode::ValidationInvalidName,
            Some("name".to_owned()),
            message,
        )
    }

    #[must_use]
    pub fn invalid_label(message: impl Into<String>) -> Self {
        Self::new(
            VaultErrorCode::ValidationInvalidLabel,
            Some("labels".to_owned()),
            message,
        )
    }

    #[must_use]
    pub fn fsm_invalid_transition(from: LifecycleState, to: LifecycleState) -> Self {
        Self::new(
            VaultErrorCode::FsmInvalidTransition,
            Some("lifecycle_state".to_owned()),
            format!("illegal lifecycle transition from {from} to {to}"),
        )
    }

    #[must_use]
    pub fn unauthorized(role: Role, operation: impl Into<String>) -> Self {
        Self::new(
            VaultErrorCode::AuthorizationDenied,
            Some("role".to_owned()),
            format!("role {role} is not authorized for {}", operation.into()),
        )
    }

    #[must_use]
    pub fn not_found(id: SecretId) -> Self {
        Self::new(
            VaultErrorCode::NotFound,
            Some("id".to_owned()),
            id.to_string(),
        )
    }

    #[must_use]
    pub fn conflict(id: SecretId, expected: LifecycleState, actual: LifecycleState) -> Self {
        Self::new(
            VaultErrorCode::Conflict,
            Some("lifecycle_state".to_owned()),
            format!("secret {id} expected {expected}, actual {actual}"),
        )
    }
}

impl fmt::Display for VaultError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Error for VaultError {}

#[cfg(test)]
mod tests {
    use super::{VaultError, VaultErrorCategory, VaultErrorCode};

    #[test]
    fn code_mapping_is_stable() {
        assert_eq!(
            VaultErrorCode::ValidationInvalidField.as_str(),
            "VC-VAL-001"
        );
        assert_eq!(VaultErrorCode::FsmInvalidTransition.as_str(), "VC-FSM-001");
        assert_eq!(VaultErrorCode::InternalFailure.as_str(), "VC-INTERNAL-001");
    }

    #[test]
    fn constructors_set_category_and_recoverability() {
        let error = VaultError::invalid_name("missing");
        assert_eq!(error.category, VaultErrorCategory::Validation);
        assert!(error.recoverable);

        let error = VaultError::new(VaultErrorCode::CryptoFailure, None, "verify failed");
        assert_eq!(error.category, VaultErrorCategory::Crypto);
        assert!(!error.recoverable);
    }

    #[test]
    fn errors_round_trip_through_serde() {
        let error = VaultError::invalid_field("service", "service is required");
        let encoded = serde_json::to_string(&error).expect("serialize error");
        let decoded: VaultError = serde_json::from_str(&encoded).expect("deserialize error");
        assert_eq!(decoded, error);
    }

    #[test]
    fn payload_is_redaction_safe() {
        let error = VaultError::invalid_field("token", "token is required");
        let encoded = serde_json::to_string(&error).expect("serialize error");
        assert!(!encoded.contains("secret"));
        assert!(!encoded.contains("key_material"));
        assert!(!encoded.contains("nonce"));
    }
}
