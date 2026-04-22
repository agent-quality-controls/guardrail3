Goal
- Restore `rs/release` config architecture so ingestion selects/parses config surfaces and config checks interpret them.
- Keep config-lane grouping by owned surface (`repo`, `crate`, `edge`, `input_failure`), but remove rule-shaped config facts from ingestion-owned types.

Approach
1. Prove the current defect with red tests.
- Add tests showing release config checks trust precomputed booleans/counts instead of the parsed config surfaces.
- Use one crate-level rule (`description_present`) and one repo-level rule (`publish_status_inventory` or similar) if needed to prove the scope.

2. Restore config-surface types.
- Update `packages/rs/release/g3rs-release-types/src/types.rs` so config inputs keep parsed documents and analysis surfaces instead of rule-shaped booleans/counts.
- Preserve only identity/binding fields that belong to the owned surface: names, rel paths, parsed documents, workflow analyses, edge bindings, input failures.

3. Move config interpretation back into config checks.
- Update release config rules to inspect parsed `Cargo.toml`, `release-plz.toml`, `cliff.toml`, workflow analyses, and bound edge data directly.
- Remove rule-facing booleans such as `description_present`, `license_present`, `repository_present`, `keywords_count`, `categories_count`, `version_valid`, `docs_rs_present`, `include_exclude_present`, `has_binstall_metadata`, workflow presence booleans, publish counts, and similar rule-shaped summaries from the config input structs.

4. Simplify ingestion to selection/parsing/binding only.
- Update `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs` to build config inputs from parsed surfaces without pre-interpreting individual rule outcomes.
- Keep edge binding and per-surface grouping, but stop deriving config-rule booleans and inventory summaries there.

5. Rewire tests and remove the previous wrong proof.
- Update config test support/builders to construct parsed config-surface inputs.
- Replace the current run-level proving test with tests that prove config checks read parsed surfaces.

Key decisions
- Keep lane grouping (`repo_checks`, `crate_checks`, `edge_checks`, `input_failure_checks`). This is surface grouping, not the `fmt` mistake.
- Do not collapse back to one whole-family bag. The defect is rule-shaped config interpretation in ingestion, not grouping by owned surface.
- Treat workflow analysis as an owned parsed surface for release config checks.

Alternatives considered
- Revert release all the way back to the old bag shape.
  - Rejected because surface grouping by repo/crate/edge is still valid and keeps rule inputs local.
- Leave release as-is because it still carries parsed docs in some structs.
  - Rejected because the active checks are driven by ingestion-owned booleans and counts rather than the parsed docs.

Files to modify
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/lib_tests/test_support.rs`
- affected release config rule files under `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/`
- affected release config rule tests
