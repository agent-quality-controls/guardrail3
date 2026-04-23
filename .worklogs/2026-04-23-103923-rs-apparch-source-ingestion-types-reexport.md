Summary
- Enabled types source ingestion to report behavior reexports from private child modules via `pub use`.
- Confirmed the new regression now finds the reexported public free function in the types crate inventory.

Decisions made
- Kept the fix in `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`.
- Reused the same `pub use` resolution path added for the IO fix instead of adding a types-only traversal.
- Verified the final tree with package tests and the apparch validate command on the package crate root.

Key files for context
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_tests/pipeline.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_tests/basic.rs`

Next steps
- None.
