# Extract RS-CLIPPY Threshold Assertions

**Date:** 2026-03-27 20:29
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/**`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_01_max_struct_bools.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_01_max_struct_bools_tests/**`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_02_max_fn_params_bools.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_02_max_fn_params_bools_tests/**`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_08_too_many_lines_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_08_too_many_lines_threshold_tests/**`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_09_too_many_arguments_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_09_too_many_arguments_threshold_tests/**`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_10_excessive_nesting_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_10_excessive_nesting_threshold_tests/**`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_19_cognitive_complexity_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_19_cognitive_complexity_threshold_tests/**`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_20_type_complexity_threshold.rs`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_20_type_complexity_threshold_tests/**`

## Summary
Moved the threshold-rule test cluster in `clippy` onto the intended self-hosted pattern: runtime sidecars now call owned rule helpers and prove behavior through sibling assertions modules instead of reaching directly into runtime-local `test_support` and asserting `CheckResult` semantics inline. This dropped the family’s `RS-TEST` count from `631` errors to `454` total findings and cut the largest structural debt bucket (`RS-TEST-03`) from `612` to `435`.

## Context & Problem
After the workspace-baseline commit (`46011b0`), `clippy` was finally green under `RS-CLIPPY` and `RS-ARCH`, but it was still deep red under `RS-TEST`:

- `RS-TEST-03`: `612`
- `RS-TEST-16`: `16`
- `RS-TEST-02`: `1`
- `RS-TEST-01`: `1`
- `RS-TEST-18`: `1`

The easiest high-signal cluster was the threshold rules:
- `g3rs-clippy/max-struct-bools`
- `g3rs-clippy/max-fn-params-bools`
- `g3rs-clippy/type-complexity-threshold`
- `g3rs-clippy/missing-method-ban`
- `g3rs-clippy/missing-type-ban`
- `g3rs-clippy/policy-context-parseable`
- `g3rs-clippy/forbid-clippy-conf-dir-override`

These all followed the same test shape:
- build a `ProjectTree`
- collect facts/config input through the runtime-local shim
- run one rule
- assert on one missing/wrong/parse-error/golden result

That made them the best place to prove the extraction pattern before touching the messier ban/policy rules.

## Decisions Made

### Move proof into sibling assertions modules rule-by-rule
- **Chose:** Add one assertions module per threshold rule in `crates/assertions/src`, backed by a tiny shared `common.rs` for `single_result` / exact result-shape helpers.
- **Why:** `RS-TEST-16` wants owned sibling assertions modules, not inline sidecar result checking. The threshold rules are uniform enough that the modules stay real, but not bloated.
- **Alternatives considered:**
  - Keep one giant assertions file for all thresholds — rejected because `RS-TEST` wants owned module parity with the runtime rules.
  - Copy/paste the same assertion logic separately in every module — rejected because it would create needless drift and make later detector changes expensive.

### Keep scenario setup in sidecars, but move rule execution into the owner module
- **Chose:** Add `run_for_tests(tree, rel_path)` helpers to the owning runtime rule files, then update the sidecars to call those helpers and only use generic tree builders from the sibling `test_support` crate.
- **Why:** This removes the direct `super::super::super::test_support::{collected_facts, config_input}` boundary escape without forcing the sidecars to know about sibling runtime modules.
- **Alternatives considered:**
  - Keep using the runtime-local `test_support` shim directly — rejected because that is the exact `RS-TEST-03` escape we are trying to shrink.
  - Move these helpers into the sibling `test_support` crate — rejected because that would push runtime-aware semantics into the generic helper crate and likely worsen `RS-TEST-18`.

### Treat warnings as acceptable intermediate fallout if the error buckets fall first
- **Chose:** Accept the temporary shift from `0` to `23` `RS-TEST-07` warnings after the threshold extraction, because the primary goal of this slice was to remove hard boundary and semantic-ownership errors.
- **Why:** The warnings are smaller and easier to audit once the heavy structural debt is out of the way. The checkpoint still materially improves the family by removing real `RS-TEST-03` and `RS-TEST-16` violations.
- **Alternatives considered:**
  - Block the commit until `RS-TEST-07` is also flat — rejected because that would bundle proof-site tuning with the more important structural extraction work.

## Architectural Notes
This slice proves the intended shape for future `clippy` cleanup:

- `test_support` supplies only generic tree/config builders like `root_workspace_tree()` and `canonical_clippy_toml()`
- runtime rule files own the test-only execution helper (`run_for_tests`)
- sibling assertions modules own the semantic result proof
- sidecars do only scenario setup plus assertion calls

That is the same general pattern already used in the stabilized families and is the right direction for the remaining `clippy` rules.

## Information Sources
- `.worklogs/2026-03-27-202045-stabilize-rs-clippy-workspace.md` — baseline workspace repair and first RS-TEST counts
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — current family contract and migration status
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_01_max_struct_bools.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_02_max_fn_params_bools.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_08_too_many_lines_threshold.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_09_too_many_arguments_threshold.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_10_excessive_nesting_threshold.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_19_cognitive_complexity_threshold.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_20_type_complexity_threshold.rs`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`

## Open Questions / Future Considerations
- The new `RS-TEST-07` warnings need auditing. They may indicate proof-bearing recognition gaps now that the assertions modules call through `common.rs`, or they may simply be sidecars that still have no owned assertions modules yet.
- The ban/policy rules (`04`-`08`, `12`-`20`) will be less uniform than the thresholds and may need smaller sub-clusters.
- The runtime-local `test_support` shim still exists. It is now used less, but it still needs to go away before the family can actually pass `RS-TEST`.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/common.rs` — shared result-shape helpers for the threshold cluster
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/rs_clippy_config_01_max_struct_bools.rs` — specimen threshold assertions module
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_01_max_struct_bools.rs` — specimen runtime rule with `run_for_tests`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_01_max_struct_bools_tests/golden.rs` — specimen sidecar after the extraction
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/test_support.rs` — still-present runtime shim that future slices should shrink and delete
- `.worklogs/2026-03-27-202045-stabilize-rs-clippy-workspace.md` — baseline before this extraction

## Next Steps / Continuation Plan
1. Audit the new `RS-TEST-07` warnings to determine whether they are proof-recognition gaps or just remaining un-migrated rules.
2. Apply the same pattern to the next coherent `clippy` rule cluster: the ban/reason rules (`04`-`08`, `15`, `18`, `19`, `20`) are probably the best next target because they already share result-shape patterns.
3. After another cluster or two, rerun the full family under `RS-TEST`, identify what remains beyond `RS-TEST-03`/`16`, and only then start adversarial rule review on `RS-CLIPPY` itself.
