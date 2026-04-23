Summary

- Fixed the `RS-TEST-SOURCE-16` helper-wrapper gap so sidecars that reach semantic result assertions through local owned wrappers are still reported.
- Added red regressions for direct `self::helper(...)`, `use self::helper as run; run(...)`, and wrapper-function variants before changing the rule.

Decisions made

- Kept the fix in `RS-TEST-SOURCE-16` rule logic instead of changing ingestion.
  - Why: the parser already exposed the required `call_paths`, `imports`, and `local_call_aliases`; the miss was incomplete local-helper traversal in the rule.
- Expanded both surfaces that matter:
  - direct test-call detection in `owns_sidecar_semantic_proof(...)`
  - recursive helper promotion in `local_semantic_helper_names(...)`
  Why: fixing only one surface would still miss either direct qualified/aliased helper calls or wrapper functions that hide the semantic assertion one level deeper.
- Reused the same local-import alias semantics already used in adjacent `rs/test` source rules instead of inventing a narrower one-off branch.

Key files for context

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_16_assertions_modules_prove/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_16_assertions_modules_prove/rule_tests/cases.rs`
- `.plans/2026-04-23-101157-rs-test-source-16-helper-wrapper-fix.md`

Next steps

- None for this bug fix.
