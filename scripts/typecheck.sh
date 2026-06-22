#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
cargo check --workspace --all-targets
if [ -f "app/package.json" ]; then
  (cd app && pnpm typecheck)
fi
echo "typecheck: ok"
