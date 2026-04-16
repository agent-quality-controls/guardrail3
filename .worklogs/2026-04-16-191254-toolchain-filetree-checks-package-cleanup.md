Summary
- Cleaned `packages/rs/toolchain/g3rs-toolchain-filetree-checks` to the current package shape and brought package validation to `No findings.`
- Removed the fake local `types` crate, moved sidecar proof into the shared assertions crate, and aligned flat runtime files with flat assertions files.

Decisions made
- Removed the local `crates/types` wrapper and used `g3rs-toolchain-types` directly because the local crate added no boundary value.
- Kept flat runtime files such as `run.rs` and `rs_toolchain_filetree_01_exists.rs`, so the shared assertions shape also stays flat (`run.rs`, `rs_toolchain_filetree_01_exists.rs`) instead of nested `.../rule.rs`.
- Moved combined run proof into `crates/assertions/src/run.rs` and kept rule-specific proof in shared assertions files so sidecar tests do not own result checks.
- Replaced the old local `test_support.rs` with owned sidecar helpers under each `*_tests/` folder.

Key files for context
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/guardrail3-rs.toml`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/assertions/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/assertions/src/rs_toolchain_filetree_01_exists.rs`

Next steps
- Continue package-by-package cleanup from the next toolchain package.
- Stop only on the next real rule contradiction, not on ordinary package debt.
