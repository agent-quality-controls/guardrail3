## Summary

Fixed `g3rs-code/ast-31-public-struct-named-fields` so shared-crate nested public structs with inherent impls are no longer skipped. The rule now matches inherent impls by qualified type path instead of a bare struct name from the file root.

## Decisions made

- Added qualified struct identity to `PublicStructFieldBagInfo`.
  - Why: the rule needs the real nested type path, not only the display name.
- Kept the impl search inside the rule, but made it recurse through nested modules and compare full paths.
  - Why: this closes the false negative without pushing more rule semantics into unrelated parser helpers.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/types.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`

## Next steps

- Fix the newly confirmed `g3rs-test/runtime-assertions-split` nested runtime-alias false negative.
- Then fix the `hook_shared_13` false positive for loop prefixes like `time while ...`.
