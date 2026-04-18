Goal
- Make `RS-ARCH-SOURCE-09` allow the chosen file-module test sidecar shape:
  - `x.rs`
  - `#[cfg(test)] #[path = "x_tests/mod.rs"] mod x_tests;`

Approach
- Add a direct regression test for `RS-ARCH-SOURCE-09`.
- Exempt only cfg(test) modules whose name ends with `_tests` and whose path is exactly `<module_name>/mod.rs`.
- Keep all other `#[path]` uses forbidden.
- Rewire `packages/rs/cargo/g3rs-cargo-filetree-checks` back to `run_tests/mod.rs`.

Key decisions
- Do not weaken the rule globally.
- Do not bless `mod tests;` and `run/tests/mod.rs`.
- Keep the chosen contract strict:
  - file module `x.rs`
  - sibling sidecar `x_tests/mod.rs`

Files to modify
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run/tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run/tests/cases.rs`
