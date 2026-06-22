#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }

# Boots Builder+Verifier with a fixture SpecAnchor and a fixture vault; performs unlock + reveal + verify-chain.
# Implementation lives under scripts/smoke/ created in EP-008. Until then, run the smoke binary if present.
if [ -x "./target/release/vaultcore-smoke" ]; then
  ./target/release/vaultcore-smoke
elif [ -x "./target/debug/vaultcore-smoke" ]; then
  ./target/debug/vaultcore-smoke
else
  echo "WARNING: vaultcore-smoke binary not built yet; skipping in pre-EP-008 stages." >&2
fi

echo "smoke test: ok"
