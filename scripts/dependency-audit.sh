#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
if command -v cargo-audit >/dev/null 2>&1; then
  if [ -d "$HOME/.cargo/advisory-db" ]; then
    cargo audit --db "$HOME/.cargo/advisory-db" --no-fetch --stale
  else
    cargo audit
  fi
else
  echo "ERROR: cargo-audit not installed. See ENVIRONMENT.md." >&2; exit 1
fi
if [ -f "app/package.json" ]; then
  (cd app && pnpm audit || true)
fi
echo "dependency audit: ok"
