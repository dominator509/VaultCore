# Validation Checklist (VaultCore)

- [ ] `./scripts/lint.sh` (clippy + pnpm lint)
- [ ] `./scripts/format-check.sh` (cargo fmt + pnpm format:check)
- [ ] `./scripts/typecheck.sh` (cargo check + pnpm typecheck)
- [ ] `./scripts/test-unit.sh` (cargo nextest libs + Vitest unit)
- [ ] `./scripts/test-integration.sh` (cargo nextest integration + pnpm integration)
- [ ] `./scripts/test-e2e.sh` (Playwright against dev binary)
- [ ] `./scripts/build.sh` (cargo release + pnpm build + tauri build)
- [ ] `./scripts/security-check.sh` (cargo deny + invariant tests + pnpm audit --prod)
- [ ] `./scripts/dependency-audit.sh` (cargo audit + pnpm audit)
- [ ] `./scripts/smoke-test.sh` (boot Builder+Verifier, unlock, reveal, verify chain)
- [ ] `./scripts/verify.sh` (full chain)
- [ ] `cargo nextest run --test invariants` (I-1..I-8 enforcement suite)
- [ ] `cargo nextest run --test threats` (threat coverage references)

If a command is not applicable, document it explicitly in `COMMANDS.md` and the active ExecPlan. Never skip silently.
