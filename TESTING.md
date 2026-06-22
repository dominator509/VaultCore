# TESTING.md

## Test Strategy
VaultCore tests follow a pyramid with two language layers (Rust + TS) and one cross-process integration layer (Builder ↔ Verifier IPC).

- **Unit tests:** pure domain (FSM, validation, error mapping), pure crypto primitives, UI components, utility functions.
- **Integration tests:** repository ↔ SQLite, Builder ↔ Verifier signed IPC, audit chain append/verify, SpecAnchor sign/verify.
- **Invariant enforcement tests:** dedicated `cargo nextest run --test invariants` suite that fails when any of I-1..I-8 is violated.
- **E2E acceptance tests:** Playwright against a built Tauri dev binary, covering unlock, search, reveal, copy with auto-clear, rotate, audit view, RBAC denials.
- **Contract tests:** signed-message schemas between Builder and Verifier, SpecAnchor format, vault file format.
- **Smoke tests:** scripted end-to-end run on a fixture vault that verifies critical paths and audit chain integrity.

## Unit Test Rules
- Domain code (`crates/core`) must be testable without infrastructure or process boundaries.
- Crypto primitives must include known-answer tests (KATs) from authoritative test vectors.
- UI components must be tested with Vitest + React Testing Library; no network mocks (there is no network).
- Tests must be deterministic; randomness must be seeded.

## Integration Test Rules
- Builder ↔ Verifier tests must run two real processes (or two threads in `cfg(test)` only with documented justification), exchange real signed messages, and assert on observable behavior.
- Persistence tests must use a temporary SQLite file under `target/test-vaults/`.
- Audit chain tests must include a tamper case: mutate a row, re-verify, expect failure.
- SpecAnchor tests must include a tampered SpecAnchor case: expect Builder/Verifier startup failure with a clear error.

## E2E / Acceptance Test Rules
- Critical user flows must be covered:
  - Unlock (passkey path, biometrics path, passphrase fallback)
  - Search (metadata-only, sub-200 ms on 10,000-record fixture)
  - Reveal a secret with auto-clear timer
  - Copy a secret with auto-clear timer
  - Create a secret of each of the eight types
  - Rotate a secret (lifecycle transitions: active → rotating → active)
  - Soft-delete and purge (with cryptographic erasure)
  - Audit view: list, filter, verify chain integrity from the UI
  - RBAC denial: Viewer attempts a write, sees a stable error
- Loading, empty, and error states must each have at least one assertion.

## Contract Test Rules
- Builder ↔ Verifier signed messages: every message type has a schema test asserting field set, type set, ordering, and signature validity.
- Vault file format: round-trip read/write tests with version markers.
- SpecAnchor format: parse, verify, reject unknown versions.

## Smoke Test Rules
- Boots Builder + Verifier with a known SpecAnchor.
- Performs unlock + reveal + audit-verify on a fixture vault.
- Shuts down cleanly.
- Must complete in under 10 seconds on CI.

## Regression Test Rules
- Every fixed bug with user impact or invariant impact gets a regression test.
- Regression tests live next to the layer that owned the bug, with a comment linking the issue ID.

## Performance Test Rules
- Cold start to lock screen: < 1.5 s (measured in smoke test).
- Unlock (passkey path): < 500 ms after user gesture.
- Search across 10,000 metadata records: < 200 ms (Rust microbench + Playwright timing assertion).
- Decrypt + reveal a single payload: < 100 ms after countersignature.
- Performance regressions > 25 % from baseline must fail CI.

## Accessibility Test Rules
- Every primary flow must be navigable by keyboard only.
- Playwright + `@axe-core/playwright` must report zero serious or critical violations on unlock, list, reveal, audit screens.
- No color-only state communication.

## Security Test Rules
- Trust-boundary validation: every Builder entry point and every Verifier entry point has at least one negative test (bad input, expired session, wrong role).
- Authz tests cover every (Role × Operation) cell explicitly.
- Redaction tests: synthesize a log line with a fake payload tag, assert it never appears verbatim.
- Dependency audit (`./scripts/dependency-audit.sh`) must be green or have an explicit accepted advisory in `deny.toml`.

## Invariant Enforcement Tests
Each invariant I-1..I-8 has at least one test that fails when the invariant is violated:
- I-1 (no plaintext at rest): scan SQLite for known payload markers after a write — must not appear.
- I-2 (JIT decrypt): Builder must not hold plaintext after a reveal completes (Drop-zeroize assertion).
- I-3 (metadata vs payload split): payload columns must round-trip ciphertext-only.
- I-4 (Trinity boundary): plaintext never appears in Verifier address space (process boundary test with `cfg(test)` introspection in Builder only).
- I-5 (no Builder write without Verifier countersig): bypass attempt fails with a stable error.
- I-6 (SpecAnchor signed): tampered SpecAnchor causes startup failure.
- I-7 (no vendor backdoor): static analysis test + dependency-allowlist test.
- I-8 (every action audited): every write path emits an audit entry; missing audit entry causes test failure.

## Test Data Rules
- Use only synthetic fixtures from `tests/fixtures/` and `app/tests/fixtures/`.
- Never commit a real secret, real signing key, or a real SpecAnchor.
- Fixture vault files are generated by `scripts/generate-test-vaults.sh` (created in EP-007) from deterministic seeds.

## Mocking Rules
- Do not mock crypto primitives in integration or invariant tests.
- Do not mock the audit chain.
- UI tests may mock the Tauri IPC bridge only for isolated component tests; flow-level tests must use the real bridge against the dev binary.

## Fixture Rules
- Fixtures are version-controlled and deterministic.
- Each fixture is documented with a short note explaining what it represents and which tests use it.

## Required Tests Per Feature
At minimum each new feature adds:
- Rust unit tests for domain and crypto.
- Integration tests for any boundary crossed.
- TS unit tests for UI logic.
- Playwright E2E if user-visible.
- Regression test if fixing a bug.
- Invariant enforcement test if the feature touches I-1..I-8.

## Validation Matrix
- `./scripts/lint.sh`
- `./scripts/format-check.sh`
- `./scripts/typecheck.sh`
- `./scripts/test-unit.sh`
- `./scripts/test-integration.sh`
- `./scripts/test-e2e.sh`
- `./scripts/build.sh`
- `./scripts/security-check.sh`
- `./scripts/dependency-audit.sh`
- `./scripts/smoke-test.sh`
- `./scripts/verify.sh`
- `./scripts/production-readiness-check.sh`

## Definition of Test Done
Testing is done only when:
- Required tests exist at the right layers.
- Relevant validation commands pass.
- Invariant enforcement tests for any touched invariant are present.
- Threat-model rows touched by the change reference the new test IDs.
- TRACEABILITY.md advances for affected rows.
- Flaky behavior is fixed or has a documented mitigation plan.
