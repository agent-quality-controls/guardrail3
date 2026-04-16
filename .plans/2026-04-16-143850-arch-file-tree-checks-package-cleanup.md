Goal
- Clean `packages/rs/arch/g3rs-arch-file-tree-checks` until `validate` returns `No findings.` and the package workspace tests pass.
- Remove the fake local `types` crate and keep the repo working by updating the one downstream path that still points at it.

Approach
- Normalize the workspace root:
  - mark the root and child crates unpublished
  - add the standard root policy files
  - add `guardrail3-rs.toml` with the needed allowed deps
- Remove `crates/types`:
  - switch the root facade and runtime crate to use `g3rs-arch-types` directly
  - update `packages/rs/arch/g3rs-arch-ingestion/crates/types` so it no longer points at the deleted path
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
- Delete the local `types` crate because it only re-exports `g3rs-arch-types` and creates fake arch noise.
- Keep the flat rule-file shape in runtime and use `x_tests/` sidecars, because this package is another file-tree checks workspace with one rule per file.
- Use per-rule shared assertions modules instead of one shared `common.rs` API, because the test rules want one owned proof file per sidecar.

Files to modify
- `.plans/2026-04-16-143850-arch-file-tree-checks-package-cleanup.md`
- `packages/rs/arch/g3rs-arch-file-tree-checks/Cargo.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/src/lib.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/rust-toolchain.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/rustfmt.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/clippy.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/deny.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/Cargo.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/lib.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/common.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/rs_arch_01_crate_has_facade.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/rs_arch_03_mod_rs_required.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/rs_arch_07a_structural_split.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/Cargo.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/lib.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_01_crate_has_facade.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_03_mod_rs_required.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_01_crate_has_facade_tests/*`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_03_mod_rs_required_tests/*`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split_tests/*`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/types/*`
- `packages/rs/arch/g3rs-arch-ingestion/crates/types/Cargo.toml`
- `packages/rs/arch/g3rs-arch-ingestion/crates/types/src/lib.rs`
