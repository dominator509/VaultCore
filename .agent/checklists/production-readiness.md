# Production Readiness Checklist (VaultCore)

Status: local evidence green; production launch NO-GO until owner approval, production signing credentials, release SpecAnchor signing, and signed release artifact evidence are present.

Last local evidence run: 2026-06-22.
Last multi-OS CI verify run: GitHub Actions run `27940112544` on commit `2adc3051d8a7a511ee28804a7ff5e0b54afd8abd` passed `Verify (windows-latest)`, `Verify (macos-latest)`, and `Verify (ubuntu-latest)`.

Gate Evidence
- [x] G1-A Foundation validation: `./scripts/verify.sh`
- [x] G1-B Core domain invariants: `cargo nextest run --test invariants`
- [x] G1-C Persistence and audit chain: `cargo nextest run --test invariants`
- [x] G1-D Crypto and Trinity boundary: `cargo nextest run --test invariants`
- [x] G1-E SpecAnchor verification fixtures: `./scripts/verify.sh`
- [x] G1-F Audit immutability: `cargo nextest run --test invariants`
- [x] G2-A Identity and local unlock paths: `./scripts/verify.sh`
- [x] G2-B Session controls: `./scripts/verify.sh`
- [x] G2-C Secret lifecycle hardening: `./scripts/verify.sh`
- [x] G3-A UI acceptance and accessibility: `./scripts/verify.sh`
- [x] G3-B Observability and health: `./scripts/smoke-test.sh`
- [x] G3-C Local release artifacts and manifest: `./scripts/build.sh`
- [x] G3-D Threat coverage: `cargo nextest run --test threats`
- [ ] G3-E Production launch approval: pending owner approval, signing credentials, release SpecAnchor signing, and signed release artifact evidence

Functional
- [x] All eight secret types supported
- [x] All five roles enforced
- [x] Lifecycle FSM transitions implemented and tested
- [x] All primary UI flows present

Testing
- [x] Local validation chain passes through `./scripts/production-readiness-check.sh`
- [x] Invariant suite (I-1..I-8) green
- [x] Threat coverage tests green or residual accepted

Security
- [x] No secrets/keys committed
- [x] Logs redacted (marker test green)
- [x] cargo deny + cargo audit + pnpm audit green or accepted
- [x] RBAC matrix complete
- [x] SpecAnchor verification not bypassed
- [ ] Production signing key available in approved release environment only

Privacy
- [x] No telemetry leaves device by default
- [x] Backup/restore documented
- [x] Purge implements cryptographic erasure

Performance
- [ ] Cold start < 1.5 s release-manager measurement pending
- [ ] Unlock (passkey) < 500 ms release-manager measurement pending
- [ ] Search (10k records) < 200 ms release-manager measurement pending
- [ ] Reveal < 100 ms after countersignature release-manager measurement pending

Accessibility
- [x] WCAG 2.1 AA primary flows
- [x] axe zero serious/critical
- [x] Keyboard-only nav

Observability
- [x] Health endpoints operational
- [x] Vault Health view present
- [x] Audit chain head visible

Deployment
- [ ] Signed Tauri bundles per platform pending production signing credentials and release artifact matrix
- [x] SHA-256 manifest generated locally
- [ ] SHA-256 manifest published pending owner-approved release
- [x] Multi-OS verify CI passed on GitHub Actions run `27940112544`

Rollback
- [x] Rollback steps documented and dry-run rehearsed
- [ ] Updater channel pause procedure ready pending owner-approved updater channel

Data
- [x] Schema documented
- [x] Migrations additive (or destructive with ADR + rollback)
- [x] Audit chain continuity preserved across migrations

Docs / Support
- [x] All specs current
- [ ] TRACEABILITY.md local rows advanced; production launch approval remains NO-GO
- [x] Residual risks R-1..R-5 surfaced where applicable
- [x] Incident response checklist exists
- [x] Escalation path defined
