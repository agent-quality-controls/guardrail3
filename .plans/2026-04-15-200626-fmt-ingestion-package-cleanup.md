# Goal
Make `packages/rs/fmt/g3rs-fmt-ingestion` validate clean under the active rules.

# Approach
- Add the missing workspace-root policy files so workspace-local families can run on this package.
- Make publish intent explicit and mirror the cleaned clippy ingestion package by marking this workspace unpublished.
- Replace the old `ingest_tests` harness with a `run_tests` harness that matches `run.rs`.
- Move final check-result proof into the shared assertions crate and keep runtime sidecars focused on setup, ingestion, and calling shared assertions helpers.

# Key decisions
- Keep the `types` crate because it owns the public ingestion error type.
- Follow the cleaned `g3rs-clippy-ingestion` package shape instead of inventing a new ingestion-specific pattern.
- Keep package-local release burden off by using explicit `publish = false` on this workspace and its child crates.

# Files to modify
- packages/rs/fmt/g3rs-fmt-ingestion/Cargo.toml
- packages/rs/fmt/g3rs-fmt-ingestion/src/lib.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/assertions/Cargo.toml
- packages/rs/fmt/g3rs-fmt-ingestion/crates/assertions/src/lib.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/assertions/src/common.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/assertions/src/run.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/Cargo.toml
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/lib.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/*
- packages/rs/fmt/g3rs-fmt-ingestion/crates/types/Cargo.toml
- packages/rs/fmt/g3rs-fmt-ingestion/guardrail3-rs.toml
- packages/rs/fmt/g3rs-fmt-ingestion/clippy.toml
- packages/rs/fmt/g3rs-fmt-ingestion/deny.toml
- packages/rs/fmt/g3rs-fmt-ingestion/rustfmt.toml
- packages/rs/fmt/g3rs-fmt-ingestion/rust-toolchain.toml
