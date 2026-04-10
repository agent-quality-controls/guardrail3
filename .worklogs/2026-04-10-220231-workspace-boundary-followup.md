# Summary
Closed the remaining workspace-scope bugs from the boundary attack. `hexarch` now respects `workspace.exclude` and fails closed on unresolved member patterns, and `arch` source ingestion no longer recurses into nested crate roots that are outside the selected workspace member set.

# Decisions made
- Fixed both issues in ingestion, not in checks.
- In `hexarch`, kept member resolution aligned with other workspace-rooted ingestion packages: validate `members`, honor `exclude`, and error on unresolved patterns.
- In `arch`, used one recursion guard that stops at any nested crate root instead of special-casing excluded paths in multiple walkers.

# Key files for context
- `.plans/2026-04-10-215952-workspace-boundary-followup.md`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/selection.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

# Next steps
- Apply the same workspace-root attack pattern to future mixed-family ingestion packages as they are extracted.
- Keep file-tree work separate; these fixes are strictly source/config ingestion boundaries.
