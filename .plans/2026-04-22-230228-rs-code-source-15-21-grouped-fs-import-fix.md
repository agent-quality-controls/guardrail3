## Goal

Fix the remaining grouped `std::fs` import survivor for `g3rs-code/ast-15-direct-fs-usage` and `g3rs-code/ast-21-fs-glob-import`, including grouped alias-backed forms where the same alias graph causes the miss.

## Approach

- Add red regressions in the direct rule tests for:
  - `use std::fs::{self, *};`
  - `use std::fs::{*};`
  - alias-backed grouped forms that share the same parser boundary root cause.
- Extend `parse/fs_visitors.rs` so grouped `UseTree` nodes are resolved recursively for both direct import and glob detection.
- Keep the fix in the visitor boundary and avoid rule-local matching hacks.
- Verify with package tests and the package validate command.

## Key decisions

- Patch the shared visitor helpers instead of the rules.
  - Why: the bug is in how the parser visitor classifies grouped use trees.
- Cover both direct-import and glob surfaces.
  - Why: grouped `self` and grouped `*` are separate visible failures from the same alias-resolution path.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs`
