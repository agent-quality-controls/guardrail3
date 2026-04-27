## Summary

Fixed `g3rs-test/runtime-assertions-split` so assertions modules cannot hide runtime `check_test_tree()` behind a chained local alias. Added red tests covering `let check = rt::check_test_tree; let run = check; run()` and `let check = check_test_tree; let run = check; run()`, then extended the shared helper to resolve alias chains recursively.

## Decisions made

- Fixed the shared helper instead of duplicating alias logic in the collector.
  - Why: `assertions_call_runtime_check_test_tree(...)` is the single place that defines this orchestration prohibition.
- Consumed per-function alias facts already produced by ingestion.
  - Why: the parser already records `local_call_aliases`; the rule helper only needed recursive resolution.
- Covered both qualified-runtime and bare-import alias chains in one regression.
  - Why: they are the same missed rebinding class and should stay fixed together.

## Key files for context

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/assertions_violations.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`
- `.plans/2026-04-22-221917-rs-test-filetree-03-runtime-check-alias-fix.md`

## Next steps

- None.
