use serde::{Deserialize, Serialize};

use crate::crypto::{SignatureBytes, VerificationKey};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecAnchor {
    pub policy_version: String,
    pub crypto_suite: SpecAnchorCryptoSuite,
    pub rbac_version: String,
    pub ipc_schema_version: u32,
    pub tauri_schema_version: u32,
    pub builder_verifier_key: VerificationKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecAnchorCryptoSuite {
    pub aead: String,
    pub kdf: String,
    pub passphrase_kdf: String,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedSpecAnchor {
    pub payload: SpecAnchor,
    pub signing_key: VerificationKey,
    pub signature: SignatureBytes,
}

impl SpecAnchor {
    #[must_use]
    pub fn development_default(builder_verifier_key: VerificationKey) -> Self {
        Self {
            policy_version: "dev-policy-v1".to_owned(),
            crypto_suite: SpecAnchorCryptoSuite {
                aead: "XCHACHA20_POLY1305".to_owned(),
                kdf: "HKDF_SHA_512".to_owned(),
                passphrase_kdf: "ARGON2ID".to_owned(),
                signature: "ED25519".to_owned(),
            },
            rbac_version: "dev-rbac-v1".to_owned(),
            ipc_schema_version: 1,
            tauri_schema_version: 1,
            builder_verifier_key,
        }
    }
}
