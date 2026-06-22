# Rollback Checklist (VaultCore)

- [ ] Rollback trigger identified (Sev-1/Sev-2 incident, smoke failure, migration corruption, key compromise)
- [ ] Rollback decision owner notified
- [ ] Rollback method selected (app rollback / updater pause / vault restore / SpecAnchor rotation / signing key rotation)
- [ ] Updater channel paused
- [ ] Bundles reverted to prior signed release
- [ ] SHA-256 manifest republished for prior version
- [ ] Vault restore from pre-migration backup (with user consent) if applicable
- [ ] Smoke test passes on rolled-back binary per platform
- [ ] Audit chain continuity verified post-rollback
- [ ] Communication: incident summary + impact window + status + next steps
- [ ] Postmortem stored under `.agent/runbooks/postmortems/`
