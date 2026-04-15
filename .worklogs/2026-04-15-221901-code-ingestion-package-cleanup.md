Summary
- Cleaned `packages/rs/code/g3rs-code-ingestion` to `No findings.` The package now follows the same cleaned ingestion shape as `cargo-ingestion`: explicit unpublished manifests, `run_tests` sidecars, shared proof in `crates/assertions/src/run.rs`, and a `types` crate that only owns the ingestion error.

Decisions made
- Kept the `types` crate only for `G3RsCodeIngestionError`. Rejected the old wrapper role because reexporting `g3rs-code-types` from inside this package created a fake boundary and triggered `apparch`.
- Marked the workspace and all child crates `publish = false`. Rejected keeping the old publishable root because this is an internal ingestion package and the release burden was fake.
- Renamed `ingest_tests` to `run_tests` and rewired sidecars to `crate::run::ingest_for_*`. Rejected keeping `ingest_tests` because the tested entry points live in `run.rs`, and the old name forced the wrong owned assertions path.
- Moved final result proof into `crates/assertions/src/run.rs`. Rejected local sidecar result checks because they were exactly what the test family is meant to stop.

Key files for context
- `.plans/2026-04-15-220530-code-ingestion-package-cleanup.md`
- `packages/rs/code/g3rs-code-ingestion/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run_tests/file_tree.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/types/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/lib.rs`

Next steps
- Commit this package cleanup.
- Move to the next code-family package.
- Stop only on the next real outdated or contradictory rule.
