#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
[ -f "PRODUCTION_READINESS.md" ] || { echo "ERROR: PRODUCTION_READINESS.md missing." >&2; exit 1; }
[ -f "DEPLOYMENT.md" ] || { echo "ERROR: DEPLOYMENT.md missing." >&2; exit 1; }
[ -f "ROLLBACK.md" ] || { echo "ERROR: ROLLBACK.md missing." >&2; exit 1; }
[ -f "THREAT_MODEL.md" ] || { echo "ERROR: THREAT_MODEL.md missing." >&2; exit 1; }
[ -f "TRACEABILITY.md" ] || { echo "ERROR: TRACEABILITY.md missing." >&2; exit 1; }

./scripts/verify.sh

# Invariant suite is required for production readiness.
if command -v cargo-nextest >/dev/null 2>&1; then
  cargo nextest run --workspace --test invariants
fi

# Threat coverage suite (added in EP-007).
if cargo nextest list --workspace --test threats >/dev/null 2>&1; then
  cargo nextest run --workspace --test threats
fi

echo "production readiness: ok"
