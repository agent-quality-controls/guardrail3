Summary
- Repaired the `rs/release` config lane so `g3rs-release-config-checks` no longer dispatches from the raw `repo`/`crates`/`edges`/`input_failures` bag.
- `g3rs-release-ingestion` now emits explicit `repo_checks`, `crate_checks`, `edge_checks`, and `input_failure_checks`, and the config package dispatches only over those lanes.

Decisions made
- Kept the existing atomic rule input types and changed only the config-lane envelope.
  - Why: the defect was bag dispatch in the check package, not the crate/repo/edge atoms.
  - Rejected: inventing wrapper structs per rule. That would have widened the change without improving the seam.
- Added a proving run test that populated only the new prebound lanes.
  - Why: this produced a real red failure before the fix instead of a compile-only break.
  - Rejected: proving the seam indirectly through type mismatch or grep-only evidence.
- Added a small run assertion helper for result-ID counts.
  - Why: the new proving test needed lane-dispatch proof across multiple rule IDs.
  - Rejected: inspecting raw `G3CheckResult` internals directly in the sidecar test.

Key files for context
- `.plans/2026-04-22-124759-rs-release-config-boundary-repair.md`
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/basic.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/lib_tests/test_support.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/assertions/src/run.rs`

Next steps
- Continue the Rust seam repair on the next bag-heavy package outside `rs/release`.
- Re-check whether any remaining release lane still leaks unnecessary bag structure once config, file-tree, and source are compared side by side.
