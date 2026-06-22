# Rollback Drill - EP-009

## Scope
Dry-run rollback procedure for the `0.1.0-rc.1` release-candidate flow. No updater channel was modified and no user vault data was touched.

## Preconditions
- Release candidate branch has completed `./scripts/verify.sh`.
- Release artifacts and `target/release-artifacts/SHA256SUMS.txt` exist for each supported platform build.
- Owner approval is required before any updater-channel publish.

## Dry-Run Steps
1. Identify the prior signed release bundle and manifest.
2. Pause updater-channel promotion before publishing any replacement metadata.
3. Replace candidate artifacts with the prior signed bundle set in the release staging area.
4. Republish the prior SHA-256 manifest.
5. Run `./scripts/smoke-test.sh` against the rolled-back binary on each platform.
6. Record incident summary, impact window, current status, and follow-up owner.

## Result
Dry-run documented only. Updater channel pause remains a manual owner-approved action because no production updater channel exists in this checkout.

## Evidence
- `ROLLBACK.md`
- `DEPLOYMENT.md`
- `RELEASE.md`
- `./scripts/smoke-test.sh`
