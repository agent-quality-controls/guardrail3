## Summary

Added a regression and fix for `RS-CODE-SOURCE-31` so nested public structs are still recognized when their inherent impl uses `super::Type`. The rule now normalizes self-type paths before comparing them to the struct's qualified name.

## Decisions made

- Fixed the identity check in `rs_code_ast_31_public_struct_named_fields/rule.rs`.
  - Why: the bug is in local rebinding of inherent impl types, not in the shared test harness.
- Added the regression in the shared-crate sidecar test file.
  - Why: the miss only matters on the shared-crate path where the rule can skip all-public structs.
- Reused the rule's existing module-path normalization approach.
  - Why: `crate`, `self`, and `super` all need the same resolution model.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.plans/2026-04-22-220853-rs-code-public-struct-super-impl-fix.md`

## Next steps

- Run the package tests and `g3rs validate` for `packages/rs/code/g3rs-code-source-checks`.
