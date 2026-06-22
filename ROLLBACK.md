# ROLLBACK.md

## Rollback Triggers
- Sev-1 incident (suspected I-1/I-4/I-5/I-7 violation).
- Smoke test failure on production binary.
- Migration failure that affects user data integrity.
- Repeated audit-chain anomalies in the field.
- Code-signing or SpecAnchor signing key compromise.

## Rollback Decision Owner
- Owner (project lead) or designated release manager.
- Documented in `OPERATIONS.md`.

## Rollback Types
- Application rollback (revert published bundle).
- Updater channel pause.
- User vault file rollback (from pre-migration backup, with user consent).
- SpecAnchor rotation (offline; new release).
- Code-signing key rotation (offline; new release; revoke old certs per platform policy).

## Application Rollback
- Pause the updater channel.
- Revert published bundles to the prior signed release.
- Republish the SHA-256 manifest for the prior version.
- Communicate via release notes channel.

## Database / Vault Rollback
- If a migration corrupted user data:
  - Surface the pre-migration backup path to the user in-app.
  - Provide a guided restore action that:
    - Stops Builder/Verifier
    - Replaces the vault file with the backup copy
    - Restarts and verifies the audit chain head
  - Never auto-roll-back without explicit user consent (the user owns the data).

## Config Rollback
- Configuration is the SpecAnchor; rollback means shipping a new release with the prior, signed SpecAnchor.
- Pause the updater channel until the new bundle is signed and verified.

## Feature Flag Rollback
- Not used in v1. (Feature flags would expand attack surface; document N/A.)

## Verification After Rollback
- Smoke test passes on the rolled-back binary per platform.
- Audit chain head hash on restored vaults matches expected continuation point.
- No Sev-1/Sev-2 incidents in the observation window.

## Communication
- Document in release notes and the public incident log:
  - Incident summary
  - Impact window
  - Rollback action taken
  - Current status
  - Next steps

## Postmortem
After rollback:
- Root cause
- Timeline
- Which invariants were touched
- Which tests should have caught it
- Required test additions, ADRs, or threat-model updates
- Owner of follow-ups and due dates
- Stored in `.agent/runbooks/postmortems/`
