Goal

- Fix the `g3rs-test/assertions-modules-prove` helper-wrapper gap only if proven with red tests.
- Detect sidecar-owned semantic result assertions when the sidecar reaches a local semantic helper through owned wrappers such as `self::helper(...)` and `use self::helper as run; run(...)`.

Approach

- Add focused regressions in `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_16_assertions_modules_prove/rule_tests/cases.rs` for:
  - a sidecar test calling `self::helper(...)` where `helper` owns the semantic result assertion
  - a sidecar test calling a `use self::helper as run; run(...)` alias where `helper` owns the semantic result assertion
- Run the targeted `g3rs-test/assertions-modules-prove` package tests first and confirm they fail.
- Fix the bug at the owning `g3rs-test/assertions-modules-prove` rule boundary unless the red tests prove ingestion is missing the required call/import facts.
- Prefer reusing existing local-helper alias semantics already present in adjacent `rs/test` source rules over adding a one-off special case.
- Re-run touched package tests and `g3rs validate` for the touched `rs/test` packages.

Key decisions

- Do not change ingestion unless the parser/source-analysis facts are insufficient.
- Keep the fix local to `g3rs-test/assertions-modules-prove` or shared `rs/test` source-check support if the same helper-resolution logic needs to be shared cleanly.
- Do not touch unrelated in-flight changes in the worktree.

Files to modify

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_16_assertions_modules_prove/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_16_assertions_modules_prove/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs` only if shared support is the right boundary
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis/proof_helpers.rs` only if the red tests prove the missing behavior is in source-analysis facts
