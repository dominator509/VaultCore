use vaultcore_core::{sign_message, SignatureBytes, SigningKeypair};

#[derive(Debug, Clone)]
pub struct AuditSigner {
    signing_key: SigningKeypair,
}

impl AuditSigner {
    #[must_use]
    pub const fn new(signing_key: SigningKeypair) -> Self {
        Self { signing_key }
    }

    #[must_use]
    pub fn countersign(&self, entry_hash: &str) -> SignatureBytes {
        sign_message(&self.signing_key, entry_hash.as_bytes())
    }
}
