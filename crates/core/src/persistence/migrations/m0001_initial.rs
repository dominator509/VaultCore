use rusqlite::Connection;

pub(crate) fn apply(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS secrets (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL CHECK (
                type IN ('API_KEY', 'LOGIN', 'OAUTH_APP', 'SSH_KEY', 'WALLET_KEY', 'CERT', 'NOTE', 'BLOB')
            ),
            name TEXT NOT NULL CHECK (length(name) > 0),
            labels TEXT NOT NULL CHECK (json_valid(labels)),
            state TEXT NOT NULL CHECK (
                state IN ('draft', 'active', 'expiring_soon', 'expired', 'rotating', 'archived', 'soft_deleted', 'purged')
            ),
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            expires_at INTEGER NULL,
            payload_envelope BLOB NULL,
            payload_dek_id TEXT NULL,
            meta TEXT NOT NULL CHECK (json_valid(meta)),
            CHECK (
                state != 'purged'
                OR (payload_envelope IS NULL AND payload_dek_id IS NULL)
            )
        );

        CREATE INDEX IF NOT EXISTS idx_secrets_type ON secrets(type);
        CREATE INDEX IF NOT EXISTS idx_secrets_state ON secrets(state);
        CREATE INDEX IF NOT EXISTS idx_secrets_name ON secrets(name);
        CREATE INDEX IF NOT EXISTS idx_secrets_expires_at ON secrets(expires_at);

        CREATE TABLE IF NOT EXISTS audit_log (
            seq INTEGER PRIMARY KEY AUTOINCREMENT,
            ts INTEGER NOT NULL,
            actor TEXT NOT NULL,
            op TEXT NOT NULL,
            target_id TEXT NULL,
            result TEXT NOT NULL CHECK (result IN ('ok', 'denied', 'error')),
            prior_hash BLOB NOT NULL CHECK (length(prior_hash) = 32),
            payload_hash BLOB NOT NULL CHECK (length(payload_hash) = 32),
            entry_hash BLOB NOT NULL CHECK (length(entry_hash) = 32),
            countersignature BLOB NOT NULL
        );

        CREATE TABLE IF NOT EXISTS specanchor_meta (
            version TEXT PRIMARY KEY,
            loaded_at INTEGER NOT NULL
        );
        ",
    )
}
