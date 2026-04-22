## Goal
Fix RS-CODE-SOURCE-31 so a same-file re-export alias stays visible through a child module glob import, e.g. `pub use self::Input as Alias; mod nested { use super::*; impl Alias { ... } }`.

## Approach
1. Add a red regression in `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs` using valid Rust that imports a re-export alias through `use super::*`.
2. Extend the public-surface binding collection so same-file re-export aliases are recorded in the module binding map before glob expansion consumes it.
3. Run the package tests and `g3rs validate` for `packages/rs/code/g3rs-code-source-checks`.
4. Write a worklog and commit the fix as a standalone bug fix.

## Key decisions
- Fix the binding source, not the rule site.
  - Reason: glob imports should see the same public surface that a child module sees; the rule should keep consuming the prebound module map.
- Keep the regression at the shared test layer.
  - Reason: this bug is about same-file public-surface binding, not a test harness quirk.

## Files to modify
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
