Summary
- Fixed cargo ingestion so inherited string fields like `edition.workspace = true` are treated as valid workspace inheritance instead of invalid values.
- Added a regression test through the public cargo ingestion path for a member crate that inherits the workspace edition.

Decisions made
- Fixed ingestion normalization instead of rewriting package manifests, because the cargo rule already says workspace inheritance is allowed.
- Accepted only the exact valid inheritance shape `{ workspace = true }` and kept all other non-string table shapes invalid.

Key files for context
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/basic.rs`

Next steps
- Commit this bug fix separately.
- Resume `packages/rs/deps/g3rs-deps-ingestion` cleanup and stop only if another rule is clearly wrong or contradictory.
