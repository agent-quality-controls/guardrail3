Summary
- Split the workflow-only release checks out of the workspace-local release package into a new live package: `g3rs-release-repo-root-checks`.
- Fixed the release path-dependency blind spot by teaching release ingestion whether a path target is inside this workspace or outside it.

Decisions made
- Kept the moved workflow rules live under `packages/rs`, not `legacy`, because they still belong to the active design and only need repo-root ingestion later.
- Added `ingest_for_repo_root_checks` now as a stub that returns a clear not-implemented error. Rejected wiring fake repo-root behavior into the existing workspace-local release ingestion.
- Fixed the path-dependency bug in ingestion, not in the rule. The rule now gets a truthful edge shape with `InWorkspace` vs `OutsideWorkspace`.
- Changed `RS-RELEASE-CONFIG-19` behavior:
  - inside-workspace unpublished path target -> error
  - outside-workspace path target with a version -> warning
  - outside-workspace path target without a version -> error
- Stopped the old release ingestion pipeline test from expecting the moved repo-root workflow rules in the workspace-local package.

Key files for context
- `.plans/2026-04-15-115744-release-repo-root-checks-package.md`
- `.plans/2026-04-15-122705-release-path-target-classification.md`
- `packages/rs/release/g3rs-release-repo-root-checks/`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_19_no_path_deps_to_unpublishable.rs`

Next steps
- There is still a separate release contradiction: release checks are still firing on crates marked `publish = false`. That needs a later rule pass for explicit publish gating.
- The new repo-root package is not wired into `guardrail3-rs` yet. It is waiting for real repo-root ingestion.
