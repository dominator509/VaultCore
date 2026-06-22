#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
if command -v cargo-deny >/dev/null 2>&1; then
  if [ -d "$HOME/.cargo/advisory-dbs/advisory-db-3157b0e258782691/.git" ]; then
    cargo deny check --disable-fetch advisories bans sources licenses
  else
    cargo deny check advisories bans sources licenses
  fi
else
  echo "ERROR: cargo-deny not installed. See ENVIRONMENT.md." >&2; exit 1
fi
if command -v cargo-nextest >/dev/null 2>&1; then
  cargo nextest run --workspace --test invariants || { echo "ERROR: invariant suite failed." >&2; exit 1; }
fi
if [ -f "app/package.json" ]; then
  (cd app && pnpm audit --prod || true)
fi
echo "security check: ok"
