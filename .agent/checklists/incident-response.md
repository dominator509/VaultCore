# Incident Response Checklist (VaultCore)

- [ ] Detect (user report, smoke failure, anomaly)
- [ ] Triage severity:
  - Sev-1: suspected I-1/I-4/I-5/I-7 violation, audit chain break, unauthorized write
  - Sev-2: unlock failures, updater channel breakage
  - Sev-3: UI or a11y regressions
- [ ] Mitigate (pause updater for Sev-1/2)
- [ ] Communicate (release notes channel)
- [ ] Resolve (regression test + ADR if architectural)
- [ ] Verify (smoke + invariant + threat suites green)
- [ ] Document (postmortem)
- [ ] Follow up (TRACEABILITY updates, new tests, threat-model rows)
