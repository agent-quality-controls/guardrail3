## Summary

Fixed `RS-TEST-FILETREE-03` so assertions modules cannot hide runtime `check_test_tree()` behind a local alias. Added a red test covering both `let run = rt::check_test_tree; run()` and `let run_direct = check_test_tree; run_direct()`, then extended the shared helper to consume per-function `local_call_aliases`.

## Decisions made

- Fixed the shared helper instead of duplicating alias logic in the collector.
  - Why: `assertions_call_runtime_check_test_tree(...)` is the single place that defines this orchestration prohibition.
- Consumed per-function alias facts already produced by ingestion.
  - Why: the parser already records `local_call_aliases`; the rule helper was simply ignoring them.
- Covered both imported-runtime and bare-import alias forms in one regression.
  - Why: they are the same missed alias-rebinding class and should stay fixed together.

## Key files for context

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/assertions_violations.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
- `.plans/2026-04-22-221834-rs-test-runtime-check-tree-alias-fix.md`

## Next steps

- Keep attacking `rs/test` for remaining cases where rules ignore parsed alias or rebinding facts already available from ingestion.
- Prefer fixing shared helpers when multiple rule paths depend on the same semantic detector.
