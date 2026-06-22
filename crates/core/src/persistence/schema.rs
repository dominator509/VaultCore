use rusqlite::Connection;

use crate::{
    error::{VaultError, VaultErrorCode},
    persistence::migrations,
};

/// Apply the `SQLite` connection PRAGMAs required by SPEC-002.
///
/// # Errors
///
/// Returns `VaultErrorCode::PersistenceFailure` when `SQLite` rejects any PRAGMA.
pub fn apply_pragmas(connection: &Connection) -> Result<(), VaultError> {
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .and_then(|()| connection.pragma_update(None, "journal_mode", "WAL"))
        .and_then(|()| connection.pragma_update(None, "synchronous", "NORMAL"))
        .map_err(persistence_error)
}

/// Apply all known additive migrations.
///
/// # Errors
///
/// Returns `VaultErrorCode::PersistenceFailure` when any migration fails.
pub fn migrate(connection: &mut Connection) -> Result<(), VaultError> {
    apply_pragmas(connection)?;
    migrations::apply_all(connection)
}

pub(crate) fn persistence_error(error: rusqlite::Error) -> VaultError {
    let message = error.to_string();
    drop(error);
    VaultError::new(
        VaultErrorCode::PersistenceFailure,
        None,
        format!("persistence operation failed: {message}"),
    )
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::migrate;

    #[test]
    fn migrations_are_idempotent() {
        let mut connection = Connection::open_in_memory().expect("open db");
        migrate(&mut connection).expect("first migration run");
        migrate(&mut connection).expect("second migration run");

        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM migrations", [], |row| row.get(0))
            .expect("count migrations");
        assert_eq!(count, 2);
    }
}
