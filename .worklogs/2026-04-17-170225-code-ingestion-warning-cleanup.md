Summary

Removed the last warning from `packages/rs/code/g3rs-code-ingestion` by replacing a stale `unreachable!()` unwrap in the shared assertions crate with a direct `expect(...)` after the presence assertion.

Decisions made

- Kept this as a one-line assertion cleanup. The warning was not a package-shape issue and did not justify a wider refactor.
- Rejected keeping `unreachable!()` after the explicit `assert!(waiver.is_some())` because that left the package warning-only instead of clean.

Key files for context

- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`

Next steps

- Commit this slice by itself.
- Move on to the next non-clean package root from the full sweep.
