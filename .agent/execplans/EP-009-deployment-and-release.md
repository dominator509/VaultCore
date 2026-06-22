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
- [ ] Milestone 1 complete
- [ ] Milestone 2 complete
- [ ] Milestone 3 complete
- [ ] Milestone 4 complete

## 13. Surprises & Discoveries
- None yet.

## 14. Decision Log
- Signing approach captured here.

## 15. Outcomes & Retrospective
- Pending execution.
