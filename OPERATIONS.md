# OPERATIONS.md

## Local Operations
- Use `./scripts/preflight.sh` before each session.
- Use `./scripts/verify.sh` before submitting changes.
- Keep dev vaults under `VAULTCORE_DEV_DIR`; never point them at a real user vault.

## Staging Operations
- "Staging" is the release-candidate bundle. The release manager runs it through the smoke test on each supported platform before promoting.

## Production Operations
- VaultCore runs on the user's device. There is no operator console.
- The release manager owns the updater channel and follows `DEPLOYMENT.md`, `RELEASE.md`, `ROLLBACK.md`.

## Health Checks
- Builder exposes a local introspection endpoint (`/health/builder`) over a Unix domain socket / named pipe (not network) reporting:
  - SpecAnchor verified
  - IPC channel established
  - Last audit append succeeded
- Verifier exposes a local introspection endpoint (`/health/verifier`) reporting:
  - SpecAnchor verified
  - Audit log tail hash
  - Session count
- Smoke test asserts both health endpoints.

## Common Failure Modes
- SpecAnchor verification failure → app refuses to start; surfaces a clear, recovery-focused error.
- Audit log tamper detected → app enters read-only "tamper-evident" mode; refuses writes; surfaces the audit chain anomaly to the user.
- Builder ↔ Verifier IPC drop → app re-establishes the channel up to N retries with backoff, then surfaces the failure to the user.
- Migration mid-launch failure → app aborts the migration; restores the pre-migration backup; surfaces a clear failure with a recovery action.

## Troubleshooting
1. Capture the exact failure (UI message, log line, error code).
2. Check Builder/Verifier health endpoints.
3. Check the audit chain head hash matches the expected continuation.
4. Inspect logs in `VAULTCORE_DEV_DIR/logs/` (dev) or platform log location (production).
5. Run `./scripts/smoke-test.sh` if reproducible locally.
6. If the issue affects data integrity, evaluate rollback per `ROLLBACK.md`.

## Database Backup / Restore
- Vault file: copy the encrypted SQLite file out-of-band. Restore by placing it back into the configured vault directory.
- Audit log: included in the vault file (single source of truth).
- Backups are encrypted-at-rest by design (the vault file itself).
- Pre-migration backups are taken automatically and retained for 7 days (configurable).

## Scheduled Jobs / Background Work
- Auto-clear timer for revealed payloads and clipboard.
- Idle timeout / auto-lock.
- Optional expiry scan that transitions secrets from `active` to `expiring_soon` / `expired` (lifecycle FSM).

## Incident Triage
1. Detect (user report, smoke-test failure on release, or local anomaly).
2. Determine severity:
   - Sev-1: any sign of plaintext-at-rest, audit chain break, or unauthorized write.
   - Sev-2: unlock failures or update channel breakage.
   - Sev-3: UI or accessibility regressions.
3. Mitigate (pause updater channel for Sev-1/2).
4. Communicate (release notes channel).
5. Resolve with a regression test and an ADR if architectural.
6. Postmortem in `.agent/runbooks/`.

## Escalation Rules
- Any Sev-1 (suspected I-1/I-4/I-5/I-7 violation) → immediate owner notification, updater channel paused.
- Any Sev-2 → notify within one business day.
- Any Sev-3 → handled in the next ExecPlan cycle.

## Maintenance Windows
- Not applicable in the traditional sense. Updates are user-initiated through the signed updater channel.

## Operational Safety Rules
- Never modify a user's vault from CI.
- Never disable invariant enforcement tests to ship a release.
- Never push to the updater channel without explicit approval.
- Always preserve audit-chain continuity across migrations.
