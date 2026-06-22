# .agent/EXECUTION_RULES.md

## One Active ExecPlan Rule
Only one ExecPlan may be active at a time. Do not interleave work.

## No Hidden Context Rule
Assume no prior conversation exists. Everything needed must come from repository files.

## No Roadmap-Only Implementation Rule
`ROADMAP.md` is strategic only. Never implement directly from it.

## Continue-by-Default Rule
Proceed autonomously through the active ExecPlan. Do not ask for next steps unless a STOP condition applies.

## STOP-Only Rule
Stop only for STOP conditions defined in `AGENTS.md`. Examples:
- missing signing key or hardware authenticator,
- code path would introduce a remote network call,
- code path would weaken Trinity boundary, SpecAnchor verification, or audit chain.

## Anti-Drift Rule
Do only the work required by the active ExecPlan. No unrelated refactors, no new roles, no new secret types, no broad cleanup.

## Anti-Hallucination Rule
Verify every command, crypto primitive, IPC message type, env var, role, secret type, audit field, and config key from repository files before use. Use only the locked crypto set (XChaCha20-Poly1305, HKDF-SHA-512, Argon2id, Ed25519) per ADR-0008.

## Anti-Fixation Rule
Use bounded retries (1st, 2nd, 3rd). After three same-root failures, abandon the approach, document failed hypotheses, choose simpler path; never tune crypto parameters or relax signatures to make a test pass.

## Test-Before-Completion Rule
Do not claim completion without running the required validation commands or documenting a STOP condition.

## Diff Review Rule
Run `git diff --name-only` before completion. Compare to the active ExecPlan's expected changed files. Justify any extra file in the Decision Log.

## Final Response Rule
Final response must include:
- ExecPlan completed,
- changed files,
- commands run,
- command results,
- acceptance criteria status,
- decisions made (with ADR IDs),
- assumptions confirmed or changed,
- remaining risks (including residual risks R-1..R-5 if touched),
- TRACEABILITY.md row status changes,
- whether production-readiness criteria advanced and which gate is next.
