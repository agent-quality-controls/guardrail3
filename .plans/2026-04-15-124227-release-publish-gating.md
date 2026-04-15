Goal
- Make the release family follow the agreed publish policy:
  - every crate must declare `publish`
  - only crates with explicit `publish = true` carry release burden
  - workspace-root release files only matter when the workspace has at least one published crate

Approach
- Add direct tests that prove the current bugs:
  - missing `publish` must be reported directly
  - unpublished source crates must not trigger release dependency rules
  - workspaces with no published crates must not trigger workspace-root release file checks
- Add the missing release fact fields:
  - crate-level `publish_declared`
  - edge-level `source_publishable`
  - filetree repo `publishable_count`
- Change release ingestion so missing `publish` does not silently act like `publish = true`
- Add a new config rule for explicit `publish`
- Gate release dependency rules and workspace-root release rules on actual publish intent
- Prove the full behavior with one end-to-end release ingestion test

Key decisions
- Treat missing `publish` as "not release-eligible", not as publishable.
  - This matches the agreed policy better than Cargo's default.
- Keep the explicit publish check as a separate rule instead of folding it into other rules.
  - That keeps the failure specific: one clear error, not many fake release errors.
- Gate repo-level release setup checks when `publishable_count == 0`.
  - A workspace with nothing published should not need `LICENSE`, `release-plz.toml`, or `cliff.toml`.

Files to modify
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/test_support.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_19_no_path_deps_to_unpublishable.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_20_interdependent_version_consistency.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/test_support.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/rs_release_filetree_01_license_file.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/rs_release_filetree_02_release_plz_exists.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/rs_release_filetree_03_cliff_exists.rs`
