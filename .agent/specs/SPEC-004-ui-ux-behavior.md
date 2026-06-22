# SPEC-004 UI / UX Behavior

- **Status:** Draft
- **Owner:** [OWNER_TO_SET]
- **Linked Roadmap Phase:** Phase 4
- **Linked ExecPlans:** EP-005

## User-Visible Goal
Provide a Tauri + React UI that lets target users perform every primary flow with clear states, full keyboard navigation, and WCAG 2.1 AA accessibility.

## Non-Goals
- Browser extension (v1)
- Mobile UI (v1)
- Marketing-quality polish; v1 focuses on correctness and accessibility.

## Terms
- **Primary flow:** a user-visible end-to-end interaction.

## Required Behavior

### Primary Flows
- Unlock (passkey, biometrics, passphrase fallback)
- List + Search (metadata-only)
- Reveal (with explicit reason; auto-clear timer)
- Copy (with auto-clear timer)
- Create (per type, with type-specific form)
- Edit metadata
- Rotate (lifecycle `active → rotating → active`)
- Archive, soft-delete, purge (with confirmation token)
- Audit view (list, filter, verify chain)
- Vault Health view (SpecAnchor status, audit chain head, last activity)
- Lock (manual or idle timeout)

### State Coverage
Every flow has explicit:
- Loading state
- Empty state
- Success state
- Error state (typed by SPEC-006 taxonomy)
- Denied state (RBAC) — surfaces minimum-role hint, never leaks payload presence

### Accessibility
- Keyboard nav for every flow
- Visible focus, semantic landmarks, ARIA where needed
- Minimum 4.5:1 contrast for normal text; 3:1 for large text
- No color-only state communication
- `@axe-core/playwright` shows zero serious/critical violations on primary flows

### Auto-Clear
- Reveal: payload visible for `ttl_ms` configured at SpecAnchor (default 30 s); cleared from DOM on timer or focus loss.
- Copy: clipboard auto-clears after `ttl_ms` (default 20 s); UI shows countdown.

### Localization (v1)
- English only. Strings live in `app/src/i18n/en.json`. No hardcoded user-facing text in components.

## Inputs / Outputs
- UI calls Tauri commands per SPEC-003.
- UI never holds plaintext beyond auto-clear window.

## Error States
- Surface stable error codes from SPEC-006 with human-readable messages.

## Data Rules
- UI displays only fields permitted by role.
- Auditor never sees payloads.

## Security Rules
- Never log payloads.
- Never store payloads in `localStorage`, `sessionStorage`, or IndexedDB.
- Validate input client-side for UX; treat server-side (Builder) validation as authoritative.

## Performance Rules
- Cold start to lock screen < 1.5 s.
- Search results render < 200 ms on 10k records.
- Reveal display < 100 ms after countersignature.

## Observability Rules
- UI logs (ui.log) include op name, status, duration; never payload.

## Required Tests
- Vitest unit tests for components.
- Playwright E2E for each primary flow.
- Accessibility tests via `@axe-core/playwright`.
- Performance timing assertions on critical flows.

## Acceptance Criteria
- All primary flows implemented with all state cases.
- Accessibility tests green.
- Performance budgets met.
