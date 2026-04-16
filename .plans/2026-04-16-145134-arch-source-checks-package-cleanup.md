Goal
- Clean `packages/rs/arch/g3rs-arch-source-checks` until `validate` returns `No findings.` and the package workspace tests pass.
- Remove the fake local `types` crate and the stray empty `rs_code_ast_*` runtime directories, and keep the repo working by updating the one downstream path that still points at the deleted `types` crate.

Approach
- Normalize the workspace root:
  - mark the root and child crates unpublished
  - add the standard root policy files
  - add `guardrail3-rs.toml` with the needed allowed deps and a structural waiver for the real runtime layout
- Remove `crates/types`:
  - switch the root facade and runtime crate to use `g3rs-arch-types` directly
  - update `packages/rs/arch/g3rs-arch-ingestion/crates/types` so it no longer points at the deleted path
- Delete the empty `rs_code_ast_*` and `rule_tests` directories under runtime `src/`
- Reshape the assertions crate:
  - add a feature contract
  - replace the public field bag helper with per-rule shared assertions modules
  - make `src/lib.rs` a gated facade
- Reshape runtime tests:
  - remove `test_support.rs` from `lib.rs`
  - move each rule to the current `x_tests/mod.rs` sidecar shape
  - split test bodies into `cases.rs` and `helpers.rs`
  - move final proof into the shared assertions crate
- Re-run package tests and `validate`, then record the cleanup in a worklog and commit.

Key decisions
- Delete the local `types` crate because it only re-exports `g3rs-arch-types` and creates fake arch and apparch coupling.
- Delete the empty `rs_code_ast_*` directories because they are dead junk, not an intentional rule layout.
- Keep the flat rule-file layout in runtime and use `x_tests/` sidecars, because this package is another source-check workspace with one rule per file.
- Use per-rule shared assertions modules instead of one shared `common.rs` API, because the test rules want one owned proof file per sidecar.

Files to modify
- `.plans/2026-04-16-145134-arch-source-checks-package-cleanup.md`
- `packages/rs/arch/g3rs-arch-source-checks/Cargo.toml`
- `packages/rs/arch/g3rs-arch-source-checks/src/lib.rs`
- `packages/rs/arch/g3rs-arch-source-checks/guardrail3-rs.toml`
- `packages/rs/arch/g3rs-arch-source-checks/rust-toolchain.toml`
- `packages/rs/arch/g3rs-arch-source-checks/rustfmt.toml`
- `packages/rs/arch/g3rs-arch-source-checks/clippy.toml`
- `packages/rs/arch/g3rs-arch-source-checks/deny.toml`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/Cargo.toml`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/lib.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/common.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/rs_arch_08a_feature_gated_exports.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/Cargo.toml`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_08a_feature_gated_exports.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_08a_feature_gated_exports_tests/*`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr_tests/*`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/test_support.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_code_ast_*`
- `packages/rs/arch/g3rs-arch-source-checks/crates/types/*`
- `packages/rs/arch/g3rs-arch-ingestion/crates/types/Cargo.toml`
- `packages/rs/arch/g3rs-arch-ingestion/crates/types/src/lib.rs`
