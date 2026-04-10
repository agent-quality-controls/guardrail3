# Summary
Closed the last two workspace-boundary test gaps from the latest adversarial pass. `arch` now has a direct config-lane exclusion test and filters out excluded non-crate path edges, and `hexarch` now has a direct invalid-exclude-pattern test pinned.

# Decisions made
- Added the missing tests first before touching ingestion code.
- In `arch`, fixed the config-lane leak at the ingestion boundary by dropping path dependency edges that do not resolve to selected crate nodes.
- In `hexarch`, the invalid exclude pattern path was already correct; the new test just locked it.

# Key files for context
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/selection.rs`
- `.plans/2026-04-10-215952-workspace-boundary-followup.md`

# Next steps
- If another boundary pass is needed, attack package-by-package and keep adding direct lane-specific negative tests instead of relying on code review alone.
