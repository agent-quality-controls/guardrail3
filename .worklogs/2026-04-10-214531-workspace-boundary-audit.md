# Summary
Audited the extracted ingestion packages for workspace-boundary widening. The real leaks were in `arch` and `hexarch`: both could widen discovery beyond the pointed workspace, and both had fallback reads for paths missing from the crawl. Those boundaries are now fixed and covered by direct workspace-root tests.

# Decisions made
- Scoped the audit to ingestion packages, because checks packages consume typed inputs and do not perform discovery.
- Fixed the root cause in ingestion, not in checks: `arch` and `hexarch` now use the pointed root workspace manifest instead of scanning the whole crawl or repo-shaped `apps/` trees.
- Removed fallback file reads for `arch` and `hexarch`; selected files must exist in the crawl.
- Kept the rest of the ingestion families unchanged after audit because they were already rooted at `crawl.root_file("Cargo.toml")` or selected entries within the pointed workspace.

# Key files for context
- `.plans/2026-04-10-213606-workspace-boundary-audit.md`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/view.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/view.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/mod.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/selection.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/reachability.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/source_layout.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

# Next steps
- Run the same workspace-boundary audit against future mixed families as they are extracted.
- Build the remaining non-file-tree lanes before coming back to structural checks.
