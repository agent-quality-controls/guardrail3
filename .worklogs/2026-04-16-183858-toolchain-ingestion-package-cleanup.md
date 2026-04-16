Summary
- Cleaned `packages/rs/toolchain/g3rs-toolchain-ingestion` to the current package contract and made it pass both package tests and full `validate`.
- The package now uses `run_tests` sidecars under `run.rs`, shared ingestion assertions in `crates/assertions/src/run.rs`, explicit unpublished manifests, and the standard root policy files.

Decisions made
- Kept `crates/types` because it owns the real public ingestion error type. Removing it would have collapsed a real boundary, not dead wrapper code.
- Moved shared proof into the assertions crate and kept runtime sidecars for setup and execution only. This matches the current test rules and keeps proof logic shared.
- Re-exported ingest entry points into `run_tests/mod.rs` for child test files. That fixed the sidecar ownership move without scattering crate-root imports through every test file.
- Removed the file-level `#![allow(...)]` from `run_tests/mod.rs`. It was causing source parsing failures during validation and was not needed.
- Changed the nightly-channel shared assertion to `assert_contains(...)` instead of exact equality. The config rule also emits expected inventory infos for `clippy` and `rustfmt`, so exact matching was the wrong proof for this test.
- Kept the real-workspace sweep tolerant of `stable` or pinned stable versions. That matches the current toolchain rule instead of baking a stale exact string into the test.

Key files for context
- `packages/rs/toolchain/g3rs-toolchain-ingestion/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/guardrail3-rs.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run_tests/filetree.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run_tests/real_workspaces.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/assertions/src/run.rs`

Next steps
- Continue package-by-package cleanup from the next failing Rust package.
- Stop only if the next blocker is a real rule contradiction instead of normal package debt.
