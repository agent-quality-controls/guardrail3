# Rust Checks And ProjectTree Cleanup

**Date:** 2026-03-22 17:06
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/{cargo,fmt,toolchain}`, `apps/guardrail3/crates/domain/project_tree.rs`, `apps/guardrail3/crates/domain/project_tree_tests.rs`, `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/tests/unit/project_walker_test.rs`

## Summary
Cleaned up the first remaining dirty bucket after the clippy work: the new Rust checks families and the shared project-tree/project-walker support tests. The changes are formatting and readability cleanup only, with targeted test runs to confirm the bucket still behaves exactly the same.

## Context & Problem
After the clippy completion work, the repo still had a large dirty backlog spanning several unrelated areas. The user asked to clean the repo. The first bucket was the new Rust check families (`cargo`, `fmt`, `toolchain`) plus the shared `ProjectTree` / project walker support code and tests. Before committing anything, this bucket needed to be separated from the legacy arch/test churn so the cleanup history stays readable.

## Decisions Made

### Commit the new-checks and project-tree cleanup separately
- **Chose:** Keep the Rust checks/project-tree cleanup in its own commit.
- **Why:** These files are the active architecture and should not be mixed with legacy arch validator formatting churn.
- **Alternatives considered:**
  - Commit all dirty files together — rejected because it would hide the active-code cleanup inside unrelated legacy/test noise.
  - Leave the cleanup uncommitted — rejected because the user explicitly asked to clean the repo.

### Treat this bucket as cleanup-only, not feature work
- **Chose:** Record this as formatting/readability cleanup with verification, not as a behavioral change.
- **Why:** The diffs in this bucket are line wrapping, import ordering, and test assertion formatting. There was no policy or rule change to describe.
- **Alternatives considered:**
  - Reframe it as a semantic follow-up to cargo/fmt/toolchain — rejected because that would overstate the actual change.
  - Drop the cleanup instead of committing it — rejected because it would leave the repo dirty for no product reason.

## Architectural Notes
This cleanup touches the new-family architecture directly:
- `cargo`, `fmt`, and `toolchain` remain the active reference families under `app/rs/checks/rs/`.
- `ProjectTree` and `project_walker` stay the shared discovery substrate for the new checker pipeline.
- The separation from the legacy `rs/validate/*` arch code is intentional; those files are handled in a later cleanup bucket.

## Information Sources
- `git status --short`
- `git diff --stat`
- Focused diffs for:
  - `apps/guardrail3/crates/app/rs/checks/rs/cargo/*`
  - `apps/guardrail3/crates/app/rs/checks/rs/fmt/*`
  - `apps/guardrail3/crates/app/rs/checks/rs/toolchain/*`
  - `apps/guardrail3/crates/domain/project_tree.rs`
  - `apps/guardrail3/crates/domain/project_tree_tests.rs`
  - `apps/guardrail3/crates/app/core/project_walker.rs`
  - `apps/guardrail3/tests/unit/project_walker_test.rs`
- Prior clippy worklogs from the current session, especially:
  - `.worklogs/2026-03-22-170103-clippy-completeness-finish.md`

## Open Questions / Future Considerations
- The larger remaining dirty bucket is still the legacy Rust/TS arch validator and test formatting churn.
- If any of that second bucket turns out to contain a real semantic edit instead of formatting, it should be split again before commit.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/mod.rs` — orchestrator for the cargo family in the new architecture
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/toolchain_tests.rs` — sidecar test pattern for a completed family
- `apps/guardrail3/crates/domain/project_tree.rs` — generic repository snapshot and query helpers used by family orchestrators
- `apps/guardrail3/crates/app/core/project_walker.rs` — builder for `ProjectTree`
- `apps/guardrail3/tests/unit/project_walker_test.rs` — exhaustive behavior checks for the walker/tree substrate
- `.worklogs/2026-03-22-170103-clippy-completeness-finish.md` — prior worklog that explains the most recent architecture-completion line before this cleanup

## Next Steps / Continuation Plan
1. Stage and commit this cleanup bucket with the worklog.
2. Audit the remaining dirty legacy arch/test files one more time for any non-formatting surprises.
3. Write a second worklog and commit the legacy arch/test cleanup separately so `git status` is clean.
