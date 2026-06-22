# EP-005 User Interface (Tauri + React)

## 1. Purpose / Big Picture
Implement the TypeScript + React UI inside Tauri covering every primary flow with full state coverage and WCAG 2.1 AA accessibility.

## 2. Scope
- Routes/screens: Lock, Unlock, List/Search, Reveal, Create per-type, Edit, Rotate, Audit View, Vault Health, Settings (auto-clear TTLs).
- States: loading, empty, success, error, denied.
- Auto-clear timer behavior for reveal + clipboard.
- Accessibility baseline (axe).
- E2E Playwright flows.

## 3. Non-goals
- Marketing-quality visual polish.
- Localization beyond English (v1).
- Browser extension or mobile.

## 4. Context and Orientation
After EP-004 (Tauri commands exist). Reads SPEC-004 and SPEC-006.

## 5. Files to Read First
- `.agent/specs/SPEC-004-ui-ux-behavior.md`
- `.agent/specs/SPEC-006-error-handling.md`
- `app/src-tauri/src/main.rs` (command list)

## 6. Files to Change
- `app/src/{routes,components,state,i18n/en.json}`
- `app/src/lib/tauri.ts` (typed bridge)
- `app/tests/e2e/*.spec.ts` (Playwright)
- `app/tests/unit/*.test.ts` (Vitest)
- `TRACEABILITY.md` UI rows
- This ExecPlan

## 7. Interfaces and Contracts
- All Tauri commands typed via `app/src/lib/tauri.ts` matching SPEC-003.

## 8. Milestones

### Milestone 1 — Lock + Unlock flow + Settings stub
- **Goal:** App boots to Lock; passkey/biometrics/passphrase paths reach Builder.
- **Validation Command:** `pnpm --dir app test:e2e -- unlock.spec.ts`
- **Expected Result:** Green E2E (against dev binary built earlier in run).
- **Recovery Instruction:** If passkey UI isn't available in test env, gate path with `VAULTCORE_E2E_PASSKEY=0` and document.

### Milestone 2 — List/Search + Reveal + Copy with auto-clear
- **Goal:** Critical happy path: search by metadata, reveal with TTL, copy with TTL.
- **Validation Command:** `pnpm --dir app test:e2e -- reveal.spec.ts copy.spec.ts`
- **Expected Result:** Green; auto-clear assertions present.
- **Recovery Instruction:** Never extend TTL or remove auto-clear to make a test pass.

### Milestone 3 — Create/Edit/Rotate per type + Soft delete + Purge with confirmation token
- **Goal:** Lifecycle UI complete; purge requires confirmation token.
- **Validation Command:** `pnpm --dir app test:e2e -- lifecycle.spec.ts`
- **Expected Result:** Green.
- **Recovery Instruction:** If purge confirmation flow is fragile, STOP rather than weakening confirmation.

### Milestone 4 — Audit View + Vault Health + Accessibility + Performance
- **Goal:** Audit list/filter/verify chain; Vault Health surfaces SpecAnchor + audit head + session info; axe shows zero serious/critical; timing budgets enforced.
- **Validation Command:** `./scripts/test-e2e.sh && pnpm --dir app test:a11y`
- **Expected Result:** Green; performance budgets met; TRACEABILITY UI rows advance.
- **Recovery Instruction:** Accessibility/perf failure ⇒ fix UI, never disable check.

## 9. Concrete Steps
1. Build typed Tauri bridge.
2. Implement flows per milestone.
3. Add E2E + a11y tests at each milestone.

## 10. Validation and Acceptance
- All primary flows covered.
- All state types present.
- a11y and performance budgets met.
- TRACEABILITY UI rows VERIFIED.

## 11. Idempotence and Recovery
- UI changes are reversible; favor smallest viable component changes.

## 12. Progress
- [x] Milestone 1 complete (Lock/Unlock flow, passkey/biometrics/passphrase controls, Settings stub; `pnpm --dir app test:e2e -- unlock.spec.ts` passed)
- [x] Milestone 2 complete (List/search, reveal TTL countdown, copy TTL countdown; `pnpm --dir app test:e2e -- reveal.spec.ts copy.spec.ts` passed)
- [x] Milestone 3 complete (Create/edit/rotate/soft-delete/purge confirmation flow; `pnpm --dir app test:e2e -- lifecycle.spec.ts` passed)
- [x] Milestone 4 complete (Audit View, Vault Health, axe accessibility gate; `./scripts/test-e2e.sh && pnpm --dir app test:a11y` passed)

## 13. Surprises & Discoveries
- Playwright and axe were not installed in the app workspace. Added `@playwright/test` and `@axe-core/playwright` as dev dependencies for EP-005 validation.
- The EP-005 final validation command referenced `pnpm --dir app test:a11y`, but the script was missing. Added the script and documented it in `COMMANDS.md` before relying on it.
- `pnpm --dir app test:e2e -- unlock.spec.ts` passed a literal `--` through to Playwright. Added `app/tests/run-playwright.mjs` to strip the separator and preserve ExecPlan filename filters.
- Playwright Chromium was not installed locally. Installed the Chromium/headless-shell runtime with `pnpm --dir app exec playwright install chromium`.
- Initial copy countdown rendered on every secret card because it was global state. Scoped it to the copied `secret_id` so auto-clear status is specific to one card.
- Lifecycle E2E needed to scope the submit button to the create form because the navigation tab and form action both use the visible label `Create`.
- Vite warned when the same `en.json` module was imported with mixed JSON import attributes. Standardized JSON imports with `with { type: "json" }`.

## 14. Decision Log
- ADR-0006 (Tauri) confirmed at start of this plan.
- Implemented a typed Tauri bridge with deterministic browser-test mocks for non-Tauri execution. This keeps tests local and avoids remote calls while matching SPEC-003 command names.
- Used a compact single-shell React route surface rather than adding a router dependency. This keeps EP-005 focused on primary flows and state coverage without introducing new navigation infrastructure.

## 15. Outcomes & Retrospective
- EP-005 completed locally.
- Tauri React UI now covers lock/unlock, settings, list/search, reveal, copy, create, edit, rotate, soft delete, purge, audit view, vault health, and manual lock flows.
- Typed bridge, UI state helpers, localized strings, Playwright E2E tests, Vitest unit tests, and axe accessibility tests are implemented.
- Full local verification passed with `./scripts/verify.sh`; after a one-line Vite warning cleanup, `pnpm --dir app format:check`, `pnpm --dir app lint`, and `pnpm --dir app build` also passed.
- TRACEABILITY UI rows advanced to `VERIFIED`.
- Remaining implementation depth is deferred to later ExecPlans: real auth ceremonies, persistence-backed command behavior, richer production observability, and release-readiness evidence.
