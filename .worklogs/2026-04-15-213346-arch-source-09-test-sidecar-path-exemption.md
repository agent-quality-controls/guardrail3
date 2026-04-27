Summary
- Fixed a real `g3rs-arch/no-path-attr` contradiction. The rule blanket-banned `#[path]`, but the chosen test shape for file modules is `x.rs` plus `#[cfg(test)] #[path = "x_tests/mod.rs"] mod x_tests;`.

Decisions made
- Added only a narrow exemption:
  - module name must end with `_tests`
  - path must be exactly `<module_name>/mod.rs`
  - module must be behind `#[cfg(test)]`
- Rejected weakening the rule for general `#[path]` usage. The bug was only the chosen test-sidecar shape.
- Added a direct regression that proves the allowed shape stays quiet and a non-test `#[path]` still fails.

Key files for context
- `.plans/2026-04-15-212716-arch-source-09-test-sidecar-path-exemption.md`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr_tests/mod.rs`

Next steps
- Keep using `x_tests/mod.rs` for file-module sidecars in package cleanup.
- Continue package-by-package cleanup and stop on the next real rule contradiction.
