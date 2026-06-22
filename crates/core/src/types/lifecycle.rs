use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    Draft,
    Active,
    ExpiringSoon,
    Expired,
    Rotating,
    Archived,
    SoftDeleted,
    Purged,
}

impl LifecycleState {
    pub const ALL: [Self; 8] = [
        Self::Draft,
        Self::Active,
        Self::ExpiringSoon,
        Self::Expired,
        Self::Rotating,
        Self::Archived,
        Self::SoftDeleted,
        Self::Purged,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
            Self::Rotating => "rotating",
            Self::Archived => "archived",
            Self::SoftDeleted => "soft_deleted",
            Self::Purged => "purged",
        }
    }

    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Purged)
    }
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::LifecycleState;

    #[test]
    fn lifecycle_states_are_exhaustive() {
        assert_eq!(LifecycleState::ALL.len(), 8);
    }

    #[test]
    fn lifecycle_serde_uses_spec_names() {
        let encoded = serde_json::to_string(&LifecycleState::ExpiringSoon).expect("serialize");
        assert_eq!(encoded, "\"expiring_soon\"");
        let decoded: LifecycleState =
            serde_json::from_str("\"soft_deleted\"").expect("deserialize");
        assert_eq!(decoded, LifecycleState::SoftDeleted);
    }

    #[test]
    fn only_purged_is_terminal() {
        assert!(LifecycleState::Purged.is_terminal());
        assert!(!LifecycleState::Archived.is_terminal());
    }
}
