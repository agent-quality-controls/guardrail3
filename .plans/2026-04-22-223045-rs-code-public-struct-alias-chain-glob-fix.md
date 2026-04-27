## Goal

Fix the remaining `g3rs-code/ast-31-public-struct-named-fields` misses where local alias chains and same-file glob imports hide inherent impls on shared-crate structs.

## Approach

1. Add red tests for:
   - `use super::Input as Alias; use Alias as Alias2; impl Alias2`
   - `use super::*; impl Input`
   - `use crate::api::*; impl Input`
2. Extend the rule to:
   - resolve later local aliases through earlier module-local bindings
   - materialize same-file struct bindings for glob imports from known module paths
3. Run the code source-checks package tests and `g3rs validate`.

## Key decisions

- Keep the fix inside the rule matcher.
  - Why: this is still local source-file name resolution for one rule.
- Build module-local struct bindings once per source file.
  - Why: glob imports need a real source of same-file names, not another string heuristic.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.worklogs/2026-04-22-*.md`
