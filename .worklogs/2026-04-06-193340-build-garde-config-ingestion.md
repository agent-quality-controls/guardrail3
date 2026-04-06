# Build g3rs-garde-config-ingestion

**Date:** 2026-04-06 19:33
**Scope:** packages/g3rs-garde-config-ingestion/ + shared marker on g3rs-garde-config-checks

## Summary
Built garde config ingestion package. Takes a workspace crawl, selects Cargo.toml (required) and clippy.toml/.clippy.toml (optional), parses both, returns a result struct with dependency check input (always) and clippy ban checks input (when clippy config exists). 7 tests. Verified on 8 real workspaces.

## Key decisions
- Cargo.toml is required — ingestion fails if missing
- Clippy config is optional — missing or malformed produces None, not error
- Malformed clippy.toml is treated same as absent (None, not error) because clippy config is optional for garde checks

## Key Files
- `packages/g3rs-garde-config-ingestion/crates/runtime/src/run.rs` — public `ingest()` entry point
- `packages/g3rs-garde-config-ingestion/crates/types/src/result.rs` — `G3RsGardeConfigIngestionResult` with dependency + optional clippy_bans
- `packages/g3rs-garde-config-ingestion/crates/runtime/src/ingest_tests/basic.rs` — 7 tests

## Next Steps
- g3rs-deps-config-ingestion (complex — needs multiple Cargo.toml files + guardrail3.toml)
- g3rs-garde-ast-ingestion (needs .rs source files, not config)
