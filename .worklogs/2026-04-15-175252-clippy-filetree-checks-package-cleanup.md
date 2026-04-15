Summary

Cleaned `packages/rs/clippy/g3rs-clippy-filetree-checks` so the package validates with no findings under the active rules. The main work was removing the dead `types` wrapper crate, adding the missing workspace-root policy files, and reshaping the runtime/assertions/test-support layout to the current shared-test pattern.

Decisions made

- Deleted the local `types` crate instead of keeping a thin reexport wrapper. The package now depends on `g3rs-clippy-types` directly.
- Rebuilt the runtime rules into nested `mod.rs` + `rule.rs` + `rule_tests/` layout so the package obeys the active arch and test rules without `#[path]`.
- Added a sibling `crates/test_support` crate and moved generic test input building there. Shared result checks now live only in the assertions crate.
- Marked the workspace and child crates unpublished with explicit `publish = false`.
- Ran an adversarial test pass on the touched filetree tests. No rule bug or missing coverage gap showed up in that package.

Key files for context

- `packages/rs/clippy/g3rs-clippy-filetree-checks/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/guardrail3-rs.toml`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/crates/runtime/src/rs_clippy_filetree_01_coverage_exists/rule.rs`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/crates/runtime/src/rs_clippy_filetree_02_same_root_conflict/rule.rs`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/crates/runtime/src/run/rule.rs`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/crates/assertions/src/run/rule.rs`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/crates/test_support/src/input.rs`

Next steps

- Move to the next clippy package and keep the same loop:
  - run full validation
  - fix clearly valid package problems
  - stop only on a real rule contradiction or unclear rule
- Keep running adversarial test passes before each commit so package-clean status is not based on weak tests.
