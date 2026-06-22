#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
if [ "${VAULTCORE_E2E_SKIP:-0}" = "1" ]; then
  echo "e2e tests: skipped (VAULTCORE_E2E_SKIP=1)"
  exit 0
fi
if [ -f "app/package.json" ]; then
  (cd app && pnpm test:e2e)
fi
echo "e2e tests: ok"
