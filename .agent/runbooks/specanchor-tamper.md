# SpecAnchor Tamper Runbook

## Trigger
Vault Health reports `SpecAnchor tamper detected`, startup refuses to proceed, or `specanchor verify` fails for a shipped bundle.

## Immediate Actions
1. Stop using the affected bundle.
2. Do not modify the local vault file or audit chain.
3. Capture the version, platform, SpecAnchor file hash, and the exact error code.
4. Run `cargo run -p vaultcore-cli -- specanchor verify --in tests/fixtures/specanchor.signed` on a clean checkout to confirm the verifier path.

## Recovery
1. Reinstall the last known-good signed release.
2. Preserve local logs from the platform log directory or `VAULTCORE_DEV_DIR/logs/`.
3. Open an incident entry with the captured version and SpecAnchor hash.
4. Resume release work only after `./scripts/verify.sh` and `cargo nextest run --test no_network` pass.

## Escalation
Treat as Sev-1 because it can affect invariant I-6. Pause promotion of the updater channel until owner approval.
