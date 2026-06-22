# SPEC-005 Authentication and Permissions

- **Status:** Draft
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phase 5
- **Linked ExecPlans:** EP-006

## User-Visible Goal
Provide phishing-resistant local authentication (passkey preferred), RBAC over five roles, finite sessions with idle timeout, exponential-backoff lockout, and explicit audit of every authz decision.

## Non-Goals
- SSO, OAuth-based remote IdP (v1)
- Federated identity (v1)
- Account recovery via remote service (v1; forbidden by I-7)

## Terms
- **Session:** an authenticated state issued by the Verifier with a finite lifetime.
- **Operation:** a named action subject to RBAC.

## Required Behavior

### Authentication Paths
1. WebAuthn passkey (preferred): platform authenticator → Builder runs ceremony → Verifier validates proof → Verifier issues signed `SessionToken`.
2. Biometrics: platform biometric tied to a hardware-backed key.
3. Master passphrase fallback: passphrase → Argon2id KDF (params tuned at install per-device, minimum `m=64 MiB, t=3, p=1`) → derived key unwraps a long-term key handle.

### Lockout / Rate Limiting
- Exponential backoff per device after consecutive auth failures.
- Verifier enforces the lockout; Builder cannot lower it.

### Roles (exhaustive)
- **Owner:** all operations, including `purge`, `rotate-master-key`, `migrate`.
- **Admin:** all except `rotate-master-key`.
- **Editor:** read, create, update, rotate. No `purge`, no `migrate`.
- **Viewer:** read-only over metadata and payloads via explicit reveal.
- **Auditor:** read-only over metadata and audit log. No payload reveal.

### Authorization Matrix
- Every operation declares minimum role.
- Verifier evaluates: `role >= min_role` for hierarchical roles; `Auditor` is parallel and matched by explicit operation set.
- Default-deny on unknown ops.
- Denial paths emit audit entries with `result = denied`.

### Session Behavior
- TTL: configured in SpecAnchor (default 15 min idle, 8 h absolute).
- Idle timeout enforced by Verifier with periodic ping from Builder.
- `lock()` invalidates the session immediately.

### Threat Mitigations
- T-007 (phishing-resistant unlock): passkey preferred path; biometrics secondary; passphrase only as fallback.
- T-014..T-016 (IPC manipulation/replay): signed framed messages; monotonic counter; rejected attempts logged.

## Inputs / Outputs
- `unlock(method, proof)` → `SessionToken`
- `lock()` → `Ack`
- Authz decisions returned by Verifier as `Countersignature` or `Denied { reason }`

## Error States
- `AuthFailed`, `LockedOut`, `Unauthorized`, `SessionExpired`, `Replay`, `BadSignature`.

## Data Rules
- Long-term key handles stored in OS keychain or hardware-backed key store.
- Argon2id parameters stored in SpecAnchor; never user-configurable below documented minimums.

## Security Rules
- Default-deny.
- Verifier is the policy authority; Builder cannot bypass.
- Auditor never sees payloads.

## Accessibility Rules
- Auth UI: keyboard navigable; biometric prompts surface OS-native UI (which the OS handles for accessibility).

## Performance Rules
- Unlock (passkey path): < 500 ms after user gesture.

## Observability Rules
- Every auth attempt and authz decision is logged and audited.

## Required Tests
- Per-path auth ceremony tests.
- Lockout backoff test.
- Full (Role × Operation) allow/deny matrix tests.
- Negative tests: bad proof, expired session, replayed message.
- Idle and absolute timeout tests.

## Acceptance Criteria
- All three auth paths work.
- RBAC matrix complete and tested.
- T-007, T-014..T-016 mitigated with linked tests.
