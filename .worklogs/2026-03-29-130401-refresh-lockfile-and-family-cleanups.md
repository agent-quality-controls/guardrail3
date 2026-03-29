# Refresh Lockfile And Family Cleanups

**Date:** 2026-03-29 13:04
**Scope:** `apps/guardrail3/Cargo.lock`, `apps/guardrail3/crates/app/rs/families/deps/**`, `apps/guardrail3/crates/app/rs/families/fmt/**`, `apps/guardrail3/crates/app/rs/families/hexarch/**`

## Summary
Committed the remaining small cleanup bucket after the larger `release` and `deny` checkpoints. This refreshes the app lockfile for the newly added release companion crates, fixes a `missing_debug_implementations` break in `deps` test support, and captures the remaining formatting-only edits in `deps`, `fmt`, and `hexarch`.

## Context & Problem
After separating the large structural buckets, the worktree still had a handful of unrelated leftovers:
- `Cargo.lock` needed to reflect the new release companion crates and current workspace dependency graph.
- `deps` family verification failed because `StubToolChecker` in test support did not implement `Debug` under the current workspace lint settings.
- `deps`, `fmt`, and `hexarch` also had small formatting-only diffs left behind from prior verification runs.

These did not belong inside the larger feature commits, but they still needed to be preserved to get the tree back to a clean committed state.

## Decisions Made

### Keep the final leftovers in one small cleanup commit
- **Chose:** bundle the lockfile refresh, the `deps` `Debug` fix, and the formatting-only file churn into one final cleanup commit.
- **Why:** all of these were low-scope, low-risk leftovers after the larger family checkpoints and do not warrant separate history entries.
- **Alternatives considered:**
  - Fold the lockfile into the earlier release commit retroactively — rejected because those commits are already written and should stay focused.
  - Split formatting-only files into their own commit — rejected because that would create needless noise.

### Fix the `deps` test-support lint break instead of just documenting it
- **Chose:** add `#[derive(Debug)]` to `StubToolChecker`.
- **Why:** the failure was real and trivial to repair; carrying it forward as “known broken” would be unnecessary churn.
- **Alternatives considered:**
  - Leave the failure for a later pass — rejected because it would keep the family test target broken for no good reason.

## Architectural Notes
This commit does not change family semantics. It only:
- updates the workspace lockfile to match already-committed package topology
- keeps test-support helpers aligned with workspace lint expectations
- records minor formatting normalization in existing files

## Information Sources
- `git diff --stat` / `git status --short` after the larger commits
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deps --lib`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-fmt --lib`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`

## Open Questions / Future Considerations
- This commit does not address the remaining family-level `RS-TEST` debt in `release` or `deny`; those are tracked in their own worklogs.
- `Cargo.lock` now reflects the current package graph, but future package moves will need another refresh.

## Key Files for Context
- `apps/guardrail3/Cargo.lock` — refreshed workspace package graph after recent family splits
- `apps/guardrail3/crates/app/rs/families/deps/test_support/src/lib.rs` — `Debug` fix for the test-support stub
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/rs_deps_11_input_failures_tests/fail_closed.rs` — formatting-only cleanup in the deps attack suite
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_04_nightly_keys_on_stable_tests/mod.rs` — formatting-only cleanup
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_27_nested_workspace_forbidden.rs` — formatting-only cleanup

## Next Steps / Continuation Plan
1. Resume the substantive `RS-TEST` migration work in `release`, using the live `RS-TEST-16` output as the checklist.
2. After `release`, return to the remaining deny-family self-hosting debt (`RS-DENY-01` plus deny-sidecar `RS-TEST-16` hits).
3. Once those families are clean, rerun repo-root `test`, `code`, `cargo`, `clippy`, and `hexarch` validation to re-establish the next priority bucket.
