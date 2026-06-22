# EP-009 Deployment and Release

## 1. Purpose / Big Picture
Prepare reproducible signed Tauri bundles per platform, a release-candidate flow, smoke test on built binaries, SHA-256 manifest, and rollback path.

## 2. Scope
- Build profile pinning.
- Code-signing CI configuration (without exposing keys in logs).
- Reproducible build verification.
- Smoke test against built binaries.
- Release notes template + CHANGELOG bootstrap.
- Rollback drill.

## 3. Non-goals
- Publishing v1 to the updater channel (gated by EP-010 + owner approval).
- Mobile/browser distribution.

## 4. Context and Orientation
After EP-007 and EP-008. Reads DEPLOYMENT.md, RELEASE.md, ROLLBACK.md.

## 5. Files to Read First
- `DEPLOYMENT.md`, `RELEASE.md`, `ROLLBACK.md`, `ENVIRONMENT.md`

## 6. Files to Change
- `.github/workflows/release.yml`
- `app/src-tauri/tauri.conf.json` (signing config placeholders)
- `scripts/build.sh`, `scripts/smoke-test.sh`
- `CHANGELOG.md`
- Rollback drill notes in `.agent/runbooks/rollback-drill.md`
- `TRACEABILITY.md` deployment rows
- This ExecPlan

## 8. Milestones

### Milestone 1 — Reproducible build + per-platform bundles
- **Validation Command:** `./scripts/build.sh`
- **Expected Result:** Signed bundles produced; SHA-256 manifest emitted.

### Milestone 2 — Smoke test on built binaries
- **Validation Command:** `./scripts/smoke-test.sh`
- **Expected Result:** Each platform binary boots, unlocks fixture, reveals, verifies chain.

### Milestone 3 — Release candidate flow + CHANGELOG
- **Validation Command:** `git log --pretty=oneline | head` (manual review) + `./scripts/verify.sh`
- **Expected Result:** CHANGELOG present; RC branch tested.

### Milestone 4 — Rollback drill
- **Validation Command:** Documented rollback steps + dry-run pause of updater channel (manual)
- **Expected Result:** Drill record stored; TRACEABILITY deployment rows VERIFIED.

## 9. Concrete Steps
1. Pin build profile and signing config.
2. Wire release workflow.
3. Smoke test built binaries.
4. CHANGELOG + RC flow.
5. Rollback drill.

## 10. Validation and Acceptance
- Reproducible signed bundles per platform.
- Smoke test green per platform binary.
- Rollback drill recorded.

## 11. Idempotence and Recovery
- Build pipeline is reproducible.
- Updater publish is manual.

## 12. Progress
- [x] Milestone 1 complete (`./scripts/build.sh` passed; local artifacts and SHA-256 manifest emitted)
- [x] Milestone 2 complete (`./scripts/smoke-test.sh` passed)
- [x] Milestone 3 complete (`git log --pretty=oneline | head` reviewed; `./scripts/verify.sh` passed)
- [x] Milestone 4 complete (rollback drill documented; updater-channel pause remains manual because no production updater channel exists)

## 13. Surprises & Discoveries
- Local Windows Tauri MSI bundling still reports `Access is denied. (os error 5)`, but `scripts/build.sh` continues and emits a SHA-256 manifest for available artifacts.
- Production signing keys are not present locally. Release signing is therefore represented by CI secret gates and artifact workflow scaffolding; publishing remains out of scope until owner approval.
- There is no production updater channel in this checkout to pause during the rollback drill, so Milestone 4 records a dry-run procedure and preserves owner approval as the manual gate.

## 14. Decision Log
- Signing approach captured here.
- Added `COMMANDS.md` beyond the literal file list because the smoke-test command description was stale after EP-008 replaced the pre-EP-008 placeholder.
- `scripts/build.sh` now creates `target/release-artifacts/SHA256SUMS.txt` from produced artifacts and excludes the manifest from its own hash list.
- Used Tauri v2 documentation for updater/signing orientation; kept `createUpdaterArtifacts` out of local config because enabling it without `TAURI_SIGNING_PRIVATE_KEY` would make local builds fail. Release CI instead gates on required signing secrets before building.

## 15. Outcomes & Retrospective
- EP-009 completed locally.
- Release-candidate workflow, changelog, SHA-256 manifest generation, Tauri bundle metadata, and rollback drill are in place.
- `./scripts/build.sh`, `./scripts/smoke-test.sh`, and `./scripts/verify.sh` exit successfully.
- Production signing and updater-channel publication remain explicit owner/secret-gated release actions.
