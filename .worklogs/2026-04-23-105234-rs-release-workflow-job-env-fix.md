Summary
- Fixed release workflow ingestion so job-level `env` is captured and feeds repo-root registry-token detection.
- Added a regression proving a workflow with `CARGO_REGISTRY_TOKEN` on the job, not the step, now sets the registry-token repo flag.

Decisions made
- Expanded the release types surface to carry the repo-root workflow booleans that the repo-root checks already consume.
- Kept workflow parsing in `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/workflow.rs` and repo-level rollups in `collect.rs`.
- Trimmed the repo-root helper fixtures to the current `G3RsReleaseConfigRepo` model and added a test-only `cargo-toml-parser` dependency for those fixtures.

Key files for context
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/workflow.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/pipeline.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/rs_release_repo_root_03_registry_token/rule.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/rs_release_repo_root_03_registry_token/rule_tests/helpers.rs`

Next steps
- Commit the release workflow fix as a standalone change, then commit the garde duplicate-name resolution fix separately.
