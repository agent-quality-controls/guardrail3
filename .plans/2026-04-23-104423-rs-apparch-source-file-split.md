Goal
- Restore `packages/rs/apparch/g3rs-apparch-ingestion` to a green state by splitting the oversized `run/source.rs` without changing source-ingestion behavior.

Approach
- Extract the `pub use` reexport traversal from `run/source.rs` into a dedicated submodule under `run/source/`.
- Keep orchestration, module walking, visibility handling, and path resolution in `run/source.rs`.
- Preserve the existing public-item collection behavior for traits and behavior reexports.
- Re-run the apparch ingestion and source-check package tests plus `g3rs validate` for both packages.

Key decisions
- Split by responsibility, not by arbitrary line chunks.
- Keep the fix local to `rs/apparch` ingestion because the regression is a size warning introduced by recent apparch bug fixes.
- Do not change rule inputs or test inventory in this pass.

Files to modify
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source/use_reexports.rs`
