#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
cargo clippy --workspace --all-targets -- -D warnings
if [ -f "app/package.json" ]; then
  (cd app && pnpm lint)
fi
echo "lint: ok"
