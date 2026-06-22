use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
    Auditor,
}

impl Role {
    pub const ALL: [Self; 5] = [
        Self::Owner,
        Self::Admin,
        Self::Editor,
        Self::Viewer,
        Self::Auditor,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "Owner",
            Self::Admin => "Admin",
            Self::Editor => "Editor",
            Self::Viewer => "Viewer",
            Self::Auditor => "Auditor",
        }
    }

    #[must_use]
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::Owner => Some(4),
            Self::Admin => Some(3),
            Self::Editor => Some(2),
            Self::Viewer => Some(1),
            Self::Auditor => None,
        }
    }

    #[must_use]
    pub const fn satisfies_minimum(self, minimum: Self) -> bool {
        match (self.rank(), minimum.rank()) {
            (Some(actual), Some(required)) => actual >= required,
            _ => matches!((self, minimum), (Self::Auditor, Self::Auditor)),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::Role;

    #[test]
    fn roles_are_exhaustive() {
        assert_eq!(Role::ALL.len(), 5);
    }

    #[test]
    fn hierarchical_roles_satisfy_lower_minimums() {
        assert!(Role::Owner.satisfies_minimum(Role::Viewer));
        assert!(Role::Admin.satisfies_minimum(Role::Editor));
        assert!(!Role::Viewer.satisfies_minimum(Role::Editor));
    }

    #[test]
    fn auditor_is_parallel_read_only_role() {
        assert!(Role::Auditor.satisfies_minimum(Role::Auditor));
        assert!(!Role::Auditor.satisfies_minimum(Role::Viewer));
        assert!(!Role::Owner.satisfies_minimum(Role::Auditor));
    }
}
