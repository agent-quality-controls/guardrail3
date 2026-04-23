Goal
- Fix the remaining release workflow ingestion bug so workflow-root `CARGO_REGISTRY_TOKEN` is captured and publish dry-run detection recognizes common wrapper forms.

Approach
- Add red regressions in `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/pipeline.rs` for workflow-root env and wrapped publish dry-run commands.
- Extend the workflow ingestion model in `packages/rs/release/g3rs-release-types/src/types.rs` and populate it in `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/workflow.rs` so workflow-level env is represented once at parse time.
- Update the repo-level release collector in `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs` to read the new workflow env facts and to normalize wrapped command forms before matching `cargo publish --dry-run`.
- Update any release config-check test fixtures that construct workflow structs directly.
- Verify with the touched package tests and `g3rs validate` for the release packages.

Key decisions
- Put YAML extraction in the workflow parser instead of duplicating workflow-root env parsing in the collector.
- Keep wrapper detection local to the release ingestion collector because it is a repo-level inventory predicate, not a public rule API.

Files to modify
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/workflow.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/pipeline.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/cases.rs`
