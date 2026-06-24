# DEPLOYMENT.md

## Deployment Environments
- **Local dev:** developer machine, `pnpm tauri dev`.
- **CI verification:** GitHub Actions jobs running `./scripts/verify.sh` per platform.
- **Release candidate:** signed Tauri bundles produced from the `release-candidate` branch.
- **Production:** user-installed Tauri application; updates delivered via the signed Tauri updater channel.

There is no production server.

## Deployment Architecture
- Single signed Tauri bundle per platform (macOS `.dmg` / `.app`, Windows `.msi` / signed `.exe`, Linux `.AppImage` / `.deb` / `.rpm`).
- Each bundle ships:
  - Builder binary
  - Verifier binary
  - SpecAnchor (signed with the project signing key)
  - UI assets
- No backend service.

## Build Artifact
- `vaultcore-<version>-<platform>.<ext>` produced by `pnpm --dir app tauri build`.
- Reproducible: same source tree + same toolchain + same SpecAnchor must produce byte-identical bundles within tolerances allowed by platform signing (signature timestamps).
- All artifacts are signed:
  - Tauri updater signature (`TAURI_SIGNING_PRIVATE_KEY`)
  - Platform code signing (`MACOS_CODESIGN_IDENTITY`, `WINDOWS_CODESIGN_PFX`)
- SHA-256 manifest is published with each release.

## Release Flow
1. Complete the active ExecPlan.
2. Run `./scripts/verify.sh` locally.
3. Open a release-candidate branch.
4. CI runs the verification matrix on macOS, Windows, Linux.
5. CI produces signed bundles and the SHA-256 manifest.
6. Release manager runs the smoke test on each platform binary.
7. Tag the release; publish bundles + manifest + release notes.
8. Updater channel is updated only after manual approval.

## Deployment Steps
1. Confirm release-candidate criteria from `RELEASE.md`.
2. Confirm SpecAnchor for the release is signed with the project key and embedded in the bundle.
3. Confirm code-signing identities are available in CI for the platforms being released.
4. Run `./scripts/build.sh` per platform.
5. Run `./scripts/smoke-test.sh` on the built binary per platform.
6. Publish bundles + SHA-256 manifest.
7. Update the updater channel only after manual approval.

The owner-controlled release signing procedure and required evidence are documented in `.agent/runbooks/owner-release-signing.md`.

## Migration Steps
- Migrations for the user's vault file run at first launch after an update.
- Migrations are additive by default; destructive migrations require an ADR + rollback path in `ROLLBACK.md`.
- A pre-migration backup of the vault file is taken automatically and retained for 7 days (or configurable).

## Rollback Steps
See `ROLLBACK.md`. At minimum:
- Pause the updater channel.
- Revert published bundles to the previous version.
- Communicate via the release notes channel.
- Restore the pre-migration vault backup if a migration corrupted user data (with user consent).

## Post-Deploy Smoke Tests
- App launches and reaches the lock screen in under 1.5 s.
- Unlock succeeds with each supported auth path.
- Reveal of a fixture secret succeeds with auto-clear.
- Audit chain verifies from the UI.
- No remote network call is made (asserted by a local network sentinel test on a sandboxed VM).

## Required Approvals
- Production release (publishing to the updater channel): explicit owner approval.
- Any destructive migration: explicit owner approval + recorded rollback drill.
- Any change to crypto primitives or SpecAnchor schema: ADR + security review.

## Deployment STOP Conditions
- Required code-signing credentials are missing.
- Smoke test fails on any supported platform.
- Verification on the release-candidate branch is not green.
- SpecAnchor for the release is not signed with the project key.
- A destructive migration lacks a tested rollback.
- The updater channel publish would happen without explicit approval.

## Production Verification
- Bundles SHA-256 match the manifest.
- Code signature verifies on each platform.
- SpecAnchor signature verifies at first launch.
- Smoke test passes on each platform.
- No telemetry leaves the device (asserted by sandboxed VM check).
