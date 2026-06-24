# Owner Release Signing Runbook

## Purpose
This runbook is the owner-controlled procedure for clearing the EP-010 production launch gate. It does not grant release approval by itself; it records the exact evidence the owner or release manager must produce before `G3-E` can be marked complete.

## Preconditions
- `main` is clean, pushed, and green in the multi-OS CI verification matrix.
- The owner has approved entry into the release-candidate phase.
- Required release credentials are available only through the approved secret manager or GitHub Actions secret store.
- No production signing key, platform certificate, PFX password, or SpecAnchor private key is committed to this repository or pasted into issue comments, chat, logs, or local plaintext files.

## Required Owner Evidence
- Owner approval text for release-candidate entry.
- Owner approval text for updater-channel publication, if publication is in scope.
- GitHub Actions release-candidate workflow URL.
- Release artifact inventory for Windows, macOS, and Linux.
- `SHA256SUMS.txt` for every produced artifact set.
- Platform code-signature verification evidence for each released platform.
- Release SpecAnchor signing evidence from the owner-controlled offline signing process.
- Smoke-test evidence against each platform binary.

## Repository Reality Checks
- `.github/workflows/release.yml` requires these GitHub Actions secrets: `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`, `MACOS_CODESIGN_IDENTITY`, and `WINDOWS_CODESIGN_PFX`.
- `crates/cli/src/specanchor.rs` currently generates a development SpecAnchor with fixed test keys. Do not use `cargo run -p vaultcore-cli -- specanchor generate` for a production SpecAnchor.
- `app/package.json` does not currently include a Tauri updater plugin dependency. Treat updater-channel publication as pending until updater configuration and publication mechanics are explicitly verified.
- `scripts/build.sh` invokes `pnpm tauri build` and gathers artifacts plus `SHA256SUMS.txt`, but it does not import platform signing certificates by itself. Confirm the release workflow actually signs artifacts before treating code-signing as complete.

## PowerShell Procedure
Run from a trusted owner or release-manager machine. Use PowerShell Admin only when platform certificate tooling requires it.

```powershell
cd C:\dev\VaultCore

git checkout main
git pull --ff-only origin main
git status -sb

gh auth status
gh repo view dominator509/VaultCore --json nameWithOwner,url,defaultBranchRef,isPrivate
```

Generate or retrieve the Tauri updater signing key through the approved owner-controlled path. If generating a new Tauri updater key with the local Tauri CLI:

```powershell
$ReleaseDir = "$env:USERPROFILE\.vaultcore-release"
New-Item -ItemType Directory -Force $ReleaseDir

pnpm --dir app tauri signer generate --write-keys "$ReleaseDir\vaultcore-tauri.key"
```

Store release secrets in GitHub Actions. Prefer interactive prompts or stdin; do not echo secret values into the terminal history.

```powershell
Get-Content -Raw "$ReleaseDir\vaultcore-tauri.key" |
  gh secret set TAURI_SIGNING_PRIVATE_KEY -R dominator509/VaultCore

gh secret set TAURI_SIGNING_PRIVATE_KEY_PASSWORD -R dominator509/VaultCore
gh secret set MACOS_CODESIGN_IDENTITY -R dominator509/VaultCore
```

For Windows PFX material, use base64 content from the approved certificate file:

```powershell
[Convert]::ToBase64String([IO.File]::ReadAllBytes("C:\path\to\windows-codesign.pfx")) |
  gh secret set WINDOWS_CODESIGN_PFX -R dominator509/VaultCore
```

Create or fast-forward the `release-candidate` branch:

```powershell
git checkout main
git pull --ff-only origin main

git branch --list release-candidate
```

If the branch does not exist:

```powershell
git switch -c release-candidate
git push -u origin release-candidate
```

If it already exists:

```powershell
git switch release-candidate
git merge --ff-only main
git push origin release-candidate
```

Watch the release-candidate workflow:

```powershell
gh run list --repo dominator509/VaultCore --workflow release.yml --branch release-candidate --limit 1
gh run watch --repo dominator509/VaultCore --exit-status
```

Download release artifacts after the workflow passes:

```powershell
$RunId = gh run list --repo dominator509/VaultCore --workflow release.yml --branch release-candidate --limit 1 --json databaseId --jq '.[0].databaseId'
gh run download $RunId --repo dominator509/VaultCore --dir C:\tmp\VaultCore-release-artifacts
Get-ChildItem C:\tmp\VaultCore-release-artifacts -Recurse
```

Verify hashes locally:

```powershell
Get-ChildItem C:\tmp\VaultCore-release-artifacts -Recurse -Filter SHA256SUMS.txt | Get-Content
Get-ChildItem C:\tmp\VaultCore-release-artifacts -Recurse -File | Get-FileHash -Algorithm SHA256
```

Verify platform signatures with the platform-specific release tooling before publication. Do not mark code-signing complete based only on secret presence.

## Production SpecAnchor
Production SpecAnchor signing is offline and owner-controlled. The current repository CLI generation path is development-only because it uses fixed test keys.

After the owner-controlled signing process produces the release SpecAnchor, verify the envelope:

```powershell
cargo run -p vaultcore-cli -- specanchor verify --in C:\path\to\production\specanchor.signed
```

Record the signed SpecAnchor evidence without committing the private key or any key material.

## Completion Criteria
EP-010 `G3-E` can be advanced only when all of these are true:
- Owner approval is recorded in `DECISIONS.md`.
- Production signing credentials were used only through the approved release environment.
- Release SpecAnchor signing evidence is recorded.
- Signed release artifacts and `SHA256SUMS.txt` are available.
- Platform code signatures are verified.
- Smoke tests pass against the released binaries.
- Updater-channel publication is explicitly approved by the owner, if publication occurs.

