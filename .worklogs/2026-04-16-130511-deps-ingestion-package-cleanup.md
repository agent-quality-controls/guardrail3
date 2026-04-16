Summary
- Cleaned `packages/rs/deps/g3rs-deps-ingestion` to the current package shape so `validate` returns `No findings.` and the package workspace tests pass.
- Finished the `ingest_tests` to `run_tests` move, moved final proof into the shared assertions crate, and made publish intent explicit across the workspace.

Decisions made
- Kept `crates/types` because it owns the real public ingestion error boundary instead of acting as a useless wrapper.
- Moved all `CheckResult` proof into `crates/assertions/src/run.rs` so runtime sidecars only set up fixtures and call shared proof helpers.
- Left the package unpublished by setting `publish = false` on the root and child crates, because this workspace is internal.
- Fixed the direct filesystem access in `run.rs` at the fs boundary instead of suppressing the rule locally.

Key files for context
- `packages/rs/deps/g3rs-deps-ingestion/Cargo.toml`
- `packages/rs/deps/g3rs-deps-ingestion/guardrail3-rs.toml`
- `packages/rs/deps/g3rs-deps-ingestion/crates/assertions/src/run.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run_tests/filetree.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run_tests/pipeline.rs`

Next steps
- Continue package cleanup from the next Rust package and stop only on the next real rule bug or contradiction.
- Reuse this package as the reference shape for other remaining `*-ingestion` workspaces with real local `types` crates.
