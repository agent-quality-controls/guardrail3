Summary:
Fixed the `g3rs-code/ast-31-public-struct-named-fields` multi-`super` normalization gap so nested inherited impl paths like `super::super::Input` resolve to the correct public struct. Added a red-first regression with a deeper public nested-module repro, then verified the focused tests and package validate path.

Decisions made:
- Kept the normalization fix in `rs_code_ast_31_public_struct_named_fields/rule.rs` because that is where impl self types are normalized for the shared-struct check.
- Left the parser-side public-surface visitor untouched after confirming it was not the code path used by this rule.
- Preserved the existing line-attribution convention for this rule: the diagnostic is still reported on the struct definition line.

Key files for context:
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.plans/2026-04-22-224228-rs-code-source-31-multi-super-normalization-fix.md`

Next steps:
- Keep the staged commit limited to the new normalization helper and the one regression hunk.
- Leave the unrelated pre-existing test hunk in `rule_tests/shared.rs` untouched.
