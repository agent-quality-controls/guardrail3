## Summary

Fixed the remaining g3rs-code/ast-31-public-struct-named-fields miss where a same-file re-export alias was invisible to a child module that imported the parent with `use super::*`. The binding map now records re-export aliases before glob-based lookup consumes the module surface, so `impl Alias { ... }` on a glob-imported re-export is caught.

## Decisions made

- Kept the fix in the binding collection path instead of adding a rule-local exception.
  - Why: the bug was in how the public surface was being prebound, not in the struct-field rule itself.
- Added a red regression that uses valid Rust with `pub use self::Input as Alias;`.
  - Why: this reproduces the concrete production-path miss exactly.
- Preserved the existing direct-struct pass and added a second alias-augmentation pass.
  - Why: glob import consumers still need the same module map shape, just with re-export names included.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `.plans/2026-04-22-224319-rs-code-public-struct-glob-reexport-alias-fix.md`

## Next steps

- None for this fix.
