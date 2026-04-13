## Summary

Built the release family into real package lanes. Release now has config, filetree, and source package surfaces with typed shared inputs, real ingestion, and working tests across all release package workspaces.

## Decisions Made

- Split old app release rules by lane instead of keeping mixed repo rules.
  - `config` owns release-plz baseline, cliff baseline, workflows, tool presence, publish integrity, binary workflow, and lane-scoped input failures.
  - `filetree` owns root license/release-plz/cliff existence, crate README existence, and lane-scoped input failures.
  - `source` owns README quality and lane-scoped input failures.
- Modeled workspace-package inheritance in shared release types and ingestion instead of re-deriving it inside individual rules.
  - This fixed `workspace = true` cases for description, license, repository, keywords, categories, version, publish, and readme.
- Kept README quality semantics aligned with the old app rule.
  - Stub threshold and heading detection are unchanged.
- Rebuilt release ingestion tests around the new aggregate lane inputs.
  - The old tests were asserting the obsolete flat config input shape.
- Added missing manifest README files for release-ingestion crates.
  - The manifests declared README paths that did not exist.

## Key Files For Context

- `.plans/2026-04-13-135302-release-family-migration.md`
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-source-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/workflow.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/deps.rs`

## Next Steps

- Run a dedicated release-family test attack against the finished package build and fix any gaps it finds.
- Decide whether release ingestion should stop computing config-only facts when only filetree or source inputs are requested.
- Clean any stale old app release bridge code only if the old app is still kept as an inventory source.
