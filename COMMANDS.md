# COMMANDS.md

## Working Directory Rule
All commands must be run from repository root.

## Package Manager Rule
- Rust crates (`crates/core`, `crates/builder`, `crates/verifier`, `crates/cli`): `cargo`
- TypeScript app (`app/`): `pnpm`
- Tauri shell: invoked via `pnpm tauri ...` from `app/`

Coding agents must not invent commands. If a command is missing, update this file first with evidence from the repository (manifests, CI workflow files, scripts).

## Canonical Commands
Use the wrappers in `scripts/` as the canonical entry points. They forward to the underlying tools.

### Install
```sh
./scripts/install.sh
```
Internally:
- `cargo fetch` for Rust workspace
- `pnpm install --frozen-lockfile` in `app/`

### Preflight
```sh
./scripts/preflight.sh
```

### Lint
```sh
./scripts/lint.sh
```
Internally:
- `cargo clippy --workspace --all-targets -- -D warnings`
- `pnpm --dir app lint`

### Format Check
```sh
./scripts/format-check.sh
```
Internally:
- `cargo fmt --all -- --check`
- `pnpm --dir app format:check`

### Typecheck / Static Validation
```sh
./scripts/typecheck.sh
```
Internally:
- `cargo check --workspace --all-targets`
- `pnpm --dir app typecheck`

### Unit Tests
```sh
./scripts/test-unit.sh
```
Internally:
- `cargo nextest run --workspace --lib --bins` (fallback: `cargo test --workspace --lib --bins`)
- `pnpm --dir app test:unit`

### Integration Tests
```sh
./scripts/test-integration.sh
```
Internally:
- `cargo nextest run --workspace --test '*'` (covers Builder↔Verifier IPC, persistence)
- `pnpm --dir app test:integration`

### E2E / Acceptance Tests
```sh
./scripts/test-e2e.sh
```
Internally:
- `pnpm --dir app test:e2e` (Playwright against a built Tauri dev binary)

### Accessibility Tests
```sh
pnpm --dir app test:a11y
```
Internally:
- `@axe-core/playwright` scans primary UI flows and fails on serious/critical violations.

### Build
```sh
./scripts/build.sh
```
Internally:
- `cargo build --workspace --release`
- `pnpm --dir app build`
- `pnpm --dir app tauri build` for the desktop bundle

### Security Check
```sh
./scripts/security-check.sh
```
Internally:
- `cargo deny check advisories bans sources licenses` (config in `deny.toml`)
- Custom invariant tests: `cargo nextest run --workspace --test invariants`
- TS audit: `pnpm --dir app audit --prod`

### Dependency Audit
```sh
./scripts/dependency-audit.sh
```
Internally:
- `cargo audit`
- `pnpm --dir app audit`

### Smoke Test
```sh
./scripts/smoke-test.sh
```
Internally:
- Runs Builder and Verifier local observability contract tests that assert `/health/builder`, `/health/verifier`, and `/metrics` exposure without remote network access.

### Full Verification
```sh
./scripts/verify.sh
```
Runs the entire local validation chain.

### Production Readiness Check
```sh
./scripts/production-readiness-check.sh
```
Verifies invariants I-1..I-8 enforcement tests, threat coverage map (T-001..T-023), TRACEABILITY status, and release-gate evidence.

### Local Development
```sh
pnpm --dir app tauri dev
```
This starts the Builder and Verifier processes in dev mode against a sandboxed SpecAnchor and a temporary local vault under `./.vaultcore-dev/`.

### Local Database Setup
Not applicable as a separate step. SQLite vault files are created on first unlock under `./.vaultcore-dev/vault.db`. Schema migrations are embedded in `crates/core/src/persistence/migrations/`.

### Migrations
```sh
cargo run -p vaultcore-cli -- migrate --dry-run
cargo run -p vaultcore-cli -- migrate --apply
```
Migrations must be additive. Destructive migrations require an approved ADR and a rollback path in `ROLLBACK.md`.

### SpecAnchor Operations (offline only)
```sh
cargo run -p vaultcore-cli -- specanchor generate --out ./specanchor.signed
cargo run -p vaultcore-cli -- specanchor verify --in ./specanchor.signed
```
SpecAnchor generation is offline-only and requires the project signing key. It is never run in user runtime.

## Forbidden Commands
Do not use:
- `git clean -fdx`
- Force pushes that rewrite shared history
- `cargo install` for crypto crates from unverified sources
- Any command that uploads vault data, audit logs, or SpecAnchor material to a remote service
- Any command that disables `clippy` warnings, `cargo deny`, or invariant tests
- Direct edits to `Cargo.lock` or `pnpm-lock.yaml` outside of normal dependency management

## Recovery Instructions
If a command fails:
1. Confirm you are at repository root.
2. Confirm the command is not a placeholder.
3. Inspect manifests (`Cargo.toml`, `app/package.json`) and CI for the canonical command.
4. Update this file and the corresponding script using repository evidence.
5. Rerun the narrowest relevant command.
6. Apply bounded retry rules from `AGENTS.md`.
