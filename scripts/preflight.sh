#!/usr/bin/env sh
set -eu
. ./scripts/env.sh

[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root (AGENTS.md missing)." >&2; exit 1; }
[ -f "COMMANDS.md" ] || { echo "ERROR: COMMANDS.md missing." >&2; exit 1; }
[ -d ".agent" ] || { echo "ERROR: .agent directory missing." >&2; exit 1; }
[ -d "scripts" ] || { echo "ERROR: scripts directory missing." >&2; exit 1; }

for f in scripts/install.sh scripts/lint.sh scripts/format-check.sh scripts/typecheck.sh scripts/test-unit.sh scripts/test-integration.sh scripts/test-e2e.sh scripts/build.sh scripts/security-check.sh scripts/dependency-audit.sh scripts/smoke-test.sh scripts/verify.sh scripts/production-readiness-check.sh; do
  [ -f "$f" ] || { echo "ERROR: required script missing: $f" >&2; exit 1; }
done

command -v cargo >/dev/null 2>&1 || { echo "ERROR: cargo not found. Install rustup and the toolchain in rust-toolchain.toml." >&2; exit 1; }
command -v pnpm  >/dev/null 2>&1 || { echo "ERROR: pnpm not found. Install pnpm at the version pinned in app/package.json." >&2; exit 1; }

if [ -d "app" ] && [ ! -f "app/package.json" ]; then
  echo "WARNING: app/ exists but app/package.json is missing." >&2
fi

if [ -d ".vaultcore-dev" ] || [ -n "${VAULTCORE_DEV_DIR:-}" ]; then
  : # dev dir managed by Tauri dev at first run
fi

echo "preflight: ok"
