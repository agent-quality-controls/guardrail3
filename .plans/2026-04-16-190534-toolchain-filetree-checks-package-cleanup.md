Goal
- Make `packages/rs/toolchain/g3rs-toolchain-filetree-checks` pass full `validate`.

Approach
- Remove the local `crates/types` wrapper and use `g3rs-toolchain-types` directly from the root facade and runtime crate.
- Mark the workspace and child crates unpublished and add the missing root policy files plus `guardrail3-rs.toml`.
- Reshape the assertions crate into nested `.../rule.rs` modules and add `crates/assertions/src/run.rs` for the combined run proof.
- Replace `mod tests;` with the exact owned sidecar declarations:
  - `#[cfg(test)]`
  - `#[path = "x_tests/mod.rs"] // reason: owned sidecar tests for file module.`
  - `mod x_tests;`
- Remove `test_support.rs` and keep helpers inside each owned sidecar directory.

Key decisions
- Remove `crates/types`.
  - Why: it is a pure wrapper around `g3rs-toolchain-types`.
- Keep the flat runtime files (`run.rs`, `rs_toolchain_filetree_01_exists.rs`, `rs_toolchain_filetree_04_legacy_file.rs`).
  - Why: the chosen design is flat file modules plus owned `x_tests` sidecars.
- Add shared run assertions instead of keeping direct `CheckResult` shape checks in `run_tests`.
  - Why: internal and external tests should use the same proof surface.

Files to modify
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/run_tests/*`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_01_exists.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_01_exists_tests/*`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_04_legacy_file.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_04_legacy_file_tests/*`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/assertions/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/assertions/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/assertions/src/common.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/assertions/src/rs_toolchain_filetree_01_exists/*`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/assertions/src/rs_toolchain_filetree_04_legacy_file/*`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/assertions/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/guardrail3-rs.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/rust-toolchain.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/rustfmt.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/clippy.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/deny.toml`
