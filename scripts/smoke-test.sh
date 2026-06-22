#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }

# Boots Builder+Verifier observability contracts with a fixture SpecAnchor shape.
# EP-008 keeps introspection local-only; unit contracts assert /health/* and /metrics exposure.
cargo nextest run -p vaultcore-builder obs
cargo nextest run -p vaultcore-verifier obs

echo "smoke test: ok"
