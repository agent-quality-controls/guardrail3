# Refresh Rust Plans And Handoffs

**Date:** 2026-03-30 15:26
**Scope:** `.plans/**`, project-level handoffs, Rust family plan surfaces, README additions under `apps/guardrail3/crates/app/core/project_walker` and `apps/guardrail3/crates/app/rs/families/release`

## Summary
This commit captures the planning and handoff churn that accumulated around the Rust-only direction, the checker-architecture migration, and the per-family implementation wave. It updates the family plan surfaces, adds missing README/handoff context, and removes stale migration-plan fragments that no longer match the current crate layout.

## Context & Problem
The repository had drift between the live Rust family implementation work and the planning material that future agents are expected to read first. Several family plan files still described incomplete or pre-split layouts, new agent briefs were sitting uncommitted, and older migration-plan files were still present even though the repo had already moved to the `ProjectTree` and per-family crate direction.

The user asked to treat the whole dirty tree as owned state and commit it in a structured way. Splitting the docs/plans refresh away from the code migration keeps the narrative recoverable: future work can read the updated planning material without wading through the much larger code diff first.

## Decisions Made

### Separate planning churn from code churn
- **Chose:** Commit the plan/README/handoff updates separately from the code migration.
- **Why:** The planning material is useful on its own and should be readable as a coherent state transition.
- **Alternatives considered:**
  - One monolithic commit for everything — rejected because the plan/history signal would be buried in a massive code diff.
  - Leave plans out of the commit — rejected because the user explicitly asked to commit all owned state.

### Preserve the new Rust-only direction in the plan surface
- **Chose:** Keep the Rust family plan files and hardening briefs aligned with the current per-family implementation push and remove stale migration-plan fragments.
- **Why:** AGENTS.md and the handoff material direct future sessions to these files first; stale planning would mislead future work.
- **Alternatives considered:**
  - Keep old migration notes for posterity — rejected because they were no longer the operative source of truth.

## Architectural Notes
This commit does not change runtime behavior directly. Its value is that it makes the human-facing architecture story match the codebase direction:
- Rust-only enforcement is the active roadmap.
- Family-local implementation and test hardening are the active execution model.
- The old migration-plan fragments are no longer the authoritative path.

## Information Sources
- `AGENTS.md`
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `.plans/by_family/rs/*`
- `.plans/todo/checks/rs/*`
- existing recent worklogs, especially `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md`
- live repo layout under `apps/guardrail3/crates/app/rs`

## Open Questions / Future Considerations
- Some plan files may still lag the exact current compile state of the code migration.
- The code commit that follows still carries an incomplete syntax-repair sweep; this docs commit only captures the planning state.

## Key Files for Context
- `AGENTS.md` — current repo-wide execution rules and Rust-only direction
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — checker architecture source of truth
- `.plans/todo/checks/rs` — family inventory plans that drive the migration
- `.plans/todo/check_review/test_hardening/README.md` — active hardening brief entrypoint
- `apps/guardrail3/crates/app/core/project_walker/README.md` — added local context for the project walker area
- `apps/guardrail3/crates/app/rs/families/release/README.md` — release-family local handoff/readme
- `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md` — prior related runtime-split verification

## Next Steps / Continuation Plan
1. Read the code migration worklog committed next to understand the runtime/root-scope and family-code changes.
2. Resume the syntax-repair sweep from the remaining `cargo check` frontier rather than reopening deleted/stale migration notes.
3. After the code tree compiles cleanly, do a smaller follow-up docs pass only if any of the updated family plans still overstate implementation status.
