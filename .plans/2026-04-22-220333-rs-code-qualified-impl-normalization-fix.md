## Goal

Fix the `g3rs-code/ast-31-public-struct-named-fields` follow-up false negative so nested public structs with inherent impls are still detected when the impl uses `crate::...`, `self::...`, or `super::...` qualified type paths.

## Approach

- Add red tests for nested shared-crate structs with:
  - `impl crate::api::Input`
  - `impl self::Input`
- Normalize impl self-type paths relative to the current module before comparing them to the qualified struct identity.
- Re-run the full `rs/code` package and `g3rs validate`.

## Key decisions

- Normalize impl paths in the rule.
  - Why: the rule is where inherent-impl identity is currently resolved.
  - Rejected: another partial string comparison against raw path segments.
- Cover both `crate` and `self` in tests.
  - Why: the production bug was the whole class of relative qualification, not one syntax form.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.worklogs/<timestamp>-rs-code-qualified-impl-normalization-fix.md`
