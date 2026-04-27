## Goal

Fix `g3rs-code/ast-31-public-struct-named-fields` so nested-module `use` bindings for a public struct are treated as inherent impls of the underlying type in shared crates.

## Approach

- Add a regression in the shared-struct test sidecar that proves `use super::Input as Alias; impl Alias` is currently missed.
- Broaden coverage to the direct imported-name form `use super::Input; impl Input` because it is the same rebinding class.
- Normalize local `use` bindings in `rs_code_ast_31_public_struct_named_fields/rule.rs` during inherent-impl matching, at the same rule-local boundary that already handles module-relative paths.
- Keep the change limited to the rule and its shared tests.
- Verify with the runtime package tests and `g3rs validate` for the touched package.

## Key decisions

- Fix this in the rule matcher, not ingestion.
  - Why: the bug is in local inherent-impl identity reconstruction for a parsed source file.
- Treat imported names and aliases as the same rebinding problem.
  - Why: both `use super::Input; impl Input` and `use super::Input as Alias; impl Alias` are the same missed production path.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.worklogs/2026-04-22-222256-rs-code-public-struct-use-alias-fix.md`
