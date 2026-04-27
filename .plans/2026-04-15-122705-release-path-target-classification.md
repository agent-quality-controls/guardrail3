Goal
- Fix release path-dependency checks so they can tell "inside this workspace" from "outside this workspace".
- Make local unpublished path dependencies fail even when a version is written.
- Make outside-workspace path dependencies visible instead of silently standing down.

Approach
- Add path-target classification to release config edges.
- Update release ingestion to resolve dependency paths relative to the source crate and classify them as:
  - in workspace
  - outside workspace
  - unresolved
- Add direct failing rule tests for:
  - local unpublished path dependency with a version
  - outside-workspace path dependency with a version
- Update `g3rs-release/no-path-deps-to-unpublishable` to:
  - error for inside-workspace unpublished target
  - warn for outside-workspace path dependency with a version
  - error for outside-workspace path dependency with no version
- Update the old ingestion pipeline test to match the corrected behavior.

Key decisions
- Fix this in release ingestion, not by guessing in the rule. The rule needs a truthful edge shape.
- Keep one rule ID for this path-dependency risk instead of adding another rule right now.
- Use a small enum for target location instead of more booleans.

Files to modify
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_19_no_path_deps_to_unpublishable.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/deps.rs`
