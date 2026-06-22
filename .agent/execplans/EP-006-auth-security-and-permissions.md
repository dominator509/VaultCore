# EP-006 Auth, Permissions, and Security Hardening

## 1. Purpose / Big Picture
Implement all three auth paths (passkey, biometrics, passphrase), session lifecycle, exponential lockout, RBAC over five roles, and trust-boundary input validation. Mitigate T-007, T-008, T-009, T-014, T-015, T-016.

## 2. Scope
- WebAuthn passkey ceremony.
- Biometrics platform wrappers.
- Argon2id passphrase KDF with tuned params.
- Session token issuance + idle/absolute timeout.
- Full (Role × Operation) matrix tests.
- Negative auth tests.

## 3. Non-goals
- Federated identity (out of scope).
- Remote recovery (forbidden by I-7).

## 4. Context and Orientation
After EP-004 (sessions plumbing exists). Reads SPEC-005.

## 5. Files to Read First
- `.agent/specs/SPEC-005-auth-and-permissions.md`
- `THREAT_MODEL.md` rows T-007..T-009, T-014..T-016
- `SECURITY.md`

## 6. Files to Change
- `crates/builder/src/auth/{passkey.rs,biometrics.rs,passphrase.rs,platform/*}`
- `crates/verifier/src/policy.rs` (RBAC matrix)
- `crates/builder/src/session.rs` (lifecycle)
- App UI: unlock paths wired through; lockout UI
- Tests (unit, integration, E2E)
- `TRACEABILITY.md` rows for L3
- This ExecPlan

## 8. Milestones

### Milestone 1 — RBAC matrix + default-deny in Verifier
- **Validation Command:** `cargo nextest run -p vaultcore-verifier policy`
- **Expected Result:** Every operation has a default-deny test and an allow test per applicable role.

### Milestone 2 — Passphrase path with Argon2id tuned at install
- **Validation Command:** `cargo nextest run -p vaultcore-builder auth::passphrase`
- **Expected Result:** Params meet minimums; KAT tests pass.

### Milestone 3 — WebAuthn passkey + biometrics adapters
- **Validation Command:** `pnpm --dir app test:e2e -- unlock-passkey.spec.ts && cargo nextest run -p vaultcore-builder auth`
- **Expected Result:** Green where platform supports; gated otherwise.

### Milestone 4 — Lockout + session timeouts + threat-coverage map
- **Validation Command:** `./scripts/security-check.sh && ./scripts/verify.sh`
- **Expected Result:** Green; THREAT_MODEL.md rows T-007..T-009, T-014..T-016 linked to tests; TRACEABILITY L3 rows VERIFIED.

## 9. Concrete Steps
1. Policy matrix and Verifier enforcement.
2. Passphrase path.
3. Passkey + biometrics.
4. Lockout + timeouts; threat-coverage updates.

## 10. Validation and Acceptance
- Per-cell allow/deny tests across (Role × Operation).
- Lockout test passes.
- Threat coverage updated.

## 11. Idempotence and Recovery
- Policy and session changes are additive.
- Never weaken backoff or signing to make a test pass.

## 12. Progress
- [x] Milestone 1 complete
- [x] Milestone 2 complete
- [x] Milestone 3 complete
- [x] Milestone 4 complete

## 13. Surprises & Discoveries
- `THREAT_MODEL.md` is referenced by this ExecPlan and `SECURITY.md`, but is not present in the checkout at the start of EP-006.
- `./scripts/verify.sh` completed with `verify: ok`, while `scripts/build.sh` reported the existing non-fatal Tauri bundling warning `Access is denied. (os error 5)` and continued by design.

## 14. Decision Log
- Implemented the Verifier RBAC matrix as a typed `Operation` enum with an explicit default-deny parser for unknown operation strings.
- Added direct `argon2` and `zeroize` dependencies to `vaultcore-builder` so the passphrase fallback can enforce SPEC-005 install-time Argon2id minimums (`m=64 MiB, t=3, p=1`) without weakening the existing core crypto types.
- Implemented passkey and biometrics as local platform-adapter ceremonies for EP-006 scaffolding; no remote identity provider, escrow, or network recovery path was introduced.
- Added `THREAT_MODEL.md`, `Cargo.lock`, `crates/core/src/crypto/mod.rs`, and `app/tests/e2e/unlock-passkey.spec.ts` beyond the initial file list because EP-006 required threat-row evidence, dependency lock updates, exported AEAD key length, and passkey acceptance coverage.

## 15. Outcomes & Retrospective
- EP-006 completed.
- Verifier now enforces a tested default-deny role-operation matrix.
- Builder now has local passkey, biometrics, and Argon2id passphrase auth paths with negative tests.
- Session lifecycle and lockout policy now cover idle timeout, absolute timeout, immediate revocation, exponential backoff, and success reset.
- `THREAT_MODEL.md` links T-007..T-009 and T-014..T-016 to implementation and validation evidence.
- `TRACEABILITY.md` L3 auth rows advanced to VERIFIED.
