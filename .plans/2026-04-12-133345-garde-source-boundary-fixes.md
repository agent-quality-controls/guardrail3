# Garde Source Boundary Fixes

## Goal

Fix the remaining package-model bugs found by the garde test-attack:
- source ingestion must not include nested member crates from one workspace crawl
- source ingestion must exclude test-only root source files like `src/test.rs` and `src/tests.rs`
- config tests must pin invalid-clippy behavior end to end

## Approach

1. Add failing tests for the nested-member and bare test-file selection cases.
2. Tighten `select_ast_source_files()` to the root package `src/**` surface only.
3. Extend test-path detection to exclude root `src/test.rs` and `src/tests.rs` forms.
4. Add config pipeline/runtime tests for `Invalid` clippy state.
5. Run garde package tests and a final adversarial recheck.

## Key decisions

- Do not touch old app garde bridge code.
  - Why: the old app is inventory only, not a maintained runtime target.
- Fix selection in ingestion, not in source checks.
  - Why: source checks should trust the governed file list in their input.
- Keep the pointed-workspace package model strict.
  - Why: nested member crates are a different package invocation, not part of one garde source input.

## Files to modify

- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/select.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/source/selection.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
