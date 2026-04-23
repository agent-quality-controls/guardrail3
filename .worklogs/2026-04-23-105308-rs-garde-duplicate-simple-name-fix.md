Summary
- Fixed garde nested validation resolution so duplicate simple type names across files no longer overwrite each other in the exact-name map.
- Added a regression proving a validated `Payload` in one file no longer causes an unrelated `Payload` in another file to trigger `RS-GARDE-SOURCE-06`.

Decisions made
- Kept the fix in `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/source_analysis/run.rs`.
- Switched exact-name resolution to reject ambiguous duplicate names instead of silently taking the last inserted state.
- Kept the regression at the source-ingestion layer so it proves the bug where the bad resolution actually happens.

Key files for context
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/source_analysis/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_ast_06_nested_validation_dive/rule.rs`

Next steps
- Commit the garde fix as the final standalone bug fix.
