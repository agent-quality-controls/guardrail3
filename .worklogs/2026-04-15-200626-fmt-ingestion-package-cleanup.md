# Summary
Cleaned `packages/rs/fmt/g3rs-fmt-ingestion` so it validates with no findings. The package now has workspace-root policy files, explicit unpublished manifests, a `run_tests` sidecar that matches `run.rs`, and a shared assertions crate that owns the final result proof.

# Decisions made
- Kept the `types` crate because it owns the public fmt ingestion error type.
- Marked the whole workspace unpublished with explicit `publish = false` instead of building release scaffolding for an internal package.
- Renamed `ingest_tests` to `run_tests` because the tests are about the runtime entry points in `run.rs`, not a missing `ingest.rs` module.
- Moved final check-result proof into `crates/assertions/src/run.rs` and removed local result-check helpers from sidecar tests.

# Key files for context
- packages/rs/fmt/g3rs-fmt-ingestion/Cargo.toml
- packages/rs/fmt/g3rs-fmt-ingestion/guardrail3-rs.toml
- packages/rs/fmt/g3rs-fmt-ingestion/crates/assertions/Cargo.toml
- packages/rs/fmt/g3rs-fmt-ingestion/crates/assertions/src/lib.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/assertions/src/run.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/lib.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/mod.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/helpers.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/basic.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/filetree.rs
- packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/pipeline.rs

# Next steps
- Move to the next package and keep cleaning package-local issues first.
- Stop only if another rule contradicts itself or blocks a package from doing the right thing.
