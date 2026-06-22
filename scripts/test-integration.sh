#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
if command -v cargo-nextest >/dev/null 2>&1; then
  cargo nextest run --workspace --test '*'
else
  cargo test --workspace --tests
fi
if [ -f "app/package.json" ]; then
  (cd app && pnpm test:integration)
fi
echo "integration tests: ok"
