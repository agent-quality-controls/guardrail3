Summary
- Fixed the garde ingestion validate failure by flattening the nested source harness into a single `run_tests/source.rs` module and removing the orphaned sidecar directory.
- Kept the runtime ingestion behavior unchanged and preserved the source-check assertions through the existing shared `run` assertions API.

Decisions made
- Rejected the nested sidecar directory shape because the validator treated it as orphaned.
- Kept the runtime-side shared assertions path on `run` and made the new `run_tests/source.rs` module self-contained with inline tests.

Key files for context
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run_tests/source.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/assertions/src/run_tests/source.rs`

Next steps
- None.
