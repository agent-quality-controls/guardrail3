Goal
- Allow exactly one `#[path]` shape for test sidecars on flat file modules:
  - `x.rs`
  - `#[cfg(test)]`
  - `#[path = "x_tests/mod.rs"]`
  - `mod x_tests;`
- Keep every other `#[path]` use forbidden.

Approach
- Add rule tests first for the allowed shape and close nearby rejects.
- Update `g3rs-arch/no-path-attr` so it exempts only the exact owned sidecar path pattern.
- Update `g3rs-test/owned-sidecar-shape` so it recognizes the same exact owned sidecar declaration.
- Rerun the two rule workspaces, then rerun the package that exposed the contradiction.

Key decisions
- Keep `#[path]` forbidden in general.
  - Why: arbitrary path redirection still bypasses module structure.
- Allow only `mod <name>_tests;` with `#[path = "<name>_tests/mod.rs"]` under `#[cfg(test)]`.
  - Why: this is the chosen file-owned sidecar pattern for flat file modules.
- Reject `mod tests;` and reject any mismatch between module name and path.
  - Why: generic names hide ownership and make the rule less exact.

Files to modify
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr_tests/mod.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/tests/mod.rs`
