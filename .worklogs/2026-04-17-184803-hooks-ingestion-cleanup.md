Summary
- Cleaned `packages/rs/hooks/g3rs-hooks-ingestion` to the current package shape and brought it to `No findings.`
- Replaced the old `ingest_tests` layout with owned `run_tests`, moved runtime file access behind a local fs boundary, and turned the assertions crate into the actual shared proof surface.

Decisions made
- Kept `crates/types` as the local error crate and moved hook source-input types onto `g3rs-hooks-types` instead of re-exporting them through local types.
- Rewrote `crates/assertions/src/run.rs` as the direct proof implementation instead of a thin wrapper over private helpers, because the test family rules require exported proof-bearing assertions.
- Allowed the cross-family test dependencies explicitly in `guardrail3-rs.toml` rather than weakening package dependency checks.

Key files for context
- `packages/rs/hooks/g3rs-hooks-ingestion/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run_tests/pipeline.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run_tests/selection.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/assertions/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-types/src/lib.rs`

Next steps
- Continue the hooks family sweep with the next dirty package after rechecking package-level validation state.
- Keep the rule bar fixed unless the next package exposes a real contradiction.
