# IPC Signature Failure Runbook

## Trigger
Vault Health reports repeated IPC signature failures, Verifier rejects Builder frames, or `trinity_ipc` tests fail.

## Immediate Actions
1. Lock the active session.
2. Capture local Builder and Verifier logs with the shared `session_id` and `err_code`.
3. Run `cargo nextest run --test trinity_ipc` and `cargo nextest run -p vaultcore-verifier obs`.
4. Confirm failures are not caused by replayed counters or a mismatched fixture key.

## Recovery
1. Restart Builder and Verifier to re-establish the local IPC channel.
2. If the failure persists, keep the vault locked and investigate signature, counter, and framing changes.
3. Do not weaken signature verification, replay protection, or the Trinity process boundary to restore service.

## Escalation
Treat repeated unexplained failures as Sev-1 because they can affect invariants I-4 and I-5.
