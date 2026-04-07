# Parser And Workspace Cleanup

**Date:** 2026-04-07 19:22
**Scope:** `packages/parsers/cargo-toml-parser`, `packages/parsers/guardrail3-rs-toml-parser`, `packages/rs/g3rs-workspace-crawl`, `packages/shared/guardrail3-check-types`

## Summary
Committed the remaining uncommitted parser/workspace/shared-package cleanup as one grouped change. This pass is mechanical cleanup only: formatting, import/export ordering, and test readability updates, with no intended behavior change.

## Context & Problem
After the deps work was committed, the repo still had one dirty group left across parser and shared utility packages. The diff did not represent a new feature lane; it was one cleanup cluster touching:

- parser assertions and test formatting
- parser/runtime re-export ordering
- workspace-crawl assertion/test formatting
- shared check-types module ordering

Because these files were all already dirty and tested cleanly together, the right move was to group them into one low-risk cleanup commit instead of forcing arbitrary micro-commits.

## Decisions Made

### Commit the remaining dirty parser/workspace/shared files together
- **Chose:** One cleanup commit for the leftover parser, workspace-crawl, and shared check-types files.
- **Why:** The remaining diff is cohesive as formatting/readability/export-order cleanup and does not benefit from further splitting.
- **Alternatives considered:**
  - Split by package — rejected because it would create multiple tiny commits with no meaningful architectural separation.
  - Leave them uncommitted — rejected because the user explicitly asked to sort and commit the remaining dirty state.

### Treat this as non-semantic cleanup
- **Chose:** Verify the affected package workspaces with tests, but avoid reframing the commit as a feature change.
- **Why:** The diff is dominated by line wrapping, import ordering, and small presentation cleanup; the verification burden should match that.
- **Alternatives considered:**
  - Re-audit every test as if behavior changed — rejected because there was no evidence of a semantic change in this leftover set.

## Architectural Notes
No new architecture was introduced here. The practical effect is:

- parser crates keep the same public API, just with cleaned re-export ordering
- parser tests and assertion helpers are more consistently formatted
- workspace-crawl helper/tests stay behaviorally the same, with readability cleanup only
- shared check-types keeps the same exports, with module declaration order cleaned up

## Information Sources
- local git diff for the remaining dirty tree after the deps commits
- `cargo test --workspace -q` in `packages/parsers/cargo-toml-parser`
- `cargo test --workspace -q` in `packages/parsers/guardrail3-rs-toml-parser`
- `cargo test --workspace -q` in `packages/rs/g3rs-workspace-crawl`
- `cargo test -q` in `packages/shared/guardrail3-check-types/crates/guardrail3-check-types`

## Open Questions / Future Considerations
- None from this cleanup itself. Any future semantic changes in these packages should be recorded as their own worklog-backed commits instead of being mixed into formatting churn.

## Key Files for Context
- `packages/parsers/cargo-toml-parser/crates/parser/runtime/src/lib.rs` — parser runtime re-export ordering
- `packages/parsers/cargo-toml-parser/crates/parser/runtime/src/parser_tests/parsing.rs` — parser test readability cleanup
- `packages/parsers/guardrail3-rs-toml-parser/crates/parser/runtime/src/lib.rs` — parser runtime re-export ordering
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/crawl_tests/ignore_state.rs` — workspace crawl test readability cleanup
- `packages/shared/guardrail3-check-types/crates/guardrail3-check-types/src/lib.rs` — shared module ordering cleanup

## Next Steps / Continuation Plan
1. Continue on new feature work from a clean worktree.
2. If parser or workspace-crawl behavior changes later, keep those as separate commits from formatting-only cleanup.
