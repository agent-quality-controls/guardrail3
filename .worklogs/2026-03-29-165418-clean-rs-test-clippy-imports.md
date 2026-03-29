# Clean RS-TEST Clippy Imports

**Date:** 2026-03-29 16:54
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/**`, `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/**`

## Summary
Cleared the remaining repo-root `RS-TEST` debt in the `clippy` family by removing direct sidecar imports of `guardrail3_domain_modules`, `guardrail3_domain_report`, and other repo-local crates. The family library suite is green, family-local `--family test` is clean, and an app-root temp-copy attack proves that reintroducing a direct `guardrail3_domain_report::Severity` import in a `clippy` sidecar still trips `RS-TEST-03`.

I also removed one stale forbidden dependency edge from the `arch` assertions crate so the repo-root `RS-TEST` scan stops carrying that leftover `RS-TEST-03` error forward.

## Context & Problem
After the earlier `release`, `garde`, `deps`, `code`, `deny`, and `arch` sweeps, the repo-root `RS-TEST` inventory had dropped substantially but `clippy` still carried a large bucket of `RS-TEST-03` errors. The family itself was structurally migrated already; the remaining failures were all boundary issues in runtime sidecars:

- direct `guardrail3_domain_modules::clippy::*` imports for canonical generated policy baselines
- direct `guardrail3_domain_report::Severity` imports for expected finding construction
- direct calls to `guardrail3_domain_modules::clippy::build_clippy_toml(...)` in policy-context tests

These were repo-root failures rather than family-local ones because the stricter app-root lane treats those imports as disallowed local-crate reach-ins even when the family-local validator stays otherwise clean.

## Decisions Made

### Keep sidecars repo-local-crate free
- **Chose:** rewrite the flagged `clippy` sidecars to use family-owned assertions exports and existing `test_support::build_fixture_clippy_toml(...)` instead of direct `guardrail3_domain_*` imports.
- **Why:** the problem was ownership, not missing coverage. The family already had a clean runtime/assertions/test_support split, so the correct fix was to route tests through that owned surface rather than weaken `RS-TEST-03`.
- **Alternatives considered:**
  - Relax `RS-TEST-03` for generator-parity sidecars — rejected because it would reopen the exact repo-local import loophole the sweep is closing.
  - Hoist baseline constants into `test_support` — rejected because `RS-TEST-18` is intentionally strict about turning `test_support` into a semantic canned-helper bucket.

### Re-export `Severity` from owned assertions modules where sidecars need it
- **Chose:** expose `Severity` from the affected `clippy` assertions modules so sidecars can construct owned `RuleFinding` expectations without importing `guardrail3_domain_report` directly.
- **Why:** the sidecars were already using the corresponding assertions crates. Re-exporting the type from the owned module keeps the proof surface inside family-owned API.
- **Alternatives considered:**
  - Replace all severity-bearing expectations with local ad hoc enums or plain strings — rejected because the assertions modules already use the real severity type and hiding that behind duplicate local encoding adds noise.

### Keep generator-parity tests exact
- **Chose:** preserve exact generator-parity coverage rather than downgrading it to representative spot checks.
- **Why:** several `clippy` parity tests are intentionally exactness tests for generated policy. Removing repo-local imports should not silently weaken that coverage.
- **Alternatives considered:**
  - Convert parity tests to “contains a few known entries” checks — rejected because it would make future generator drift easier to miss.

### Remove the stale `arch` assertions dependency instead of exempting it
- **Chose:** delete the unused `guardrail3_domain_project_tree` dependency/import from `apps/guardrail3/crates/app/rs/families/arch/crates/assertions`.
- **Why:** the crate no longer uses it, and leaving it in place kept one spurious repo-root `RS-TEST-03` error alive.
- **Alternatives considered:**
  - Leave the dependency and suppress the report — rejected because there was no legitimate reason for the edge to remain.

## Architectural Notes
This checkpoint reinforces the current `RS-TEST` model for already-migrated families:

- runtime sidecars own scenario setup only
- sibling assertions crates own semantic result construction/proof types
- `test_support` stays generic and file-system/fixture oriented
- sidecars do not reach directly into app-local/domain-local crates once the family is embedded back into the repo-root validation lane

The repo-root lane matters here: a family can look locally clean while still violating repo-local crate-boundary rules once it is validated as part of the whole app. The temp-copy attack for `clippy` was run at the app-root level for exactly that reason.

## Information Sources
- Repo-root inventory:
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family test --inventory --format json`
- Family-local verification:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`
- App-root adversarial regression:
  - temp copy of `apps/guardrail3`, reintroducing `guardrail3_domain_report::Severity` into `rs_clippy_17_test_relaxations_tests/multiple_relaxations.rs`
  - repo-root `RS-TEST` correctly reports `RS-TEST-03` on that file
- Prior context:
  - `.worklogs/2026-03-29-155903-finish-rs-test-clippy-family.md`
  - `.worklogs/2026-03-29-161458-finish-rs-test-arch-family.md`

## Open Questions / Future Considerations
- The repo-root `RS-TEST` backlog is now dominated by `hooks-shared`, `hooks-rs`, `hexarch`, `code`, `garde`, and `deps`.
- `release` and `project-tree` still have unrelated dirty edits in the worktree and were intentionally excluded from this checkpoint.
- Some repo-root `RS-TEST` debt in already-migrated families is the same shape as this `clippy` cleanup: direct sidecar imports of `guardrail3_domain_report` or other repo-local crates. Those families should be handled before the heavier legacy hooks migration if the count curve still favors that order.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_01_coverage_tests/multi_root_coverage.rs` — representative sidecar now using owned assertions/test-support instead of direct repo-local imports
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_17_test_relaxations_tests/multiple_relaxations.rs` — representative `Severity` ownership fix and the file used for the repo-root regression attack
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/rs_clippy_01_coverage.rs` — owned assertions module exposing the severity type needed by its sidecars
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/Cargo.toml` — stale forbidden dependency removed from the `arch` assertions crate
- `.worklogs/2026-03-29-155903-finish-rs-test-clippy-family.md` — previous `clippy` family checkpoint
- `.worklogs/2026-03-29-161458-finish-rs-test-arch-family.md` — previous `arch` family checkpoint

## Next Steps / Continuation Plan
1. Stage only the `clippy` and incidental `arch` files plus this worklog, leaving the dirty `release` and `project-tree` files unstaged.
2. Commit this checkpoint, then rerun repo-root `RS-TEST` to refresh the next family ranking.
3. Clear the remaining repo-root import/ownership debt in already-migrated families next, in the current order:
   - `code`
   - `deps`
   - `garde`
   - `hexarch`
4. Once the already-migrated families are quiet, take the heavy structural `RS-TEST-02` migrations for:
   - `hooks-rs`
   - `hooks-shared`
5. Finish the non-family tail (`ast`, `generate`, `project-tree`, `test`) only after the family buckets are no longer dominating the repo-root inventory.
