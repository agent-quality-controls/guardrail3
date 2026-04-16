Summary
- Re-cleaned `packages/rs/cargo/g3rs-cargo-ingestion` after the sidecar rule tightened.
- Moved `run_tests` off `lib.rs` and attached it directly to `run.rs`.

Decisions made
- Kept the package design unchanged because this was only stale test ownership.
- Attached `run_tests/mod.rs` to `run.rs`, which is the real runtime entry file that owns those tests.

Key files for context
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/mod.rs`

Next steps
- Commit this package cleanup.
- Continue scanning packages until the next real rule contradiction appears.
