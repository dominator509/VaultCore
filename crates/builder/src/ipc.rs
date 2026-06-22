use vaultcore_core::{
    encode_frame, sign_trinity_frame, SignedTrinityFrame, SigningKeypair, TrinityRequest,
    VaultError,
};

#[derive(Debug, Clone)]
pub struct BuilderIpcSigner {
    signing_key: SigningKeypair,
    session_id: String,
    next_counter: u64,
}

impl BuilderIpcSigner {
    #[must_use]
    pub fn new(signing_key: SigningKeypair, session_id: impl Into<String>) -> Self {
        Self {
            signing_key,
            session_id: session_id.into(),
            next_counter: 1,
        }
    }

    /// Sign and length-prefix a Builder-to-Verifier request.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::IpcFailure` if frame serialization fails.
    pub fn sign_request(&mut self, payload: TrinityRequest) -> Result<Vec<u8>, VaultError> {
        let frame = self.sign_frame(payload)?;
        encode_frame(&frame)
    }

    /// Sign a Builder-to-Verifier request without encoding it.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::IpcFailure` if the signing payload cannot be canonicalized.
    pub fn sign_frame(
        &mut self,
        payload: TrinityRequest,
    ) -> Result<SignedTrinityFrame, VaultError> {
        let counter = self.next_counter;
        self.next_counter = self.next_counter.saturating_add(1);
        sign_trinity_frame(&self.signing_key, self.session_id.clone(), counter, payload)
    }
}
