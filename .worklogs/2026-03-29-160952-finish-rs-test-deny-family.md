# Finish RS-TEST Deny Family

**Date:** 2026-03-29 16:09
**Scope:** `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/*_tests/*.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site_tests/assertions_calls.rs`

## Summary
Completed the `RS-TEST` cleanup for the `deny` family by migrating the remaining direct semantic result assertions out of runtime sidecars and into the owned sibling assertions crate. While validating the sweep, I also fixed a checker gap in `RS-TEST-07` so macro-generated `define_result_assertions!` helpers count as real proof sites, matching the family pattern already used by `deny`.

## Context & Problem
Repo-root `RS-TEST` still had a `deny` bucket after the earlier `release`, `garde`, `deps`, `code`, and `clippy` sweeps. Family-local validation for `apps/guardrail3/crates/app/rs/families/deny` showed 33 remaining errors, all `RS-TEST-16`, and they were all the same shape: runtime sidecars were still asserting on `result.id`, `result.severity`, `result.title`, `result.message`, `result.file`, and `result.inventory` directly instead of delegating semantic result proof to the sibling assertions crate.

After converting those sidecars, the family went error-clean but surfaced 33 `RS-TEST-07` warnings. Those warnings were not a `deny` family design problem; they came from the `test` checker failing to recognize the `define_result_assertions!` macro-generated proof helpers that `deny` uses pervasively. Without fixing that centrally, `deny` would stay warning-dirty and future families using the same macro pattern would keep re-triggering the same false positive.

## Decisions Made

### Move `deny` sidecars to owned assertions helpers instead of adding local exceptions
- **Chose:** Rewrite the remaining flagged `deny` sidecars to import `guardrail3_app_rs_family_deny_assertions::rs_deny_xx as assertions` and use `assertions::assert_findings(...)` with per-rule `error`, `warn`, and `info` constructors.
- **Why:** The family already had a consistent assertions surface generated via `define_result_assertions!`, so the fastest correct fix was to use that existing structure rather than inventing per-file exceptions or weakening `RS-TEST-16`.
- **Alternatives considered:**
  - Leave direct `result.*` assertions in place and suppress `RS-TEST-16` — rejected because it would explicitly violate the ownership split the family migration is enforcing.
  - Add one-off local helper functions inside each sidecar — rejected because that would still keep semantic proof inside the sidecar rather than in the owned assertions crate.

### Fix `RS-TEST-07` centrally for `define_result_assertions!`
- **Chose:** Extend the `test` family parser so `define_result_assertions!` registers `assert_findings`, `assert_no_findings`, and `assert_contains` as macro-defined proof-bearing assertions, and add a regression to `rs_test_07_real_proof_site`.
- **Why:** The warnings were a checker blind spot, not a property of `deny`. `define_result_assertions!` is a valid owned assertions pattern and should count as a real proof site the same way `define_rule_assertions!` already does.
- **Alternatives considered:**
  - Rewrite `deny` assertions to use hand-written public wrapper functions — rejected because it would contort one family to satisfy a checker gap instead of fixing the checker.
  - Ignore the warnings and commit an error-clean but warning-dirty family — rejected because the goal of this sweep is zero family-local `RS-TEST` findings where possible, not just zero errors.

## Architectural Notes
`deny` now follows the same `RS-TEST` proof split as the other stabilized families in this sweep:
- runtime sidecars own fixture setup and rule invocation
- sibling assertions crates own semantic result expectations
- the `test` family recognizes both hand-written exported proof helpers and macro-generated result-assertion helpers as valid proof sites

The checker change belongs in the `test` family because the distinction is architectural, not family-specific. `define_result_assertions!` is a reusable assertion-packaging pattern and should be treated as such by `RS-TEST-07`.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/deny/crates/assertions/src/common.rs` — existing macro-generated assertion API for the family
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site.rs` — proof-site rule logic
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs` — parser support for macro-defined proof functions
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — proof-bearing assertions catalog assembly
- `.worklogs/2026-03-29-155441-finish-rs-test-code-family.md` — prior `RS-TEST-16` migration pattern for another family
- `.worklogs/2026-03-29-155903-finish-rs-test-clippy-family.md` — prior family-local attack-pass expectations

## Open Questions / Future Considerations
- Repo-root `RS-TEST` is still not clean after `deny`; the next remaining buckets are elsewhere in the tree and should be handled family by family.
- There are unrelated dirty files in `release` and `project-tree` that were intentionally excluded from this commit. They still need their own cleanup lane.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/deny/crates/assertions/src/common.rs` — shared `deny` assertions API and `define_result_assertions!` macro
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_09_ban_baseline_complete_tests/library_profile.rs` — representative migrated multi-finding sidecar
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_25_allow_override_channel_tests/overrides.rs` — representative migrated duplicate-message sidecar
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs` — parser support for macro-defined proof-bearing assertions
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site_tests/assertions_calls.rs` — regression coverage for owned assertions proof-site detection
- `.worklogs/2026-03-29-155441-finish-rs-test-code-family.md` — prior `RS-TEST-16` cleanup specimen
- `.worklogs/2026-03-29-155903-finish-rs-test-clippy-family.md` — recent companion cleanup in the same sweep

## Next Steps / Continuation Plan
1. Re-run repo-root `RS-TEST` on `apps/guardrail3` and sort the remaining findings by family after this `deny` commit lands.
2. Take the next smallest mechanically clean family-local bucket rather than the most structurally messy one, unless the repo-root counts point to a single blocker dominating the remainder.
3. Keep staging narrowly: there are already unrelated dirty edits under `release` and `project-tree`, so future commits need explicit path selection to avoid bundling those changes by accident.
