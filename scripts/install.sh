#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }

if [ -f "Cargo.toml" ]; then
  cargo fetch
fi

if [ -f "app/package.json" ]; then
  if [ -f "app/pnpm-lock.yaml" ]; then
    (cd app && pnpm install --frozen-lockfile)
  else
    (cd app && pnpm install --no-frozen-lockfile)
  fi
fi

echo "install: ok"
