# Changelog

All notable changes to VaultCore are tracked here.

## 0.1.0-rc.1 - Unreleased

### User-Visible Changes
- Initial local-first Tauri vault shell with lock, unlock, list/search, reveal, copy, lifecycle, audit, and Vault Health flows.
- Local Vault Health view surfaces SpecAnchor status, audit head status, active session, and local alert states.

### Security Changes
- Six-layer architecture scaffolding enforces the Builder and Verifier Trinity boundary.
- Sealed crypto wrappers, signed SpecAnchor verification, signed Trinity IPC frames, RBAC default-deny policy, auth lockout, and invariant tests are in place.
- Observability is local-only with shared redaction and a no-network sentinel.

### Schema / IPC / Migration Notes
- SQLite persistence schema is additive and audit rows are hash-chained.
- Trinity IPC schema is signed, length-prefixed, and replay-protected.
- No destructive migration is included in this release candidate.

### Known Risks
- Production signing credentials and updater-channel publish are owner-gated and not exercised locally.
- Windows MSI bundling can report `Access is denied. (os error 5)` on this local machine; `scripts/build.sh` still emits a manifest for available release artifacts.

### Rollback Notes
- Use `ROLLBACK.md` and `.agent/runbooks/rollback-drill.md`; pause updater publication before replacing bundles.
