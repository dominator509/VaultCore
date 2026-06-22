use vaultcore_core::Role;

#[must_use]
pub fn allows(role: Role, op: &str) -> bool {
    match op {
        "list" | "reveal" | "copy" => role.satisfies_minimum(Role::Viewer),
        "create" | "update" | "rotate" => role.satisfies_minimum(Role::Editor),
        "soft_delete" => role.satisfies_minimum(Role::Admin),
        "purge" => matches!(role, Role::Owner),
        "audit_view" => matches!(role, Role::Auditor),
        _ => false,
    }
}
