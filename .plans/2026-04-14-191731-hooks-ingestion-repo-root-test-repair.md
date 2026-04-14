# Goal
Make the hooks family mechanically green after the repo-root-only scope change without weakening the family contract.

# Approach
- Keep repo-global hooks semantics in package code.
- Repair hooks ingestion tests so fixtures that expect active hook analysis create a real git repo root.
- Keep the explicit nested package test proving out-of-scope behavior for non-root workspaces.
- Replace stale hook command fixtures that still use dead CLI forms like `g3rs rs validate --staged .` with the current `g3rs validate --path ...` shape.
- Re-run hooks package tests and app tests.

# Key Decisions
- Do not relax `hooks_scope_is_active(...)` to satisfy tests. The tests are stale, not the family behavior.
- Fix fixture setup at the test boundary instead of adding special-cases to ingestion.
- Treat the new CLI shape as the only valid Rust validate command in hooks fixtures.

# Files To Modify
- packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/selection.rs
- packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/pipeline.rs
- packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/integration.rs
