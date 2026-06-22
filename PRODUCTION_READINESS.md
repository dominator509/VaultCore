# PRODUCTION_READINESS.md

## Final Readiness Status
Status: local evidence green; production launch NO-GO.

Last local evidence run: 2026-06-22.

Local commands completed:
- `./scripts/preflight.sh`
- `./scripts/verify.sh && cargo nextest run --test invariants && cargo nextest run --test threats`
- `./scripts/production-readiness-check.sh`
- `./scripts/build.sh`
- `./scripts/smoke-test.sh`

External launch blockers:
- Owner approval to enter release-candidate launch and publish any updater channel is not recorded.
- Production signing credentials are not present in the local checkout, and must not be committed.
- Live GitHub Actions multi-OS release matrix evidence has not been collected after pushing the release-readiness commits.
- Release SpecAnchor signing with the project key remains an owner-controlled offline action.

Recommended default: do not publish. Push the local evidence commits, let CI run, collect owner approval and signing material through the approved release process, then re-run this gate.

## Definition of Production Readiness
VaultCore v1 is production-ready only when functional, test, security, privacy, performance, accessibility, observability, deployment, rollback, data, documentation, and support gates are all satisfied; every invariant I-1..I-8 has at least one passing enforcement test; and every threat T-001..T-023 is either mitigated with linked evidence or accepted as a documented residual risk.

## Functional Readiness
- Core user outcomes from `PROJECT_BRIEF.md` and `SPEC-000-product-scope.md` all work.
- Lifecycle FSM transitions implemented and tested.
- Five roles enforced for every operation.
- Eight secret types supported.

## Test Readiness
- `./scripts/lint.sh` passes
- `./scripts/format-check.sh` passes
- `./scripts/typecheck.sh` passes
- `./scripts/test-unit.sh` passes
- `./scripts/test-integration.sh` passes
- `./scripts/test-e2e.sh` passes
- `./scripts/build.sh` passes (per platform)
- `./scripts/security-check.sh` passes
- `./scripts/dependency-audit.sh` passes
- `./scripts/smoke-test.sh` passes
- `./scripts/verify.sh` passes
- Invariant enforcement suite `cargo nextest run --test invariants` passes

## Security Readiness
- No secrets, signing keys, or real SpecAnchors committed
- Logs redacted; redaction test passes
- `cargo deny` and `cargo audit` are green or have explicit accepted advisories
- Input validation present at every boundary
- RBAC enforced for every operation; allow/deny test matrix complete
- All 23 threats addressed with linked mitigations
- All five residual risks (R-1..R-5) documented for the user where applicable

## Privacy Readiness
- Payloads encrypted at rest; metadata constrained to SPEC-002
- Export/deletion behavior implemented (cryptographic erasure on purge)
- No telemetry leaves the device by default
- Backup mechanism documented

## Performance Readiness
- Cold start, unlock, reveal, search performance budgets met
- Performance assertions in CI catch > 25 % regression

## Accessibility Readiness
- WCAG 2.1 AA baseline
- Playwright + axe shows zero serious/critical violations on primary flows
- Keyboard-only navigation verified for every primary flow

## Observability Readiness
- Structured logs with redaction
- Local metrics and health endpoints
- Audit chain head hash surfaced in UI

## Deployment Readiness
- Signed Tauri bundles per platform
- Reproducible build verified
- SHA-256 manifest published
- Updater channel ready

## Rollback Readiness
- Rollback steps documented and rehearsed
- Pre-migration vault backups taken automatically
- Updater channel can be paused

## Data Readiness
- Schema documented in SPEC-002
- Migrations additive; rollback for destructive migrations documented
- Backup/restore procedure documented

## Documentation Readiness
- All `.agent/specs/`, ExecPlans, runbooks, ARCHITECTURE.md, SECURITY.md, COMMANDS.md, DEPLOYMENT.md, ROLLBACK.md, OBSERVABILITY.md current
- TRACEABILITY.md rows all reach VERIFIED + GATE PASSED
- Decision Log and ASSUMPTIONS.md current

## Support Readiness
- Incident response checklist exists
- Escalation path defined (release manager / owner)
- Known risks documented (residual risks R-1..R-5)

## Final Launch Gate
Launch only when:
- `./scripts/production-readiness-check.sh` exits 0
- EP-000 through EP-010 complete with Outcomes & Retrospective updated
- All gates G1-A..G3-E passed with linked evidence
- All invariants I-1..I-8 verified
- Owner approval recorded
- Production signing credentials are available only through the approved release environment
- Multi-OS release CI has produced signed artifacts and `SHA256SUMS.txt`

## Checklist
- Functional readiness locally verified
- Test readiness locally verified
- Security readiness locally verified
- Privacy readiness locally verified
- Performance readiness locally evidenced by existing scripted gates; dedicated production benchmarking remains a release-manager review item
- Accessibility readiness locally verified by the existing axe gate
- Observability readiness locally verified
- Deployment readiness locally verified for unsigned local artifacts and SHA-256 manifest
- Rollback readiness locally documented and dry-run reviewed
- Data readiness locally verified
- Documentation readiness locally updated
- Support readiness locally documented
- Production launch approval pending
