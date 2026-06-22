use rusqlite::Connection;

pub(crate) fn apply(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS migration_continuity_marker (
            version INTEGER PRIMARY KEY,
            note TEXT NOT NULL
        );
        INSERT OR IGNORE INTO migration_continuity_marker (version, note)
            VALUES (2, 'additive migration continuity fixture');
        ",
    )
}
