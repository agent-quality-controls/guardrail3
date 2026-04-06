# Build g3rs-cargo-config-ingestion

**Date:** 2026-04-06 15:40
**Scope:** `packages/g3rs-cargo-config-ingestion/**` + shared metadata on 3 facade packages

## Summary
Built the first ingestion package. Takes a `G3RsWorkspaceCrawl`, selects the root `Cargo.toml`, parses it with `cargo-toml-parser`, and returns `G3RsCargoConfigChecksInput` for the checks package. Zero arch/code violations.

## Decisions Made

### Depend on facade crates, not internal types crates
- **Chose:** Runtime depends on `g3rs-workspace-crawl`, `g3rs-cargo-config-checks`, and `cargo-toml-parser` (all facades) instead of their internal `crates/types` subcrates.
- **Why:** RS-ARCH-05 forbids cross-boundary dependencies on internal crates. Using facades avoids coupling to another package's internal structure.

### Mark 3 facade packages as shared
- **Chose:** Added `[package.metadata.guardrail3] shared = true` to `cargo-toml-parser`, `g3rs-cargo-config-checks`, and `g3rs-workspace-crawl` root Cargo.tomls.
- **Why:** RS-ARCH-06 requires non-child dependency targets to be marked shared. These are genuinely shared utility/contract packages consumed by multiple packages.

### Route fs access through dedicated module
- **Chose:** Added `fs.rs` with `read_to_string()` wrapper instead of calling `std::fs` directly in `parse.rs`.
- **Why:** RS-CODE-15 requires filesystem access through a dedicated module.

## Key Files for Context
- `packages/g3rs-cargo-config-ingestion/crates/runtime/src/run.rs` — public `ingest()` entry point
- `packages/g3rs-cargo-config-ingestion/crates/runtime/src/select.rs` — root Cargo.toml selection
- `packages/g3rs-cargo-config-ingestion/crates/runtime/src/parse.rs` — file read + parse
- `packages/g3rs-cargo-config-ingestion/crates/runtime/src/ingest.rs` — checks input assembly
- `packages/g3rs-cargo-config-ingestion/crates/types/src/error.rs` — ingestion error type
- `packages/g3rs-cargo-config-ingestion/crates/runtime/src/ingest_tests/basic.rs` — 4 tests

## Next Steps
1. The crawl→ingestion→checks pipeline is now wired for Cargo config
2. Next ingestion package could be `g3rs-fmt-config-ingestion` or `g3rs-garde-ast-ingestion`
3. An orchestrator can now: `crawl(root)` → `ingest(&crawl)` → `check(&input)`
