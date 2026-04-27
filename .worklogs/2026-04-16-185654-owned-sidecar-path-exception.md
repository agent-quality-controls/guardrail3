Summary
- Fixed the stale test-sidecar path contradiction by making the `#[path]` exception exact instead of broad.
- The allowed shape is now only `x.rs` plus `#[cfg(test)] #[path = "x_tests/mod.rs"] mod x_tests;`. Generic `mod tests;` and other mismatches are no longer silently accepted.

Decisions made
- Kept `#[path]` forbidden in general.
  - Why: arbitrary path redirection still bypasses normal module structure.
  - Rejected: broadly allowing `#[path]` for any test module.
- Allowed only the exact file-owned sidecar pattern.
  - Why: flat file modules need a narrow bridge to reach `x_tests/mod.rs`, and this is the chosen repo shape.
  - Rejected: `mod tests;` with `#[path = "rule_tests/mod.rs"]`, because ownership is hidden in the path string instead of the module name.
- Tightened `g3rs-test/owned-sidecar-shape` to derive the expected sidecar from the source file name.
  - Why: the old rule wrongly treated any `*_tests` name as acceptable for `lib.rs`, which let the wrong module name look valid.
  - Rejected: keeping the old "any *_tests is fine" logic.

Key files for context
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr_tests/cases.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/tests/mod.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/false_positives.rs`

Next steps
- Continue package cleanup from the next failing package.
- Toolchain config and filetree packages now fail only on normal package debt, not on this rule contradiction.
