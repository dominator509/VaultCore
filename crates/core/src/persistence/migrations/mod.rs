mod m0001_initial;
mod m0002_noop;

use rusqlite::{params, Connection};

use crate::{error::VaultError, persistence::schema::persistence_error};

struct Migration {
    version: i64,
    apply: fn(&Connection) -> Result<(), rusqlite::Error>,
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        apply: m0001_initial::apply,
    },
    Migration {
        version: 2,
        apply: m0002_noop::apply,
    },
];

pub(crate) fn apply_all(connection: &mut Connection) -> Result<(), VaultError> {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY,
                applied_at INTEGER NOT NULL
            )",
            [],
        )
        .map_err(persistence_error)?;

    for migration in MIGRATIONS {
        if !is_applied(connection, migration.version)? {
            let transaction = connection.transaction().map_err(persistence_error)?;
            (migration.apply)(&transaction).map_err(persistence_error)?;
            transaction
                .execute(
                    "INSERT INTO migrations (version, applied_at) VALUES (?1, unixepoch('subsec') * 1000)",
                    params![migration.version],
                )
                .map_err(persistence_error)?;
            transaction.commit().map_err(persistence_error)?;
        }
    }

    Ok(())
}

fn is_applied(connection: &Connection, version: i64) -> Result<bool, VaultError> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM migrations WHERE version = ?1",
            params![version],
            |row| row.get(0),
        )
        .map_err(persistence_error)?;
    Ok(count == 1)
}
