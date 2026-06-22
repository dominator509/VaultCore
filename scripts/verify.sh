#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }

./scripts/preflight.sh
./scripts/lint.sh
./scripts/format-check.sh
./scripts/typecheck.sh
./scripts/test-unit.sh
./scripts/test-integration.sh
./scripts/test-e2e.sh
./scripts/build.sh
./scripts/security-check.sh
./scripts/dependency-audit.sh
./scripts/smoke-test.sh

echo "verify: ok"
