Goal
- Make `g3rs validate --path packages/rs/garde/g3rs-garde-ingestion` return `No findings.` without changing runtime behavior.

Approach
- Flatten the nested garde source harness under `run_tests/source` into a single `source.rs` module with inline tests.
- Keep the runtime checks unchanged and preserve the same result assertions through the existing shared `run` assertions crate.
- Verify the package tests and the exact `g3rs validate --path` command.

Key decisions
- Remove the nested sidecar directory instead of trying to make the validator accept a test-of-a-test harness.
- Keep the shared assertions API on `run` and only update the ingestion-side test structure.

Files to modify
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run_tests/source.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run_tests/source_tests/*` if needed during the move
- `packages/rs/garde/g3rs-garde-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/assertions/src/run_tests/mod.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/assertions/src/run_tests/source.rs`
