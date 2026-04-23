Summary
- Fixed `rs/apparch` source ingestion so private child modules no longer leak public behavior items into the types public-surface lane.
- Kept the existing IO-trait behavior intact: public traits from private modules are still collected for IO crates, matching the established ingestion contract.

Decisions made
- Narrowed the fix to behavior items only. The worker's first patch gated every child module item on module visibility and broke existing IO-trait ingestion tests.
- Left module traversal in place for private modules so trait collection still works; only free functions and inherent methods stay behind the `public_module` gate.
- Added a regression in source-ingestion tests proving a private child module under a types crate does not populate `public_behavior_items`.

Key files for context
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_tests/pipeline.rs`
- `.plans/2026-04-23-095532-rs-apparch-private-module-public-surface-fix.md`

Next steps
- Finish and review the in-flight hook engine fix for helper chaining and later redefinitions.
- Attack the remaining medium-strength `rs/test` and `rs/apparch` test weaknesses after the hook batch is green.
