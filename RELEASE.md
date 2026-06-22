# RELEASE.md

## Release Types
- Patch: bug fixes, low-risk improvements, no schema change, no IPC schema change.
- Minor: backward-compatible features, additive migrations only.
- Major: breaking schema/IPC changes, new threat-model rows, new architectural invariants. Requires ADR + security review.

## Versioning
- Semantic versioning. v1.x for the first production release line.
- Tauri updater version matches the semver tag.

## Changelog
- Maintain `CHANGELOG.md` (created in EP-009).
- Entry per release with: user-visible changes, security changes, schema/IPC changes, migration notes, residual risks if any newly surfaced.

## Branch Strategy
- `main` is releasable state.
- Feature work on short-lived branches.
- `release-candidate` branch is the cut-over point for release verification.

## Release Candidate Criteria
- Active ExecPlan complete
- `./scripts/verify.sh` green on macOS, Windows, Linux CI matrix
- Invariant enforcement suite green
- No open Sev-1 or Sev-2 issues
- Rollback path confirmed for any migration in the release
- Owner approval to enter release candidate

## Release Checklist
- Version selected
- Changelog entry prepared
- Verification matrix green per platform
- Smoke test green per platform binary
- Signed bundles + SHA-256 manifest produced
- Code signing verified per platform
- SpecAnchor for the release signed with the project key
- Updater channel publish approved by owner
- Release notes published

## Smoke Tests
Run `./scripts/smoke-test.sh` against the built binary on each platform after build, before publish.

## Approvals
- Owner approval required to publish to the updater channel.
- Security review required for any major release.
- Architecture review required for any change touching invariants or the SpecAnchor schema.

## Release Notes
Include:
- What changed (user-visible)
- Security-relevant changes
- Schema or IPC schema changes (and migration impact)
- Known risks
- Rollback notes

## Post-Release Monitoring
- Monitor local smoke-test reports from the release manager.
- Watch the issue tracker for Sev-1/Sev-2 reports.
- Keep the updater channel rollback procedure ready until at least one observation window after publish.
