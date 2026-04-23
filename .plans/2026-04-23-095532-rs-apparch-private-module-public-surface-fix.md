Goal
- Ensure `rs/apparch` source ingestion excludes behavior items that are only reachable through private child modules, while preserving public behavior inventory from actually public module paths.

Approach
- Add a regression in the ingestion pipeline tests that builds a public root with a private `mod internal;` child containing public behavior and proves it does not appear in `public_behavior_items`.
- Add or update the source-check rule fixture coverage only if needed to lock the user-visible behavior against regressions.
- Fix module traversal in `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs` so nested module traversal carries `current_public_context && child_module_is_public` instead of reusing the parent visibility unchanged.
- Re-run the touched package tests and `g3rs validate` for the affected `rs/apparch` packages.

Key decisions
- Fix the bug in ingestion rather than in the source rule. The bad inventory is produced before the rule runs, so rule-side filtering would be a band-aid.
- Keep the visibility model local to module traversal. The traversal already owns module discovery and file walking, so this is the narrow architectural boundary for public-surface semantics.
- Use a red regression first in the pipeline test because it exercises the end-to-end inventory feeding `public_behavior_items`.

Files to modify
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_tests/pipeline.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/rs_apparch_source_05_types_public_surface_tests/cases.rs`
