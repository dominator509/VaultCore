# EP-001 Foundation

## 1. Purpose / Big Picture
Stand up the workspace: Cargo workspace with `crates/{core,builder,verifier,cli,tests/invariants}`, TS workspace under `app/` with Tauri shell, formatter/linter/typechecker configs, baseline CI, and the script chain that produces `verify: ok` against an empty workspace.

## 2. Scope
- Create Cargo workspace and crate skeletons (binaries empty, libs empty).
- Create `app/` with Tauri scaffold and TS toolchain.
- Wire `cargo clippy`, `cargo fmt`, `cargo nextest`, `cargo deny`, `cargo audit`.
- Wire `pnpm` scripts: `lint`, `format:check`, `typecheck`, `test:unit`, `test:integration`, `test:e2e`, `build`.
- Add `deny.toml` with crypto allowlist (audited crates only).
- Add baseline GitHub Actions workflow running `./scripts/verify.sh` on macOS, Windows, Linux.
- Add `rust-toolchain.toml`, `.editorconfig`, `.gitignore`.

## 3. Non-goals
- No domain logic.
- No crypto implementation.
- No UI flows beyond a placeholder Tauri window.
- No production signing.

## 4. Context and Orientation
Run after EP-000. Greenfield repo; smallest reversible scaffolding only.

## 5. Files to Read First
- `AGENTS.md`, `COMMANDS.md`, `ARCHITECTURE.md`, `ENVIRONMENT.md`, `TESTING.md`
- `.agent/specs/SPEC-000-product-scope.md`
- Uploaded `Architecture.md` for layer/dependency rules

## 6. Files to Change
- `Cargo.toml` (workspace)
- `rust-toolchain.toml`
- `deny.toml`
- `crates/core/Cargo.toml`, `crates/core/src/lib.rs` (empty + module placeholders)
- `crates/builder/Cargo.toml`, `crates/builder/src/main.rs` (empty bin)
- `crates/verifier/Cargo.toml`, `crates/verifier/src/main.rs` (empty bin)
- `crates/cli/Cargo.toml`, `crates/cli/src/main.rs` (empty bin)
- `crates/tests/invariants/Cargo.toml`, placeholder test
- `app/package.json`, `app/tsconfig.json`, `app/src/main.tsx`, `app/src-tauri/{tauri.conf.json,src/main.rs}`
- `.github/workflows/ci.yml`
- `.editorconfig`, `.gitignore`
- `COMMANDS.md` (any final command corrections)
- This ExecPlan

## 7. Interfaces and Contracts
- Developer/agent workflow only; no end-user behavior yet.

## 8. Milestones

### Milestone 1 — Cargo workspace + Rust toolchain
- **Goal:** Workspace compiles empty.
- **Files to Read:** `ARCHITECTURE.md`, ADR-0004.
- **Files to Change:** `Cargo.toml`, `rust-toolchain.toml`, all `crates/*/Cargo.toml`, empty `lib.rs`/`main.rs`.
- **Exact Edits Expected:** Members listed; pinned toolchain.
- **Validation Command:** `cargo build --workspace`
- **Expected Result:** Builds cleanly.
- **Recovery Instruction:** If a member fails, fix the smallest manifest issue; do not add features.

### Milestone 2 — Lint, format, typecheck, deny, audit
- **Goal:** Wire static-validation chain.
- **Files to Read:** existing configs (none yet).
- **Files to Change:** `deny.toml`, `app/package.json`, `app/tsconfig.json`, `scripts/lint.sh`, `scripts/format-check.sh`, `scripts/typecheck.sh`, `scripts/security-check.sh`, `scripts/dependency-audit.sh`.
- **Exact Edits Expected:** Scripts forward to `cargo clippy/fmt/check/deny/audit` and `pnpm` equivalents.
- **Validation Command:** `./scripts/lint.sh && ./scripts/format-check.sh && ./scripts/typecheck.sh`
- **Expected Result:** All three exit 0 on empty workspace.
- **Recovery Instruction:** Isolate failing tool; do not loosen rules.

### Milestone 3 — Test harness and verify chain
- **Goal:** Get `./scripts/verify.sh` green on empty workspace.
- **Files to Read:** `TESTING.md`.
- **Files to Change:** `scripts/test-*.sh`, `scripts/build.sh`, `scripts/smoke-test.sh`, `scripts/verify.sh`, placeholder integration test in `crates/tests/invariants/`.
- **Exact Edits Expected:** Scripts forward to `cargo nextest` and `pnpm test:*`; placeholder Playwright config compiles without browsers (Playwright install in EP-005).
- **Validation Command:** `./scripts/verify.sh`
- **Expected Result:** `verify: ok`.
- **Recovery Instruction:** If E2E tooling missing, mark Playwright skipped behind explicit `VAULTCORE_E2E_SKIP=1` (test plan installs in EP-005).

### Milestone 4 — Baseline CI and Tauri scaffold
- **Goal:** CI runs `./scripts/verify.sh` per platform; Tauri opens an empty window in dev.
- **Files to Read:** `.github/workflows/` (none yet), Tauri docs (offline).
- **Files to Change:** `.github/workflows/ci.yml`, `app/src-tauri/tauri.conf.json`, `app/src-tauri/src/main.rs`, `app/src/main.tsx`.
- **Exact Edits Expected:** Matrix job per platform; Tauri shell minimal; UI renders placeholder lock screen.
- **Validation Command:** `pnpm --dir app tauri dev` (manual; gated by `VAULTCORE_DEV=1`)
- **Expected Result:** Window opens; placeholder visible.
- **Recovery Instruction:** Tauri prereqs missing → document in ENVIRONMENT.md; do not bypass.

## 9. Concrete Steps
1. Create workspace and crates.
2. Wire static validation.
3. Wire tests and verify.
4. Wire CI and Tauri shell.
5. Update docs and progress.

## 10. Validation and Acceptance
- `./scripts/verify.sh` green locally and in CI on all three OSes.
- `deny.toml` enforces crypto allowlist.

## 11. Idempotence and Recovery
- Manifests and configs are additive.
- Do not introduce features into placeholders.

## 12. Progress
- [x] Milestone 1 complete (Cargo workspace, pinned Rust toolchain, core/builder/verifier/cli/invariants crates, and Tauri crate compile).
- [x] Milestone 2 complete (lint, format, typecheck, deny, and audit scripts wired and passing under `./scripts/verify.sh`).
- [x] Milestone 3 complete (unit, integration, e2e placeholder, invariant placeholder, build, audit, smoke, and full verify chain green).
- [x] Milestone 4 complete (GitHub Actions baseline added; Tauri shell builds and bundles MSI/NSIS locally).

## 13. Surprises & Discoveries
- WSL network/TLS behavior made RustSec and crate fetches flaky; the local RustSec advisory database was seeded, and `scripts/security-check.sh` now uses cargo-deny offline mode when that cache is present.
- Tauri v2 requires `app/src-tauri` to be a Cargo workspace member for workspace builds and metadata checks.
- Tauri Windows packaging requires an explicit `.ico` icon; the scaffold now includes both `icons/icon.png` and `icons/icon.ico`.
- `cargo-deny` v0.19 uses advisory scopes for `unmaintained`; EP-001 sets `unmaintained = "workspace"` so known transitive Tauri Linux GTK3 maintenance advisories do not block the empty scaffold.
- `cargo audit` reports warning-class transitive advisories from the current Tauri Linux stack and one low esbuild dev-server advisory, while `cargo deny` reports no known vulnerabilities.

## 14. Decision Log
- ADR-0004 accepted at this stage.
- ADR-0009 accepted (cargo nextest + Vitest + Playwright).
- Tauri v2 config follows current Tauri app docs: Vite frontend build, Rust `tauri::Builder::default()`, and `tauri.conf.json` `devUrl`/`frontendDist` wiring.
- Added generated/build-output ignores for Prettier (`app/dist`, `app/src-tauri/gen`) so `format-check` validates source files only.
- Allowed observed OSI licenses needed by the Tauri dependency graph in `deny.toml`: MPL-2.0, Zlib, and Apache-2.0 WITH LLVM-exception, while keeping workspace crates private via `publish = false` and `licenses.private.ignore = true`.
- Extra scaffold files beyond the initial list were required by the selected tools: `Cargo.lock`, `app/pnpm-lock.yaml`, `app/index.html`, `app/vite.config.ts`, `app/src/styles.css`, app placeholder tests, `app/.prettierignore`, `app/src-tauri/Cargo.toml`, `app/src-tauri/build.rs`, and Tauri icon files.

## 15. Outcomes & Retrospective
- EP-001 completed locally. `./scripts/verify.sh` exits `verify: ok`.
- Tauri produced local Windows bundles at `target/release/bundle/msi/VaultCore_0.1.0_x64_en-US.msi` and `target/release/bundle/nsis/VaultCore_0.1.0_x64-setup.exe`.
- No domain, crypto, IPC, vault data, production signing, or production deployment behavior was introduced.
- Residual dependency warnings are documented for later hardening: transitive Tauri Linux GTK3 maintenance warnings, `glib` warning-class RustSec advisory, duplicate transitive crates, and low esbuild dev-server advisory.
