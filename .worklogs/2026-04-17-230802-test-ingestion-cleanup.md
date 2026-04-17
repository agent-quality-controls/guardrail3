Summary
- Cleaned `packages/rs/test/g3rs-test-ingestion` to the current package shape and restored a fully validate-able ingestion boundary.
- Split the oversized runtime ingestion logic into facade-owned modules, normalized the package shell, and moved sidecar proof helpers into the assertions crate.

Decisions made
- Kept `g3rs-test-ingestion-types` as the local typed boundary for the ingestion error instead of folding it into `g3rs-test-types`, because this package owns the ingestion-specific error contract.
- Split `components.rs` into `collect`, `facts`, `classify`, and `support` modules to fix the size violation at the source instead of waiving the rule.
- Reshaped runtime ingestion to `ingest/mod.rs` facade plus `ingest/run.rs` owner module with `run_tests/`, because the rules require a leaf owner module for owned sidecars and a facade-only `mod.rs`.
- Moved shared result assertions to `crates/assertions/src/ingest/run.rs` so sidecars stop owning semantic `G3CheckResult` checks directly.

Key files for context
- `packages/rs/test/g3rs-test-ingestion/Cargo.toml`
- `packages/rs/test/g3rs-test-ingestion/guardrail3-rs.toml`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/mod.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components/mod.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/assertions/src/ingest/run.rs`

Next steps
- Clean `packages/rs/test/g3rs-test-file-tree-checks`.
- Then clean `packages/rs/test/g3rs-test-source-checks`.
- Finish the remaining `topology` packages and rerun the full validate sweep.
