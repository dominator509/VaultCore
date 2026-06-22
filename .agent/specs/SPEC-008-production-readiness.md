# SPEC-008 Production Readiness

- **Status:** Draft
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phase 9
- **Linked ExecPlans:** EP-010

## User-Visible Goal
A reproducible, objective launch gate that lets the owner decide whether VaultCore v1 is safe to publish.

## Non-Goals
- Subjective approvals.
- Release without invariant enforcement tests.

## Terms
- **Gate:** an objective condition verifiable from the repository.
- **Dry run:** a release rehearsal without publishing to the updater channel.

## Required Behavior

### Required Evidence Before Release
- `./scripts/verify.sh` green on macOS, Windows, Linux CI matrix
- `cargo nextest run --test invariants` green
- `./scripts/security-check.sh` green
- `./scripts/dependency-audit.sh` green (or accepted advisories documented in `deny.toml`)
- `./scripts/smoke-test.sh` green on each built platform binary
- Signed bundles + SHA-256 manifest produced
- SpecAnchor for release signed with project key
- TRACEABILITY.md rows all `VERIFIED + GATE PASSED`
- Threat coverage map updated: every threat T-001..T-023 has a linked mitigation or accepted residual risk
- Residual risks R-1..R-5 surfaced in release notes and in-app where applicable
- Rollback drill recorded within the prior 14 days

### Required Approvals
- Owner approval to enter release candidate
- Owner approval to publish to the updater channel
- Security review for any major release

### Gates
- G1-A Foundation green
- G1-B Core domain frozen
- G1-C Persistence + audit chain
- G1-D Crypto + Trinity
- G1-E SpecAnchor
- G1-F Audit immutability
- G2-A Identity
- G2-B Session
- G2-C Secret lifecycle hardening
- G3-A UI acceptance
- G3-B Observability
- G3-C Release
- G3-D Threat model verification
- G3-E Launch

Every gate has a linked checklist item in `.agent/checklists/production-readiness.md` and a script step in `./scripts/production-readiness-check.sh`.

## Inputs / Outputs
- Inputs: verification results, security review, performance/accessibility/privacy checks, deployment readiness evidence.
- Outputs: a go/no-go decision packet with linked evidence and residual risk list.

## Error States
- Missing evidence on any gate.
- Open Sev-1 or Sev-2 issue.
- Missing signing credentials.

## Data Rules
- No production user data is touched during readiness checks.

## Security Rules
- Secrets and signing keys never appear in CI logs.
- Telemetry remains off by default.

## Performance Rules
- Performance budgets met (SPEC-000, SPEC-004).

## Observability Rules
- Health endpoints and Vault Health view present and accurate.

## Required Tests
- All verification scripts.
- All invariant enforcement tests.
- Smoke test per platform binary.

## Acceptance Criteria
- `./scripts/production-readiness-check.sh` exits 0.
- All gates passed with linked evidence.
- Owner approval recorded.
