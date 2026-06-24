# EP-010 Production Readiness

## 1. Purpose / Big Picture
Final gate. Verify every invariant, every threat, every spec, every gate. Produce the go/no-go evidence packet.

## 2. Scope
- Full verification on all three OSes.
- Invariant suite re-run.
- Threat coverage map re-verified.
- Security review notes.
- Performance and accessibility re-checks.
- Privacy review.
- Backup/restore verification.
- Monitoring/health re-verified.
- Deployment dry run + rollback drill review.
- Documentation review.
- TRACEABILITY.md fully green.

## 3. Non-goals
- Net-new features (only launch-blocking fixes).
- Publishing without owner approval.

## 4. Context and Orientation
After EP-009. Reads PRODUCTION_READINESS.md, SPEC-008.

## 5. Files to Read First
- `PRODUCTION_READINESS.md`, all specs, runbooks, CHANGELOG.md

## 6. Files to Change
- Readiness docs/checklists.
- Narrow launch-blocking fixes only.
- `PRODUCTION_READINESS.md` final status.
- `TRACEABILITY.md` (all rows VERIFIED + GATE PASSED).
- This ExecPlan.

## 8. Milestones

### Milestone 1 — Full verification + invariant suite
- **Validation Command:** `./scripts/verify.sh && cargo nextest run --test invariants && cargo nextest run --test threats`
- **Expected Result:** Green on each OS.

### Milestone 2 — Readiness domains review (security, privacy, performance, a11y, observability, deployment, rollback, data, docs)
- **Validation Command:** `./scripts/production-readiness-check.sh`
- **Expected Result:** Exit 0 or exact remaining gaps recorded.

### Milestone 3 — Final launch packet + owner approval
- **Validation Command:** `git diff --name-only`
- **Expected Result:** Only readiness-related files changed; owner approval recorded in DECISIONS.md.

## 9. Concrete Steps
1. Run verification matrix.
2. Close any launch-blocking gap (narrow fixes only).
3. Run readiness script.
4. Produce evidence packet.

## 10. Validation and Acceptance
- All checks pass.
- All gates G1-A..G3-E passed with linked evidence.
- All invariants I-1..I-8 have passing enforcement tests.
- All threats T-001..T-023 mitigated or accepted as residual.
- Owner approval recorded.

## 11. Idempotence and Recovery
- Readiness review can be rerun.
- No destructive operations.

## 12. Progress
- [x] Milestone 1 complete (`./scripts/verify.sh && cargo nextest run --test invariants && cargo nextest run --test threats` passed)
- [x] Milestone 2 complete (`./scripts/production-readiness-check.sh` passed)
- [ ] Milestone 3 blocked on owner approval, production signing credentials, release SpecAnchor signing, and signed release artifact evidence

## 13. Surprises & Discoveries
- Local checks are green, and multi-OS verify CI is green, but owner approval, production signing credentials, release SpecAnchor signing, and signed release artifact evidence are not present in the local checkout.
- `.agent/checklists/production-readiness.md` already existed, so EP-010 uses that checklist as the evidence packet instead of introducing a parallel readiness artifact.
- After pushing the readiness packet, GitHub CI exposed two setup gaps: shell scripts were committed without executable bits, and Playwright's Chromium browser was not installed by `./scripts/install.sh`.
- GitHub Actions run `27940112544` passed `Verify (windows-latest)`, `Verify (macos-latest)`, and `Verify (ubuntu-latest)` for commit `2adc3051d8a7a511ee28804a7ff5e0b54afd8abd`; this proves the multi-OS verify gate, not production signing or signed artifact publication.

## 14. Decision Log
- Final owner approval is not recorded. Per EP-010 non-goals and AGENTS.md STOP conditions, no production publish, updater-channel activation, or release SpecAnchor signing was performed.
- `DECISIONS.md` was updated to record the EP-010 NO-GO launch decision without fabricating owner approval.
- `TRACEABILITY.md` now distinguishes local gate-passed evidence from the production launch gate, which remains pending owner approval and release-signing evidence.
- `scripts/install.sh` now installs Playwright Chromium because E2E and accessibility tests are part of the canonical verification chain and CI runners do not retain browser binaries.
- Shell scripts under `scripts/` were marked executable in git so documented `./scripts/*.sh` commands work on Linux and macOS runners.
- Multi-OS verify CI evidence is accepted for EP-010 local readiness evidence, while signed release artifact evidence remains pending behind owner approval and production signing credentials.
- Added `.agent/runbooks/owner-release-signing.md` to make the remaining owner-controlled signing procedure explicit without recording false approval or committing signing material.
- Added a 60-minute timeout to `.github/workflows/ci.yml` because GitHub Actions run `28080516394` stayed in the Windows and Ubuntu `Verify` steps without logs; this is a launch-readiness CI guard, not a scope expansion.

## 15. Outcomes & Retrospective
- Local production-readiness evidence is green through Milestones 1 and 2.
- Launch packet is prepared in `PRODUCTION_READINESS.md`, `.agent/checklists/production-readiness.md`, `DECISIONS.md`, and `TRACEABILITY.md`.
- Production launch remains NO-GO until the owner approves release-candidate entry, follows `.agent/runbooks/owner-release-signing.md`, provides the approved production signing path, completes release SpecAnchor signing, and produces signed release artifact evidence.
