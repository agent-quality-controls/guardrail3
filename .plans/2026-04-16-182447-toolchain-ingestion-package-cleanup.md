Goal
- Clean `packages/rs/toolchain/g3rs-toolchain-ingestion` so full `validate` passes without findings.

Approach
- Make the workspace explicitly unpublished at the root and child crates.
- Add the missing workspace-root policy files and `guardrail3-rs.toml`.
- Move runtime tests from the old crate-root `ingest_tests` shape onto `run.rs` as owned `run_tests`.
- Add `crates/assertions/src/run.rs` and move final `CheckResult` proof there.
- Remove the stale assertions `common.rs` anchor and replace it with the standard runtime dependency shape.

Key decisions
- Keep the local `crates/types` crate.
  - Why: it owns the public ingestion error type, so it is not a fake wrapper.
- Attach the sidecar tests to `run.rs`, not `lib.rs`.
  - Why: `run.rs` owns the public ingestion entry points; `lib.rs` is only facade wiring.
- Keep direct assertions for non-CheckResult ingestion output in the sidecars.
  - Why: the test rules only require shared proof for final `CheckResult` semantics, not for plain data-structure checks.

Files to modify
- `packages/rs/toolchain/g3rs-toolchain-ingestion/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/guardrail3-rs.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/rust-toolchain.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/rustfmt.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/clippy.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/deny.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/assertions/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run_tests/*`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/types/Cargo.toml`
