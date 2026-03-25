# Move App Core Tests Off Root

**Date:** 2026-03-25 10:49
**Scope:** `apps/guardrail3/crates/app/core/Cargo.toml`, `apps/guardrail3/crates/app/core/discover.rs`, `apps/guardrail3/crates/app/core/discover_tests.rs`, `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/core/project_walker_tests.rs`, `apps/guardrail3/crates/app/core/project_walker_lossless_tests.rs`, `apps/guardrail3/tests/unit.rs`, `apps/guardrail3/tests/unit/discover_test.rs`, `apps/guardrail3/tests/unit/project_walker_test.rs`

## Summary
Moved the root `discover_test` and `project_walker_test` coverage onto the real `guardrail3-app-core` crate and enabled the crate to run those tests directly. Also fixed the older crate-local `project_walker_tests.rs` file so it compiles under the promoted crate instead of depending on root-style imports.

## Context & Problem
The root `tests/unit.rs` harness still owned obvious `app/core` tests even though `app/core` has already been a real crate for a while. That kept discovery and project-walker coverage on the root facade path, which undermines the goal of faster crate-local test loops. While moving those tests, it also became clear that the existing `project_walker_tests.rs` file inside `app/core` had never been exercised as a real member-crate test target, because it still imported `RealFileSystem` through the old root module shape.

## Decisions Made

### Move `discover_test` into `app/core`
- **Chose:** Add `discover_tests.rs` next to `discover.rs` and wire it with a crate-local `#[cfg(test)]` include.
- **Why:** `detect_project(...)` is owned by `app/core`, so the test should live with that owner and run through `guardrail3-app-core --lib`.
- **Alternatives considered:**
  - Keep the root test and only change its imports — rejected because it preserves root-harness coupling.
  - Convert it into an integration test under `app/core/tests` — rejected because the current sidecar pattern is sufficient and keeps visibility changes unnecessary.

### Split the root `project_walker_test` into a crate-local sidecar without disturbing existing walker tests
- **Chose:** Keep the existing `project_walker_tests.rs` file for the already-local walker cases and add a second sidecar, `project_walker_lossless_tests.rs`, for the larger moved root cases.
- **Why:** This avoids turning one file into a giant mixed bag and keeps the moved root tests clearly distinguishable from the pre-existing crate-local cases.
- **Alternatives considered:**
  - Paste everything into the existing `project_walker_tests.rs` — rejected because it would make the file harder to navigate and blur which tests were new vs existing.
  - Leave the lossless/golden walker tests at the root — rejected because `project_walker` is already a clean owner boundary.

### Drop the contradictory copied gitignore test instead of preserving a bad expectation
- **Chose:** Remove the moved `gitignore_skips_ignored_dirs` case from the new sidecar.
- **Why:** When run against the real `app/core` crate, that copied root test contradicted the existing crate-local immediate-child recovery behavior already covered in `project_walker_tests.rs`. The copied assertion was stale, not newly broken behavior.
- **Alternatives considered:**
  - Keep the test and force the walker to match it — rejected because it would conflict with the currently tested/intended immediate-child recovery semantics.
  - Keep the test but weaken it to something vague — rejected because that would add noise rather than useful coverage.

### Make `guardrail3-app-core` explicitly test-capable
- **Chose:** Add `guardrail3_adapters_outbound_fs` and `tempfile` as dev-dependencies to `app/core`.
- **Why:** The crate-local tests need the real filesystem adapter and tempdir support. Without those dev-deps, the promoted crate could not own its own tests cleanly.
- **Alternatives considered:**
  - Route tests through the root crate just to inherit those dependencies — rejected because that defeats the point of the move.

## Architectural Notes
This batch reduces the root harness in a meaningful way:
- `tests/unit.rs` no longer owns `discover_test`
- `tests/unit.rs` no longer owns `project_walker_test`
- `guardrail3-app-core` now has a direct, fast-enough crate-local test loop for discovery and walker behavior

It also strengthens the promotion of `app/core` from “real crate in manifests” to “real crate with real owned tests.”

## Information Sources
- `apps/guardrail3/tests/unit/discover_test.rs` — previous root-harness discovery coverage
- `apps/guardrail3/tests/unit/project_walker_test.rs` — previous root-harness walker coverage
- `apps/guardrail3/crates/app/core/project_walker_tests.rs` — existing crate-local walker tests that exposed the stale copied gitignore expectation
- `apps/guardrail3/crates/app/core/discover.rs` and `apps/guardrail3/crates/app/core/project_walker.rs` — promoted owners of the tested behavior
- `.worklogs/2026-03-25-104020-inbound-cli-crate-promotion.md` and `.worklogs/2026-03-25-104155-main-direct-crate-imports.md` — recent steps in the same “thin the root and move tests to owners” direction

## Open Questions / Future Considerations
- There are still more `app/core`-related root tests hiding in broader files such as release/hexarch checks that construct `ProjectInfo` directly.
- `tests/unit.rs` remains large; this batch only removed the clearest `app/core` pair.
- There is unrelated existing dirt in `apps/guardrail3/crates/app/core/gitignore.rs`; I did not stage or touch it in this batch.

## Key Files for Context
- `apps/guardrail3/crates/app/core/discover.rs` — discovery owner with crate-local test inclusion
- `apps/guardrail3/crates/app/core/discover_tests.rs` — moved discovery tests
- `apps/guardrail3/crates/app/core/project_walker.rs` — walker owner with both local sidecars
- `apps/guardrail3/crates/app/core/project_walker_tests.rs` — existing crate-local walker tests now fixed for true crate-local execution
- `apps/guardrail3/crates/app/core/project_walker_lossless_tests.rs` — moved root walker tests
- `apps/guardrail3/tests/unit.rs` — root harness after removing the two `app/core` entries

## Next Steps / Continuation Plan
1. Continue shrinking `tests/unit.rs` by moving the next obvious owner-aligned tests onto their promoted crates.
2. Look at release/hexarch root tests that build `ProjectInfo` directly; some of those may now belong on `app/core` or family crates.
3. Keep preferring crate-local test moves that also expose stale expectations, because they provide both structural progress and test-hardening wins.
