# Commit Assertion And Test Fixes

**Date:** 2026-04-03 19:43
**Scope:** `apps/guardrail3/crates/app/rs/families/{garde,fmt,toolchain,hexarch,test}`, `.worklogs/2026-04-03-194314-commit-all-assertion-and-test-fixes.md`

## Summary
This session picked up from the prior handoff, cleared the remaining assertion drift in `garde`, `fmt`, and `toolchain`, and repaired the broken `hexarch` test surface plus two production ownership issues that were still causing failures. The commit also includes pre-existing dirty-tree edits in the `test` family after verifying that those changes pass their package tests and do not conflict with the handoff work.

## Context & Problem
The prior session handoff identified four immediate buckets: assertion/message drift in `garde`, `fmt`, and `toolchain`, plus `hexarch` compile and test failures after the facade/helper split. The working tree also already contained unrelated `test` family edits when this session resumed. The user explicitly asked to "commit everything" and to remove any stale worktree state, so the task became: make the dirty tree coherent, verify the changed packages, record the reasoning, and commit the whole result without discarding pre-existing work.

## Decisions Made

### Assertion Expectations Were Updated To Match Current Rule Output
- **Chose:** Update affected `garde`, `fmt`, `toolchain`, and `test` assertions/tests to match the current production titles, severities, and messages.
- **Why:** The failures were drift between assertion helpers and the rule output, not evidence that the rules should be rolled back.
- **Alternatives considered:**
  - Revert rule output to older phrasing — rejected because the current rule text was already the active behavior and the tests were stale.
  - Leave the drift in place and commit only `hexarch` work — rejected because the handoff explicitly called these assertion buckets out as the next blocking work.

### Hexarch Test Wiring Was Repaired Locally Instead Of Re-expanding Facade Surfaces
- **Chose:** Restore helper access inside `hexarch` test modules with local wrappers and allow-attributes in test-only modules, while keeping the facade split intact.
- **Why:** The failures came from test module resolution after the facade cleanup. Reintroducing broader facade exports would have undone the structural cleanup that the earlier worklogs were converging on.
- **Alternatives considered:**
  - Re-export helper entrypoints from broader parent modules again — rejected because it would re-couple tests to facade internals.
  - Rewrite all `hexarch` tests to a new harness pattern in one pass — rejected because it was much larger than needed to restore a green package.

### Two Hexarch Production Behaviors Were Tightened Instead Of Masking Them In Tests
- **Chose:** Exclude nested workspaces under `tests/fixtures/` from `RS-HEXARCH-27`, and fix absolute-path fallback handling in `RootWorkspaceMemberHexarchInput::covers_dir()`.
- **Why:** These were real ownership/coverage bugs surfaced by the package tests. Adjusting only the tests would have left the actual family behavior wrong on the repo itself and on absolute app-member paths.
- **Alternatives considered:**
  - Change only expected test output — rejected because the failures represented actual rule/input behavior gaps.
  - Add one-off fixture exceptions in assertions — rejected because the ownership rules belong in production logic.

### The Existing Test-Family Dirty Tree Was Included In The Commit
- **Chose:** Commit the pre-existing `test` family edits together with the handoff fixes after verifying `guardrail3-app-rs-family-test`.
- **Why:** The user explicitly requested "commit everything", and the package tests confirmed the combined tree was coherent.
- **Alternatives considered:**
  - Split the commit and exclude the `test` family edits — rejected because it would have ignored the explicit request.
  - Revert the `test` family edits as unrelated — rejected because they were not my changes and they were already passing.

## Architectural Notes
The broad architectural direction remains the same as the recent facade-cleanup work: keep production surfaces narrow, keep helper/test wiring local, and prefer fixing ownership/coverage logic in production over baking exceptions into assertions. The `test` family edits in this commit also move away from scoped helper wrappers in `tests/mod.rs` and toward full-family execution in the affected tests, which matches the current package behavior those tests are asserting against.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-04-03-185024-session-handoff.md`
- `.worklogs/2026-04-03-180603-fix-deps-release-cargo-assertions.md`
- `.worklogs/2026-04-03-180121-fix-code-assertions.md`
- `.worklogs/2026-04-03-175617-fix-clippy-assertions.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/inputs.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_integrity/rs_hexarch_27_nested_workspace_forbidden/rule.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_02_owned_sidecar_shape/tests/scope.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/mutation/rs_test_12_mutants_toml_exists/tests/scope.rs`
- Verified with:
  - `cargo test -p guardrail3-app-rs-family-garde --lib`
  - `cargo test -p guardrail3-app-rs-family-fmt --lib`
  - `cargo test -p guardrail3-app-rs-family-toolchain --lib`
  - `cargo test -p guardrail3-app-rs-family-hexarch --lib`
  - `cargo test -p guardrail3-app-rs-family-test --lib`
  - `git worktree list --porcelain`
  - `git worktree prune -v`

## Open Questions / Future Considerations
- The handoff mentioned returning to `ARCH-04` and `ARCH-02` after these family failures were cleared; that work is still pending.
- The `test` family’s move away from scoped helper wrappers should be reviewed when validation-scope semantics are revisited, since these tests now assert current full-family behavior rather than scope filtering.
- No stale linked worktree was actually present. If there is an expected external worktree path that should be removed, it was not registered in Git metadata for this checkout.

## Key Files for Context
- `AGENTS.md` — repo operating rules, worklog requirements, and current Rust-only direction
- `.worklogs/2026-04-03-185024-session-handoff.md` — immediate backlog that this session completed
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/inputs.rs` — absolute-path ownership fix for root workspace members
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs` — test helper surface used by many repaired `hexarch` tests
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_integrity/rs_hexarch_27_nested_workspace_forbidden/rule.rs` — fixture-workspace exclusion logic
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_02_owned_sidecar_shape/tests/scope.rs` — representative `test` family scope expectation update
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/mutation/rs_test_09_nextest_timeouts/tests/scope.rs` — representative removal of scoped helper usage in favor of full-family behavior

## Next Steps / Continuation Plan
1. Resume the architecture backlog from the handoff: rerun the arch validation commands that produced the remaining `ARCH-04` and `ARCH-02` buckets.
2. Start with the still-unsplit facade and `mod.rs` offenders identified in the earlier worklogs, then re-bucket any remaining violations after each pass.
3. Revisit `libarch` retirement and any residual `ARCH-06` dependency-chain questions only after the facade-related violations are back under control.
