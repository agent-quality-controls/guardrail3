Summary
- Fixed release workflow ingestion so workflow-root `env:` contributes `CARGO_REGISTRY_TOKEN` and wrapped `cargo publish --dry-run` commands are detected.
- Kept workflow-root YAML extraction in the workflow parser and wrapper matching in the release collector.

Decisions made
- Extended `G3RsReleaseWorkflowAnalysis` with workflow-level env facts instead of reparsing raw YAML in the collector.
- Accepted wrapped publish dry-run forms by normalizing shell wrappers and env prefixes in the repo inventory predicate.
- Reworked the new regression tests so the tests assert directly instead of proving through a local helper.

Key files for context
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/workflow.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/pipeline.rs`
- `.plans/2026-04-23-105934-release-workflow-ingestion-bugfix.md`

Next steps
- None.
