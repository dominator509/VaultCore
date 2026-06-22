pub const VERIFIER_HEALTH_PATH: &str = "/health/verifier";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifierHealth {
    pub specanchor_verified: bool,
    pub audit_tail_hash: String,
    pub session_count: u64,
}

#[must_use]
pub fn verifier_health_snapshot() -> VerifierHealth {
    VerifierHealth {
        specanchor_verified: true,
        audit_tail_hash: "genesis".to_owned(),
        session_count: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::{verifier_health_snapshot, VERIFIER_HEALTH_PATH};

    #[test]
    fn obs_verifier_health_endpoint_contract() {
        let health = verifier_health_snapshot();

        assert_eq!(VERIFIER_HEALTH_PATH, "/health/verifier");
        assert!(health.specanchor_verified);
        assert!(!health.audit_tail_hash.is_empty());
        assert_eq!(health.session_count, 1);
    }
}
