## Summary

Fixed the grouped `std::fs` survivor for `RS-CODE-SOURCE-15` and `RS-CODE-SOURCE-21`. The parser visitor now resolves grouped `UseTree` nodes recursively, so grouped `self` imports and grouped `*` imports are detected even when they are reached through `std` aliases or `std::fs` aliases.

## Decisions made

- Fixed the shared visitor helpers in `parse/fs_visitors.rs`.
  - Why: the miss was in grouped `UseTree` traversal, not in the rule matchers.
- Added regressions for both direct and alias-backed grouped forms.
  - Why: the same root cause affects `use std::fs::{self, *};` and alias-backed forms such as `use s::fs::{self, *};`.
- Kept the change limited to the fs visitor boundary and the two direct rule test files.
  - Why: the rules already consume visitor output, so they did not need local special cases.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs`
- `.plans/2026-04-22-230228-rs-code-source-15-21-grouped-fs-import-fix.md`

## Next steps

- None for this fix.
