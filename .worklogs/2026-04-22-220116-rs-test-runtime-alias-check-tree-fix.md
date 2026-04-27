## Summary

Fixed `g3rs-test/runtime-assertions-split` so assertions modules can no longer hide runtime orchestration behind `use runtime::{self as alias, check_test_tree}; alias::check_test_tree(...)`. The runtime alias collector now treats `self as <alias>` as a runtime root alias.

## Decisions made

- Patched the alias collector in `helpers.rs`.
  - Why: the false negative was caused by incomplete import binding, not by the violation emitter.
- Kept the fix scoped to the proven alias form.
  - Rejected: broader import-shape refactors without another red case.

## Key files for context

- `.plans/2026-04-22-220116-rs-test-runtime-alias-check-tree-fix.md`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs`

## Next steps

- Fix the follow-up `g3rs-code/ast-31-public-struct-named-fields` false negative for `impl crate::...` / `impl self::...` inherent impls on nested public structs.
- Keep the parallel attack queue running on `rs/release` and `rs/hooks`.
