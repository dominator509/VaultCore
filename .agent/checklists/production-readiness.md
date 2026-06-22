# Production Readiness Checklist (VaultCore)

Functional
- [ ] All eight secret types supported
- [ ] All five roles enforced
- [ ] Lifecycle FSM transitions implemented and tested
- [ ] All primary UI flows present

Testing
- [ ] All `./scripts/*` validation commands pass
- [ ] Invariant suite (I-1..I-8) green
- [ ] Threat coverage tests green or residual accepted

Security
- [ ] No secrets/keys committed
- [ ] Logs redacted (marker test green)
- [ ] cargo deny + cargo audit + pnpm audit green or accepted
- [ ] RBAC matrix complete
- [ ] SpecAnchor verification not bypassed

Privacy
- [ ] No telemetry leaves device by default
- [ ] Backup/restore documented
- [ ] Purge implements cryptographic erasure

Performance
- [ ] Cold start < 1.5 s
- [ ] Unlock (passkey) < 500 ms
- [ ] Search (10k records) < 200 ms
- [ ] Reveal < 100 ms after countersignature

Accessibility
- [ ] WCAG 2.1 AA primary flows
- [ ] axe zero serious/critical
- [ ] Keyboard-only nav

Observability
- [ ] Health endpoints operational
- [ ] Vault Health view present
- [ ] Audit chain head visible

Deployment
- [ ] Signed Tauri bundles per platform
- [ ] SHA-256 manifest published
- [ ] Reproducible build verified

Rollback
- [ ] Rollback steps documented and rehearsed
- [ ] Updater channel pause procedure ready

Data
- [ ] Schema documented
- [ ] Migrations additive (or destructive with ADR + rollback)
- [ ] Audit chain continuity preserved across migrations

Docs / Support
- [ ] All specs current
- [ ] TRACEABILITY.md fully green
- [ ] Residual risks R-1..R-5 surfaced where applicable
- [ ] Incident response checklist exists
- [ ] Escalation path defined
