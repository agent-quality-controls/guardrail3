# Narrow g3rs-clippy/local-policy-root Ownership

**Date:** 2026-03-27 21:54
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_13_local_policy_root_baseline.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_13_local_policy_root_baseline_tests/*`

## Summary
Narrowed `g3rs-clippy/local-policy-root` so it no longer double-reports keys already owned by `RS-CLIPPY-16` and `g3rs-clippy/avoid-breaking-exported-api`. Added a regression proving that a local `clippy.toml` with only `avoid-breaking-exported-api` and test-relaxation drift still remains `g3rs-clippy/local-policy-root`-clean and is left to the dedicated rules.

## Context & Problem
The adversarial ownership review across `RS-CLIPPY`, `RS-CARGO`, and `RS-CODE` found that `g3rs-clippy/local-policy-root` had drifted beyond its intended role. The rule is supposed to answer whether a local policy root below the validation root drops the inherited managed Clippy baseline, but the implementation also folded in:

- `avoid-breaking-exported-api`
- all test-relaxation booleans

That created duplicate reporting with:

- `RS-CLIPPY-16` for `avoid-breaking-exported-api`
- `g3rs-clippy/avoid-breaking-exported-api` for test relaxations

The family plan already describes `g3rs-clippy/local-policy-root` in terms of thresholds and ban sections, so the extra key ownership was implementation drift rather than a deliberate policy split.

## Decisions Made

### Keep `g3rs-clippy/local-policy-root` as a local-root completeness rule, not a second bool-policy rule
- **Chose:** Remove `avoid-breaking-exported-api` and the five test-relaxation booleans from the `g3rs-clippy/local-policy-root` aggregate baseline check.
- **Why:** Those keys already have dedicated rule ownership with sharper diagnostics. Leaving them inside `13` caused one local config defect to produce both an aggregate baseline error and a specific key-policy error.
- **Alternatives considered:**
  - Keep the duplication as “defense in depth” — rejected because it gives overlapping findings for one defect and blurs which rule owns those keys.
  - Remove `g3rs-clippy/local-policy-root` entirely — rejected because local policy roots still need an aggregate completeness rule for thresholds and ban sections.

### Add a regression that proves the dedicated rules own those bools
- **Chose:** Add `owned_by_specific_rules.rs` under the `g3rs-clippy/local-policy-root` sidecars.
- **Why:** The fix is only trustworthy if a local policy root with otherwise-complete baseline but wrong bools still inventories as self-contained under `13`.
- **Alternatives considered:**
  - Only adjust the old incomplete-baseline assertion text — rejected because that would not prove the ownership split.

### Remove the orphaned helper from `clippy_support`
- **Chose:** Delete `expected_bool_value(...)`.
- **Why:** Once `g3rs-clippy/local-policy-root` stopped consuming the bool helper, the nested family workspace failed under `-D warnings` because the helper became dead code.
- **Alternatives considered:**
  - Leave the helper for future reuse — rejected because dead exported helpers are exactly the kind of drift this family is trying to prevent.

## Architectural Notes
This change tightens the internal boundary inside `RS-CLIPPY`:

- `g3rs-clippy/local-policy-root` now owns “local policy root completeness” for broad managed sections.
- `RS-CLIPPY-16` owns `avoid-breaking-exported-api`.
- `g3rs-clippy/avoid-breaking-exported-api` owns test-relaxation key policy.

That keeps diagnostics more local to the actual defect and reduces overlap inside the family before the broader cross-family ownership review continues.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- `.plans/todo/checks/rs/clippy.md`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_13_local_policy_root_baseline.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_15_test_relaxations.rs`
- nested family verification run:
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml --workspace --lib`

## Open Questions / Future Considerations
- `g3rs-clippy/local-policy-root` still overlaps the exact threshold and ban-completeness rules (`02/03/04/05/20/21/22`) at an aggregate level. That may still be acceptable as a “local root replaced inherited policy” summary, but it should be reviewed explicitly rather than left as accidental drift.
- The top-level `guardrail3 rs validate ... --family clippy` command could not be rerun during this slice because the in-flight nested `deny` workspace currently causes Cargo to detect multiple workspace roots.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_13_local_policy_root_baseline.rs` — current aggregate local policy root completeness rule
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api.rs` — dedicated ownership of `avoid-breaking-exported-api`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_15_test_relaxations.rs` — dedicated ownership of test-relaxation booleans
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs` — managed-key support shared across clippy rules
- `.worklogs/2026-03-27-214025-harden-clippy-policy-context.md` — nearby clippy policy-context hardening
- `.worklogs/2026-03-27-213947-harden-rs-clippy-policy-context-and-test-relaxations.md` — prior tightening around test-relaxation ownership

## Next Steps / Continuation Plan
1. Continue the cross-family ownership audit for `RS-CLIPPY` vs `RS-CARGO` and `RS-CODE`, especially around whether `RS-CODE-16` should still exist now that Cargo owns `clippy::panic`.
2. Revisit `g3rs-clippy/local-policy-root` overlap with threshold and ban-section exact rules to decide whether it should stay aggregate or shrink further to section-presence only.
3. Once the nested `deny` workspace stops shadowing the outer Cargo workspace, rerun top-level `guardrail3 rs validate ... --family clippy --inventory --format json` to confirm the family still self-validates outside the nested workspace.
