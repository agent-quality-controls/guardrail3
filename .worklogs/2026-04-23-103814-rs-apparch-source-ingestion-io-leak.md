Summary
- Fixed the IO source ingestion leak by stopping private child modules from inheriting public visibility and by resolving `pub use` reexports explicitly.
- Added a regression that proves a public trait inside a private child module no longer enters `public_traits`.

Decisions made
- Kept the fix in `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`.
- Preserved trait reexports from private children through `pub use` instead of reopening private module traversal.
- Replaced the crate-local let chains in `source.rs` and `config.rs` with stable nested `if let` forms so the package compiles on `rustc 1.85`.

Key files for context
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_tests/pipeline.rs`

Next steps
- Flip the types source ingestion path to include behavior reexports from private child modules.
- Keep the existing red regression for that case and then re-run the package tests and validate command.
