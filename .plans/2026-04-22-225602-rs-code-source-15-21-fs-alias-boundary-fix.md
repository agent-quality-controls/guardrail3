## Goal

Fix the `g3rs-code/ast-15-direct-fs-usage` and `g3rs-code/ast-21-fs-glob-import` fs alias handling bug so valid Rust forms like `use std::fs as fs2;`, `use std as s; use s::fs;`, `fs2::read_to_string(...)`, and `use fs2::*;` are all detected at the parser visitor boundary.

## Approach

- Add red regressions in the direct rule tests for:
  - `use std::fs as fs2; fn main() { fs2::read_to_string(...) }`
  - `use std as s; use s::fs;`
  - `use std::fs as fs2; use fs2::*;`
- Extend `parse/fs_visitors.rs` to track both std-root aliases and std::fs subtree aliases.
- Keep the fix in the shared visitor state so all three detection surfaces share the same alias resolution.
- Verify with the package test suite and the package validate command.

## Key decisions

- Use parser visitor state instead of rule-local string matching.
  - Why: all missed cases come from the same alias graph and the rules already consume visitor output.
- Keep one commit for the shared root cause.
  - Why: the direct import, call, and glob misses are the same boundary defect.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/direct.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_21_fs_glob_import/rule_tests/direct.rs`
