# Code AST Ingestion Test Fixes

## Goal
Add the missing end-to-end ingestion coverage for the code AST lane without changing production code.

## Approach
1. Extend `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` with the missing pipeline cases for trait-surface boundaries, public error forms, include-str traversal, and exact dispatch/line-count boundaries.
2. Extend `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/basic.rs` with profile-resolution cases for custom library paths, explicit binary paths, and nested workspace member ownership.
3. Keep the edits inside `ingest_tests/**` only and reuse the existing `write`, `git_init`, and assertion helpers.

## Key Decisions
- Keep the extracted rule package untouched.
- Prefer focused tests over new helpers unless a repeated pattern makes a helper clearly simpler.
- Verify with the code AST ingestion test suite after the patch.

## Files To Modify
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
