# Session Handoff - Code AST Test Hardening

## Summary
Added the missing rule-local coverage for the extracted `code` AST checks package and verified the package test suite passes. Also included the already-prepared code AST ingestion test hardening for profile resolution and pipeline boundary cases so the related ingestion suite stays aligned with the same rule coverage.

## Decisions made
- Kept the fix scoped to tests and test wiring.
- Ported missing parity cases from the legacy `code` suite for grouped lint lists, `#[expect]`, module/trait/impl surfaces, `garde(skip)` reason handling, `cfg_attr` nesting, extern-block suppression, include traversal, threshold boundaries, and exact-cap clean cases.
- Preserved existing extracted rule behavior instead of changing production code when the test attack exposed contract details.
- Included the ingestion boundary tests already in flight so the `code` AST lane remains end-to-end covered.

## Key files for context
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_03_item_level_allow_without_reason/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_04_item_level_allow_with_reason/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_05_garde_skip_without_comment/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_06_garde_skip_with_comment/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_08_cfg_attr_allow_inventory/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_09_too_many_effective_code_lines/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_20_extern_allow/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_23_include_bypass/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_29_large_trait_surface/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_34_generic_parameter_cap/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_36_string_dispatch_cap/rule_tests/*`
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Next steps
- Commit the hardening batch.
- If more `code` AST migration continues, attack the next remaining rule group with the same test-local pattern.
