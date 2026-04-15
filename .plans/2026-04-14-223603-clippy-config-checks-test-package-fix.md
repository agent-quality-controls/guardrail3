# Goal
Make `packages/rs/clippy/g3rs-clippy-config-checks` clean under the `test` family by using one shared assertions crate for semantic proof, one sibling `test_support` crate for generic builders, and sidecars that only arrange inputs and call runtime plus assertions.

# Approach
1. Add `crates/test_support` as a sibling workspace crate.
   - Port only the generic part of the old legacy pattern.
   - Move raw input builders and generic fixture helpers out of `crates/runtime/src/test_support.rs`.
   - Do not move semantic finding/proof helpers there.
2. Fix package wiring.
   - Add `test_support` as a workspace member.
   - Add runtime dev-dependencies on `g3rs-clippy-config-checks-assertions` and `test_support`.
   - Add assertions dependencies on runtime and `test_support`.
   - Remove runtime-local `mod test_support;`.
3. Fix the assertions crate shape.
   - Keep `common.rs` generic.
   - Remove the inline test from `common.rs`.
   - Add rule-owned assertions modules for every currently flagged sidecar.
   - For `01..08`, add `crates/assertions/src/<rule_dir>/rule.rs`.
   - For `09..21`, add `crates/assertions/src/rs_clippy_config_<id>_<name>.rs`.
4. Fix runtime sidecars.
   - Remove local `rule_tests/assertions.rs`.
   - Keep `helpers.rs` only for building input and running the owned production module.
   - Change tests to call the shared assertions crate directly.
   - Move direct `CheckResult` shape assertions out of `cases.rs` into the assertions crate.
   - Convert `#[cfg(test)] mod test_support;` to `#[cfg(test)] mod test_support { mod ...; }` only if needed by the rule, otherwise delete it entirely after the new crate is wired.
5. Re-run package tests and `guardrail3-rs validate --family test` until clean.

# Key Decisions
- Consult the old `legacy/.../test_support` crate only for the generic crate pattern. Reject any old semantic finding helpers there because current `RS-TEST-FILETREE-18` requires `test_support` to stay generic.
- Keep semantic proof in the assertions crate even when local sidecar wrappers look shorter. Internal and external tests must use the same proof functions.
- Fix the package rather than relaxing the `test` family. The validator output already points at the right package boundaries for this slice.

# Files To Modify
- `packages/rs/clippy/g3rs-clippy-config-checks/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/common.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/test_support.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/rule_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_02_max_fn_params_bools/rule_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_03_too_many_lines_threshold/rule_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_04_too_many_arguments_threshold/rule_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_05_excessive_nesting_threshold/rule_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_06_test_relaxations/rule_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_07_cognitive_complexity_threshold/rule_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_08_type_complexity_threshold/rule_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_09_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_10_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_11_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_12_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_13_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_14_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_15_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_16_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_17_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_18_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_19_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_20_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_21_*_tests/*`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/test_support/*`
