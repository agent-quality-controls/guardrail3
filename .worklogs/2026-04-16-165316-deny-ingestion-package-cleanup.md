Summary

Cleaned `packages/rs/deny/g3rs-deny-ingestion` until workspace tests and full `validate` returned clean. The package now follows the working ingestion pattern: root policy files at the workspace root, explicit unpublished crates, `run_tests` as the owned sidecar, and shared proof in `crates/assertions/src/run.rs`.

Decisions made

- Kept the local `crates/types` crate.
  - Why: unlike the fake wrapper crates in other packages, this one owns the public deny ingestion error type.
  - Rejected: deleting it and pushing the error into runtime.
- Moved the old `ingest_tests` sidecar to `run_tests`.
  - Why: the tests are for the public ingestion entry points in `run.rs`, not for the helper assembly file `ingest.rs`.
  - Rejected: patching around the sidecar boundary errors while leaving the sidecar attached to the wrong owner.
- Added `crates/assertions/src/run.rs` and moved the final pipeline proof there.
  - Why: `pipeline.rs` and `filetree.rs` must call the package's shared assertions crate instead of importing sibling family assertion crates directly.
  - Rejected: keeping the direct config/filetree assertion imports in the sidecars.
- Removed the `unreachable!()` in `parse.rs`.
  - Why: this was just a local parse helper shape problem, not a rule bug. Returning a simple string reason keeps the control flow honest and removes the warning.

Key files for context

- `packages/rs/deny/g3rs-deny-ingestion/Cargo.toml`
- `packages/rs/deny/g3rs-deny-ingestion/guardrail3-rs.toml`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/run.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/run_tests/filetree.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/assertions/src/run.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/types/src/error.rs`

Next steps

- Commit this package slice as a standalone cleanup.
- Continue to `packages/rs/deny/g3rs-deny-types`.
- Stop only on the next real contradictory rule or false positive.
