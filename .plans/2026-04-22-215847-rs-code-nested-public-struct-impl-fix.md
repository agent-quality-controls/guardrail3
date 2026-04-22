## Goal

Fix `RS-CODE-SOURCE-31` so shared-crate nested public structs with inherent impls still trigger the public-field rule instead of being skipped as plain data structs.

## Approach

- Add a red shared-crate test with a nested public module containing:
  - a public named-field struct
  - an inherent impl on that struct
- Bind public struct field-bag facts with qualified type identity instead of only a bare struct name.
- Update the rule's inherent-impl lookup to recurse through nested modules and compare full qualified paths.
- Re-run the full `rs/code` package tests and `g3rs validate`.

## Key decisions

- Fix the identity mismatch by carrying qualified names through the field-bag fact.
  - Rejected: matching only the last path segment, which would still confuse duplicate names in different modules.
- Keep the change inside the `rs/code` source-checks package.
  - Rejected: pushing this into a broader cross-family type system change.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/types.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.worklogs/<timestamp>-rs-code-nested-public-struct-impl-fix.md`
