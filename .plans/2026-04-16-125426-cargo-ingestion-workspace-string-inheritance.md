Goal
- Fix cargo ingestion so inherited string fields like `edition.workspace = true` are treated as valid instead of invalid.

Approach
- Add a regression test through the public cargo ingestion path for a workspace member using `edition.workspace = true`.
- Update cargo ingestion normalization to accept the valid workspace-inheritance table shape in addition to plain string values.
- Re-run cargo ingestion tests and the failing `deps-ingestion` package validation.

Key decisions
- Fix ingestion normalization, not package manifests, because the current cargo rule message already says workspace inheritance is allowed.
- Treat only the exact valid inheritance shape as accepted: `{ workspace = true }`.

Files to modify
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest.rs`
