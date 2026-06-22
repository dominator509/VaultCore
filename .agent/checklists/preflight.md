# Preflight Checklist (VaultCore)

- [ ] Working directory is repository root.
- [ ] `AGENTS.md`, `COMMANDS.md`, `.agent/`, `scripts/`, `docs/` present.
- [ ] `rustup` available; toolchain in `rust-toolchain.toml` installs.
- [ ] `pnpm` available at the version pinned in `app/package.json`.
- [ ] Tauri platform prerequisites installed for the current OS.
- [ ] `cargo-nextest`, `cargo-deny`, `cargo-audit` available (install hints in ENVIRONMENT.md).
- [ ] `VAULTCORE_DEV_DIR` writable (default `./.vaultcore-dev`).
- [ ] No real user vault present anywhere reachable by tests.
- [ ] Signing keys not present in working tree (release-only secrets live in CI).
- [ ] Known blockers documented before implementation begins.
