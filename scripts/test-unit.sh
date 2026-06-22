#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
if command -v cargo-nextest >/dev/null 2>&1; then
  cargo nextest run --workspace --lib --bins
else
  cargo test --workspace --lib --bins
fi
if [ -f "app/package.json" ]; then
  (cd app && pnpm test:unit)
fi
echo "unit tests: ok"
