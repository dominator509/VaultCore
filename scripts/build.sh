#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
cargo build --workspace --release
if [ -f "app/package.json" ]; then
  (cd app && pnpm build)
  (cd app && pnpm tauri build) || echo "WARNING: tauri build skipped or platform prereqs missing"
fi
echo "build: ok"
