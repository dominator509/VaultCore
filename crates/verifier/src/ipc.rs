use std::collections::HashMap;

use vaultcore_core::{
    decode_frame, verify_trinity_frame, SignedTrinityFrame, TrinityRequest, VaultError,
    VaultErrorCode, VerificationKey,
};

#[derive(Debug, Clone)]
pub struct VerifierIpc {
    trusted_builder_key: VerificationKey,
    last_counters: HashMap<String, u64>,
}

impl VerifierIpc {
    #[must_use]
    pub fn new(trusted_builder_key: VerificationKey) -> Self {
        Self {
            trusted_builder_key,
            last_counters: HashMap::new(),
        }
    }

    /// Decode, verify, and replay-check a Builder frame.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::IpcFailure` for malformed frames, invalid
    /// signatures, untrusted keys, or non-monotonic counters.
    pub fn receive(&mut self, bytes: &[u8]) -> Result<TrinityRequest, VaultError> {
        let frame = decode_frame(bytes)?;
        self.receive_frame(frame)
    }

    /// Verify and replay-check an already decoded Builder frame.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::IpcFailure` for invalid signatures, untrusted keys,
    /// or non-monotonic counters.
    pub fn receive_frame(
        &mut self,
        frame: SignedTrinityFrame,
    ) -> Result<TrinityRequest, VaultError> {
        verify_trinity_frame(&frame, self.trusted_builder_key)?;
        let last_counter = self
            .last_counters
            .entry(frame.session_id.clone())
            .or_insert(0);
        if frame.counter <= *last_counter {
            return Err(VaultError::new(
                VaultErrorCode::IpcFailure,
                None,
                "Trinity replay detected",
            ));
        }
        *last_counter = frame.counter;
        Ok(frame.payload)
    }
}
