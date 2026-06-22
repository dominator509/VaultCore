#![forbid(unsafe_code)]

pub mod crypto;
pub mod error;
pub mod fsm;
pub mod persistence;
pub mod specanchor;
pub mod trinity;
pub mod types;
pub mod validation;

pub use crypto::{
    decrypt_payload, derive_argon2id_key, derive_hkdf_sha512, encrypt_payload, sign_message,
    verify_message, AeadKey, AeadNonce, Ciphertext, KdfSalt, SealedBytes, SignatureBytes,
    SigningKeypair, VerificationKey,
};
pub use error::{VaultError, VaultErrorCategory, VaultErrorCode};
pub use fsm::{can_transition, transition, LEGAL_TRANSITIONS};
pub use persistence::{
    apply_pragmas, compute_entry_hash, compute_payload_hash, genesis_hash, migrate, verify_chain,
    AppendAuditEntry, AuditChainEntry, AuditEntry, AuditRepo, ListSecretsFilter, NewSecret,
    SecretRecord, SecretRepo, UpdateSecret, VerifiedChain,
};
pub use specanchor::{
    decode_signed_specanchor, encode_signed_specanchor, sign_specanchor, verify_signed_specanchor,
    SignedSpecAnchor, SpecAnchor, SpecAnchorCryptoSuite,
};
pub use trinity::{
    decode_frame, encode_frame, sign_trinity_frame, verify_trinity_frame, AuditResult,
    Countersignature, SignedTrinityFrame, TrinityRequest, TrinityResponse, TrinityStatus,
    TRINITY_SCHEMA_VERSION,
};
pub use types::{
    ApiKeyMeta, BlobMeta, CertMeta, LifecycleState, LoginMeta, NoteMeta, OAuthAppMeta, Role,
    SecretId, SecretMeta, SecretType, SshKeyMeta, WalletKeyMeta,
};
pub use validation::{
    validate_labels, validate_name, validate_optional_field, validate_required_field,
    validate_uri_list, FieldRule,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn exposes_package_version() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
