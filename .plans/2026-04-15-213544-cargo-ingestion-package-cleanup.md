Goal
- Clean `packages/rs/cargo/g3rs-cargo-ingestion` to `No findings.` Keep the real shared `types` crate, make publish intent explicit, move test proof into the shared assertions crate, and use the agreed sidecar shape for `run.rs`.

Approach
- Add the missing workspace-root policy files and `guardrail3-rs.toml`.
- Mark the workspace unpublished with explicit `publish = false` where this package is internal.
- Keep the real `types` crate because it owns the public ingestion error.
- Make `crates/assertions` depend on `crates/runtime` and stop importing the local `types` crate directly.
- Rename `ingest_tests` to `run_tests` because the tests are about the runtime entry points in `run.rs`.
- Move final result proof for config and filetree pipelines into shared assertions modules and remove direct `CheckResult` assertions from sidecar tests.
- Tighten weak `matches!` assertions so they prove specific payloads.

Key decisions
- Do not delete the `types` crate. It owns a real boundary.
- Do not patch the rules. The current failures are package debt.
- Reuse the cleaned `fmt-ingestion` shape as the reference for runtime tests and shared assertions.

Files to modify
- `packages/rs/cargo/g3rs-cargo-ingestion/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/guardrail3-rs.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/clippy.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/deny.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/rust-toolchain.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/rustfmt.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/assertions/src/common.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/assertions/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/mod.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
