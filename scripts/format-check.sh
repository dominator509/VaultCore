#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
cargo fmt --all -- --check
if [ -f "app/package.json" ]; then
  (cd app && pnpm format:check)
fi
echo "format check: ok"
