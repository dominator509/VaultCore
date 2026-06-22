#!/usr/bin/env sh
set -eu
. ./scripts/env.sh
[ -f "AGENTS.md" ] || { echo "ERROR: run from repository root." >&2; exit 1; }
ARTIFACT_DIR="target/release-artifacts"
MANIFEST="$ARTIFACT_DIR/SHA256SUMS.txt"
rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR"
cargo build --workspace --release
if [ -f "app/package.json" ]; then
  (cd app && pnpm build)
  (cd app && pnpm tauri build) || echo "WARNING: tauri build skipped or platform prereqs missing"
fi

find target/release -maxdepth 4 -type f \( \
  -name "vaultcore-app" -o \
  -name "vaultcore-app.exe" -o \
  -name "VaultCore_*.msi" -o \
  -name "*.AppImage" -o \
  -name "*.deb" -o \
  -name "*.rpm" -o \
  -name "*.dmg" \
\) -exec cp {} "$ARTIFACT_DIR"/ \;

if [ -z "$(find "$ARTIFACT_DIR" -type f ! -name 'SHA256SUMS.txt' -print -quit)" ]; then
  echo "ERROR: no release artifacts found." >&2
  exit 1
fi

(
  cd "$ARTIFACT_DIR"
  find . -maxdepth 1 -type f ! -name "SHA256SUMS.txt" -print0 \
    | sort -z \
    | xargs -0 sha256sum > SHA256SUMS.txt
)

echo "sha256 manifest: $MANIFEST"
echo "build: ok"
