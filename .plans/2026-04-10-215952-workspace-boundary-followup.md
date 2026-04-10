# Goal
Close the remaining workspace-boundary bugs from the adversarial pass: `hexarch` must respect `workspace.exclude`, and `arch` source ingestion must not recurse into excluded nested crates.

# Approach
1. Add failing tests for the exact two scope bugs.
2. Fix `hexarch` member selection to honor `workspace.exclude` and fail closed on unresolved member patterns.
3. Fix `arch` source/facade recursion to stop at nested crate roots.
4. Re-run package-local tests and another adversarial pass.

# Key decisions
- Fix the discovery/recursion boundary in ingestion, not in checks.
- Prefer one broad recursion guard in `arch` over special-casing specific rule paths.
- Bring `hexarch` member resolution up to the same standard as the other workspace-rooted ingestion packages.

# Files to modify
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/selection.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
