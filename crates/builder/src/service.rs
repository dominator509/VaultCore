use std::sync::atomic::{AtomicU64, Ordering};

use vaultcore_core::{AuditResult, Role, SecretType, TrinityRequest, VaultError, VaultErrorCode};

use crate::{
    api::{
        Ack, AuditFilter, AuditViewEntry, ChainStatus, RevealResponse, SecretInput,
        SecretListFilter, SecretPatch, SecretSummary,
    },
    session::{AuthProof, SessionToken},
};

pub trait VerifierGateway {
    /// Authorize a Builder operation through the Verifier boundary.
    ///
    /// # Errors
    ///
    /// Returns a `VaultCore` error when the Verifier rejects the operation or the IPC
    /// boundary cannot be reached.
    fn authorize(&mut self, request: TrinityRequest) -> Result<(), VaultError>;

    /// Append an audit event through the Verifier boundary.
    ///
    /// # Errors
    ///
    /// Returns a `VaultCore` error when audit append fails.
    fn append_audit(&mut self, request: TrinityRequest) -> Result<(), VaultError>;
}

#[derive(Debug, Default)]
pub struct InMemoryVerifierGateway {
    pub requests: Vec<TrinityRequest>,
}

impl VerifierGateway for InMemoryVerifierGateway {
    fn authorize(&mut self, request: TrinityRequest) -> Result<(), VaultError> {
        if authorized_request(&request) {
            self.requests.push(request);
            Ok(())
        } else {
            self.requests.push(request);
            Err(VaultError::new(
                VaultErrorCode::AuthorizationDenied,
                Some("role".to_owned()),
                "role is not authorized for operation",
            ))
        }
    }

    fn append_audit(&mut self, request: TrinityRequest) -> Result<(), VaultError> {
        self.requests.push(request);
        Ok(())
    }
}

#[derive(Debug)]
pub struct BuilderService<G> {
    gateway: G,
    session_id: String,
    next_id: AtomicU64,
}

impl Default for BuilderService<InMemoryVerifierGateway> {
    fn default() -> Self {
        Self::new(InMemoryVerifierGateway::default(), "local-session")
    }
}

impl<G> BuilderService<G>
where
    G: VerifierGateway,
{
    #[must_use]
    pub fn new(gateway: G, session_id: impl Into<String>) -> Self {
        Self {
            gateway,
            session_id: session_id.into(),
            next_id: AtomicU64::new(1),
        }
    }

    /// Unlock a local session.
    ///
    /// # Errors
    ///
    /// Returns an auth error if the supplied proof is empty.
    pub fn unlock(&mut self, auth: &AuthProof) -> Result<SessionToken, VaultError> {
        if auth.method.is_empty() || auth.proof.is_empty() {
            return Err(VaultError::new(
                VaultErrorCode::AuthSessionExpired,
                Some("proof".to_owned()),
                "unlock proof is required",
            ));
        }
        Ok(SessionToken {
            session_id: self.session_id.clone(),
        })
    }

    /// List secret metadata summaries.
    ///
    /// # Errors
    ///
    /// Returns an authorization error if the role cannot list secrets.
    pub fn list(&mut self, filter: &SecretListFilter) -> Result<Vec<SecretSummary>, VaultError> {
        self.authorize("list", None, filter.role)?;
        self.audit("list", None, AuditResult::Allowed)?;
        Ok(Vec::new())
    }

    /// Reveal a payload handle without returning plaintext.
    ///
    /// # Errors
    ///
    /// Returns authorization or validation errors for invalid reveal requests.
    pub fn reveal(
        &mut self,
        secret_id: &str,
        reason: &str,
        role: Role,
    ) -> Result<RevealResponse, VaultError> {
        if reason.trim().is_empty() {
            return Err(VaultError::invalid_field("reason", "reason is required"));
        }
        self.authorize("reveal", Some(secret_id.to_owned()), role)?;
        self.audit("reveal", Some(secret_id.to_owned()), AuditResult::Allowed)?;
        Ok(RevealResponse {
            ttl_ms: 30_000,
            payload_handle: format!("payload://{secret_id}"),
        })
    }

    /// Copy a payload through a local handle with a bounded TTL.
    ///
    /// # Errors
    ///
    /// Returns authorization or validation errors for invalid copy requests.
    pub fn copy(&mut self, secret_id: &str, ttl_ms: u64, role: Role) -> Result<Ack, VaultError> {
        if ttl_ms == 0 {
            return Err(VaultError::invalid_field(
                "ttl_ms",
                "ttl_ms must be positive",
            ));
        }
        self.authorize("copy", Some(secret_id.to_owned()), role)?;
        self.audit("copy", Some(secret_id.to_owned()), AuditResult::Allowed)?;
        Ok(Ack { ok: true })
    }

    /// Create a metadata summary for a new secret.
    ///
    /// # Errors
    ///
    /// Returns authorization or validation errors for invalid create requests.
    pub fn create(&mut self, input: SecretInput) -> Result<SecretSummary, VaultError> {
        if input.payload_handle.trim().is_empty() {
            return Err(VaultError::invalid_field(
                "payload_handle",
                "payload handle is required",
            ));
        }
        self.authorize("create", None, input.role)?;
        let id = format!("local-{}", self.next_id.fetch_add(1, Ordering::Relaxed));
        self.audit("create", Some(id.clone()), AuditResult::Allowed)?;
        Ok(summary(id, input.secret_type, input.name))
    }

    /// Update secret metadata.
    ///
    /// # Errors
    ///
    /// Returns authorization or validation errors for invalid update requests.
    pub fn update(
        &mut self,
        secret_id: &str,
        patch: SecretPatch,
    ) -> Result<SecretSummary, VaultError> {
        self.authorize("update", Some(secret_id.to_owned()), patch.role)?;
        self.audit("update", Some(secret_id.to_owned()), AuditResult::Allowed)?;
        Ok(summary(
            secret_id.to_owned(),
            SecretType::Note,
            patch.name.unwrap_or_else(|| "updated secret".to_owned()),
        ))
    }

    /// Rotate a secret payload using a payload handle.
    ///
    /// # Errors
    ///
    /// Returns authorization or validation errors for invalid rotate requests.
    pub fn rotate(
        &mut self,
        secret_id: &str,
        new_payload_handle: &str,
        role: Role,
    ) -> Result<SecretSummary, VaultError> {
        if new_payload_handle.trim().is_empty() {
            return Err(VaultError::invalid_field(
                "new_payload_handle",
                "new payload handle is required",
            ));
        }
        self.authorize("rotate", Some(secret_id.to_owned()), role)?;
        self.audit("rotate", Some(secret_id.to_owned()), AuditResult::Allowed)?;
        Ok(summary(
            secret_id.to_owned(),
            SecretType::Note,
            "rotated secret".to_owned(),
        ))
    }

    /// Soft-delete a secret.
    ///
    /// # Errors
    ///
    /// Returns an authorization error if the role cannot soft-delete secrets.
    pub fn soft_delete(&mut self, secret_id: &str, role: Role) -> Result<Ack, VaultError> {
        self.authorize("soft_delete", Some(secret_id.to_owned()), role)?;
        self.audit(
            "soft_delete",
            Some(secret_id.to_owned()),
            AuditResult::Allowed,
        )?;
        Ok(Ack { ok: true })
    }

    /// Purge a soft-deleted secret after explicit confirmation token validation.
    ///
    /// # Errors
    ///
    /// Returns authorization or validation errors for invalid purge requests.
    pub fn purge(
        &mut self,
        secret_id: &str,
        confirmation_token: &str,
        role: Role,
    ) -> Result<Ack, VaultError> {
        if confirmation_token != "PURGE" {
            return Err(VaultError::invalid_field(
                "confirmation_token",
                "confirmation token is invalid",
            ));
        }
        self.authorize("purge", Some(secret_id.to_owned()), role)?;
        self.audit("purge", Some(secret_id.to_owned()), AuditResult::Allowed)?;
        Ok(Ack { ok: true })
    }

    /// View audit metadata.
    ///
    /// # Errors
    ///
    /// Returns an authorization error if the role cannot view audit data.
    pub fn audit_view(&mut self, filter: &AuditFilter) -> Result<Vec<AuditViewEntry>, VaultError> {
        self.authorize("audit_view", None, filter.role)?;
        Ok(Vec::new())
    }

    /// Verify audit-chain status.
    ///
    /// # Errors
    ///
    /// Returns a Verifier error if chain verification cannot run.
    pub fn verify_audit_chain(&mut self) -> Result<ChainStatus, VaultError> {
        self.gateway.authorize(TrinityRequest::VerifyChain {
            head: "local-head".to_owned(),
        })?;
        Ok(ChainStatus { valid: true })
    }

    /// Lock the active session.
    ///
    /// # Errors
    ///
    /// Returns a Verifier error if session revocation cannot run.
    pub fn lock(&mut self) -> Result<Ack, VaultError> {
        self.gateway.authorize(TrinityRequest::RevokeSession {
            session_id: self.session_id.clone(),
        })?;
        Ok(Ack { ok: true })
    }

    fn authorize(
        &mut self,
        op: &'static str,
        target_id: Option<String>,
        role: Role,
    ) -> Result<(), VaultError> {
        self.gateway.authorize(TrinityRequest::AuthorizeOp {
            op: op.to_owned(),
            target_id,
            role,
            session_id: self.session_id.clone(),
        })
    }

    fn audit(
        &mut self,
        op: &'static str,
        target_id: Option<String>,
        result: AuditResult,
    ) -> Result<(), VaultError> {
        self.gateway.append_audit(TrinityRequest::AppendAudit {
            op: op.to_owned(),
            target_id,
            result,
            payload_hash: "metadata-only".to_owned(),
        })
    }
}

fn summary(id: String, secret_type: SecretType, name: String) -> SecretSummary {
    SecretSummary {
        id,
        secret_type,
        name,
        state: vaultcore_core::LifecycleState::Draft,
    }
}

fn authorized_request(request: &TrinityRequest) -> bool {
    match request {
        TrinityRequest::AuthorizeOp { op, role, .. } => minimum_role(op)
            .map_or(matches!(role, Role::Owner), |minimum| {
                role.satisfies_minimum(minimum)
            }),
        TrinityRequest::VerifyChain { .. }
        | TrinityRequest::RevokeSession { .. }
        | TrinityRequest::AppendAudit { .. }
        | TrinityRequest::IssueSession { .. } => true,
    }
}

fn minimum_role(op: &str) -> Option<Role> {
    match op {
        "list" | "reveal" | "copy" => Some(Role::Viewer),
        "create" | "update" | "rotate" => Some(Role::Editor),
        "soft_delete" => Some(Role::Admin),
        "purge" => Some(Role::Owner),
        "audit_view" => Some(Role::Auditor),
        _ => None,
    }
}
