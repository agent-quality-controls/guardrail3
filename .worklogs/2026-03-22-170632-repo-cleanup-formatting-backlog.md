# Repo Cleanup Formatting Backlog

**Date:** 2026-03-22 17:06
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/{cargo,fmt,toolchain}`, `apps/guardrail3/crates/domain/project_tree*`, `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/rs/validate/**`, `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`, `apps/guardrail3/tests/unit/**`, `apps/guardrail3/tests/adversarial_categories.rs`, related module wiring files

## Summary
This commit cleans the remaining dirty worktree after the clippy implementation line by staging the outstanding formatting and import-order churn across the Rust checks, project tree/walker support, and legacy architecture validation test suites. The purpose is to return the repository to a clean baseline before continuing new rule-family work.

## Context & Problem
After the clippy line was finished, the repository still had a large dirty backlog spread across 50+ files. A review of the remaining diff showed that the overwhelming majority of the changes were formatting-only reflow, import ordering, and module declaration reordering, with no new intended behavior. Leaving that backlog uncommitted would make subsequent work ambiguous and increase the risk of mixing unrelated changes into future commits.

## Decisions Made

### Treat the remaining diff as a cleanup batch
- **Chose:** Commit the remaining dirty worktree as a single cleanup-oriented batch.
- **Why:** The diff clusters shared the same character: formatting/reflow, import ordering, and test-file wrapping. They did not represent a coherent new feature, but they did need to be committed so the repo could return to a clean state.
- **Alternatives considered:**
  - Split into many micro-commits — rejected because the remaining changes did not carry distinct architectural decisions; splitting would create noisy history with little value.
  - Revert the formatting churn — rejected because the user explicitly asked to clean the repo and commit everything, and the diff was already present in the worktree.

### Preserve behavioral work from earlier commits and avoid inventing new logic here
- **Chose:** Limit this commit to the dirty backlog already present and avoid mixing in new code changes.
- **Why:** The substantive architectural work for clippy, cargo, fmt, toolchain, and project-tree helpers was already captured in prior commits and worklogs. This cleanup commit should only normalize the repository state.
- **Alternatives considered:**
  - Fold additional refactors into this commit — rejected because it would blur the boundary between cleanup and new implementation.

## Architectural Notes
This commit does not introduce a new subsystem or change the checker architecture. It restores a clean baseline after the recent Rust-check-family work by settling formatting/import-order churn across:
- the new `checks/rs/*` families,
- `ProjectTree` / project walker support,
- legacy Rust and TS architecture validation code,
- large collocated unit-test suites.

## Information Sources
- `git status --short` — used to identify the remaining dirty files
- `git diff --stat` — used to gauge the size and clustering of the remaining backlog
- Targeted `git diff -- ...` on:
  - `apps/guardrail3/crates/app/rs/checks/rs/{cargo,fmt,toolchain}`
  - `apps/guardrail3/crates/domain/project_tree.rs`
  - `apps/guardrail3/crates/domain/project_tree_tests.rs`
  - `apps/guardrail3/crates/app/core/project_walker.rs`
  - `apps/guardrail3/tests/unit/project_walker_test.rs`
  - `apps/guardrail3/crates/app/rs/validate/**`
  - `apps/guardrail3/tests/unit/{rs_arch_01,ts_arch_01}/**`
- Prior worklogs from the current session, especially:
  - `.worklogs/2026-03-22-170103-clippy-completeness-finish.md`

## Open Questions / Future Considerations
- The legacy Rust and TS architecture validation areas still need real structural refactoring later; this commit only cleans the current backlog.
- `RS-CLIPPY-19` remains intentionally temporary and typo-focused by explicit decision.

## Key Files for Context
- `AGENTS.md` — current repo instructions, worklog rules, and architecture expectations
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — current checker architecture plan
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/mod.rs` — representative new-family orchestrator
- `apps/guardrail3/crates/domain/project_tree.rs` — generic project query helpers that family discovery depends on
- `apps/guardrail3/tests/unit/project_walker_test.rs` — broad walker coverage touched by this cleanup
- `.worklogs/2026-03-22-170103-clippy-completeness-finish.md` — latest clippy completeness context before this cleanup

## Next Steps / Continuation Plan
1. Start the next Rust check family from a clean repo state, most likely `rs/deny`.
2. When touching legacy architecture validation next, decide explicitly whether it is being migrated, audited, or only test-hardened before editing.
3. Keep future commits narrow so this cleanup batch remains the boundary between finished clippy work and the next implementation line.
