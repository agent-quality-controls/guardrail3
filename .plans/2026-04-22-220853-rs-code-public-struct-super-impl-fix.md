## Goal

Fix `RS-CODE-SOURCE-31` so a nested public struct with an inherent impl written as `super::Type` is still recognized in shared crates.

## Approach

- Add a regression in the shared-struct test sidecar that proves `impl super::Input` is currently missed.
- Normalize inherent-impl self-type paths in `rs_code_ast_31_public_struct_named_fields/rule.rs` using the same module-relative resolution used elsewhere in the package.
- Keep the change local to the rule and its tests.
- Verify with the runtime package tests and `g3rs validate` for the touched package.

## Key decisions

- Fix this in the rule matcher, not in the test harness.
  - Why: the bug is in how inherent impl identity is reconstructed for a parsed source file.
- Reuse module-relative normalization instead of special-casing `super::Type`.
  - Why: `crate`, `self`, and `super` are all part of the same identity problem.

## Files to modify

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.worklogs/<timestamp>-rs-code-public-struct-super-impl-fix.md`
