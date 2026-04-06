# Build g3rs-deny-config-ingestion

**Date:** 2026-04-06 16:54
**Scope:** packages/g3rs-deny-config-ingestion/ + shared markers on 2 facade packages

## Summary
Built deny config ingestion package. Takes a workspace crawl, selects `deny.toml` or `.deny.toml`, parses with `deny-toml-parser`, returns `G3RsDenyConfigChecksInput`. Same pattern as cargo-config and clippy-config ingestion but with two possible file names.

## Key Files
- `packages/g3rs-deny-config-ingestion/crates/runtime/src/run.rs` — public `ingest()` entry point
- `packages/g3rs-deny-config-ingestion/crates/runtime/src/select.rs` — deny.toml/.deny.toml selection
- `packages/g3rs-deny-config-ingestion/crates/runtime/src/ingest_tests/basic.rs` — 7 tests

## Next Steps
- Remaining ingestion packages: deps-config, garde-config, garde-ast
- Orchestrator to wire all ingestion packages together
