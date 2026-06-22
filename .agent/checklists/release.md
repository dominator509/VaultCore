# Release Checklist (VaultCore)

- [ ] Version selected (semver; v1.x line)
- [ ] CHANGELOG.md entry prepared
- [ ] Release candidate criteria met
- [ ] `./scripts/verify.sh` green on macOS/Windows/Linux CI matrix
- [ ] Invariant suite green
- [ ] Threat coverage map updated
- [ ] Signed Tauri bundles + SHA-256 manifest produced
- [ ] Code signing verified per platform
- [ ] SpecAnchor for release signed with project key
- [ ] Smoke test passes against built binary per platform
- [ ] Owner approval to publish to updater channel
- [ ] Release notes published (with residual risks if applicable)
- [ ] Post-release monitoring window planned
