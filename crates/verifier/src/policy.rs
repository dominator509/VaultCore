use vaultcore_core::Role;

#[must_use]
pub fn allows(role: Role, op: &str) -> bool {
    operation_from_str(op).is_some_and(|operation| allows_operation(role, operation))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Unlock,
    Lock,
    List,
    Reveal,
    Copy,
    Create,
    Update,
    Rotate,
    SoftDelete,
    Purge,
    AuditView,
    VerifyAuditChain,
    RotateMasterKey,
    Migrate,
}

impl Operation {
    pub const ALL: [Self; 14] = [
        Self::Unlock,
        Self::Lock,
        Self::List,
        Self::Reveal,
        Self::Copy,
        Self::Create,
        Self::Update,
        Self::Rotate,
        Self::SoftDelete,
        Self::Purge,
        Self::AuditView,
        Self::VerifyAuditChain,
        Self::RotateMasterKey,
        Self::Migrate,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unlock => "unlock",
            Self::Lock => "lock",
            Self::List => "list",
            Self::Reveal => "reveal",
            Self::Copy => "copy",
            Self::Create => "create",
            Self::Update => "update",
            Self::Rotate => "rotate",
            Self::SoftDelete => "soft_delete",
            Self::Purge => "purge",
            Self::AuditView => "audit_view",
            Self::VerifyAuditChain => "verify_audit_chain",
            Self::RotateMasterKey => "rotate_master_key",
            Self::Migrate => "migrate",
        }
    }
}

#[must_use]
pub const fn allows_operation(role: Role, operation: Operation) -> bool {
    match operation {
        Operation::Unlock | Operation::Lock => true,
        Operation::List => matches!(
            role,
            Role::Owner | Role::Admin | Role::Editor | Role::Viewer | Role::Auditor
        ),
        Operation::Reveal | Operation::Copy => matches!(
            role,
            Role::Owner | Role::Admin | Role::Editor | Role::Viewer
        ),
        Operation::Create | Operation::Update | Operation::Rotate => {
            matches!(role, Role::Owner | Role::Admin | Role::Editor)
        }
        Operation::SoftDelete => matches!(role, Role::Owner | Role::Admin),
        Operation::Purge | Operation::RotateMasterKey => matches!(role, Role::Owner),
        Operation::AuditView | Operation::VerifyAuditChain => {
            matches!(role, Role::Owner | Role::Admin | Role::Auditor)
        }
        Operation::Migrate => matches!(role, Role::Owner | Role::Admin),
    }
}

#[must_use]
pub fn operation_from_str(op: &str) -> Option<Operation> {
    match op {
        "unlock" => Some(Operation::Unlock),
        "lock" => Some(Operation::Lock),
        "list" => Some(Operation::List),
        "reveal" => Some(Operation::Reveal),
        "copy" => Some(Operation::Copy),
        "create" => Some(Operation::Create),
        "update" => Some(Operation::Update),
        "rotate" => Some(Operation::Rotate),
        "soft_delete" => Some(Operation::SoftDelete),
        "purge" => Some(Operation::Purge),
        "audit_view" => Some(Operation::AuditView),
        "verify_audit_chain" => Some(Operation::VerifyAuditChain),
        "rotate_master_key" | "rotate-master-key" => Some(Operation::RotateMasterKey),
        "migrate" => Some(Operation::Migrate),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{allows, allows_operation, Operation};
    use vaultcore_core::Role;

    const MATRIX: &[(Operation, &[Role])] = &[
        (
            Operation::Unlock,
            &[
                Role::Owner,
                Role::Admin,
                Role::Editor,
                Role::Viewer,
                Role::Auditor,
            ],
        ),
        (
            Operation::Lock,
            &[
                Role::Owner,
                Role::Admin,
                Role::Editor,
                Role::Viewer,
                Role::Auditor,
            ],
        ),
        (
            Operation::List,
            &[
                Role::Owner,
                Role::Admin,
                Role::Editor,
                Role::Viewer,
                Role::Auditor,
            ],
        ),
        (
            Operation::Reveal,
            &[Role::Owner, Role::Admin, Role::Editor, Role::Viewer],
        ),
        (
            Operation::Copy,
            &[Role::Owner, Role::Admin, Role::Editor, Role::Viewer],
        ),
        (Operation::Create, &[Role::Owner, Role::Admin, Role::Editor]),
        (Operation::Update, &[Role::Owner, Role::Admin, Role::Editor]),
        (Operation::Rotate, &[Role::Owner, Role::Admin, Role::Editor]),
        (Operation::SoftDelete, &[Role::Owner, Role::Admin]),
        (Operation::Purge, &[Role::Owner]),
        (
            Operation::AuditView,
            &[Role::Owner, Role::Admin, Role::Auditor],
        ),
        (
            Operation::VerifyAuditChain,
            &[Role::Owner, Role::Admin, Role::Auditor],
        ),
        (Operation::RotateMasterKey, &[Role::Owner]),
        (Operation::Migrate, &[Role::Owner, Role::Admin]),
    ];

    #[test]
    fn policy_covers_every_operation() {
        assert_eq!(Operation::ALL.len(), MATRIX.len());
        for operation in Operation::ALL {
            assert!(
                MATRIX.iter().any(|(candidate, _)| *candidate == operation),
                "missing matrix row for {operation:?}"
            );
        }
    }

    #[test]
    fn policy_allows_exact_matrix_cells() {
        for (operation, allowed_roles) in MATRIX {
            for role in Role::ALL {
                assert_eq!(
                    allows_operation(role, *operation),
                    allowed_roles.contains(&role),
                    "unexpected decision for {role:?} / {operation:?}"
                );
            }
        }
    }

    #[test]
    fn policy_default_denies_unknown_operations() {
        assert!(!allows(Role::Owner, "unknown"));
        assert!(!allows(Role::Auditor, "reveal_payload_without_reason"));
    }

    #[test]
    fn policy_accepts_spec_wire_names() {
        for operation in Operation::ALL {
            assert!(
                allows(Role::Owner, operation.as_str()),
                "owner should be allowed for {}",
                operation.as_str()
            );
        }
        assert!(allows(Role::Owner, "rotate-master-key"));
    }

    #[test]
    fn auditor_is_metadata_and_audit_only() {
        assert!(allows_operation(Role::Auditor, Operation::List));
        assert!(allows_operation(Role::Auditor, Operation::AuditView));
        assert!(allows_operation(Role::Auditor, Operation::VerifyAuditChain));
        assert!(!allows_operation(Role::Auditor, Operation::Reveal));
        assert!(!allows_operation(Role::Auditor, Operation::Copy));
        assert!(!allows_operation(Role::Auditor, Operation::Create));
    }
}
