Summary
- Cleaned `packages/rs/release/g3rs-release-ingestion` to the current internal package shape and brought it to `No findings.`
- Split the oversized ingestion implementation into a real directory module with a facade `mod.rs`, moved the owned sidecar onto `collect.rs`, and mirrored that owned shape in the assertions crate.

Decisions made
- Restored the runtime dependency on `g3rs-release-ingestion-types` instead of trying to hide the local error type behind another boundary. `run.rs` and the facade both legitimately re-export that typed error.
- Reshaped `ingest` to `ingest/mod.rs` plus `ingest/collect.rs` after the rules proved that a `foo.rs` file alongside `foo/` was the wrong shape. The sidecar moved with the owned file to `ingest/collect_tests`.
- Split the old `CrateBase` inventory into nested release and readme facts instead of waiving the large-type warning. That removed the warning at the root instead of papering over it.
- Moved semantic result assertions into `crates/assertions/src/ingest/collect.rs` and made the exported helpers directly inspect `G3CheckResult` fields so the proof surface is explicit to the test rules.
- Kept the cleanup package-local. No rule changes were needed.

Key files for context
- `packages/rs/release/g3rs-release-ingestion/Cargo.toml`
- `packages/rs/release/g3rs-release-ingestion/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/mod.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/crate_base.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/repo.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/pipeline.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/assertions/src/ingest/collect.rs`

Next steps
- Continue with the next dirty release package root after `g3rs-release-ingestion`, likely `packages/rs/release/g3rs-release-repo-root-checks` or `packages/rs/release/g3rs-release-source-checks` depending on current validate output.
- Keep the sweep package-by-package and stop only if a later release package exposes a real rule contradiction.
