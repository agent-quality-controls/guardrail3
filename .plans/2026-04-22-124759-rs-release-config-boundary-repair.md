Goal
- Repair the `rs/release` config lane so `g3rs-release-config-checks` stops dispatching from the raw `repo`/`crates`/`edges`/`input_failures` bag and instead consumes ingestion-owned, prebound config-lane inputs.

Approach
- Add a proving run test in `packages/rs/release/g3rs-release-config-checks` that expects dispatch from explicit prebound config lanes rather than the current bag shape.
- Narrow `G3RsReleaseConfigChecksInput` in `packages/rs/release/g3rs-release-types/src/types.rs` to explicit lane collections:
  - repo checks
  - crate checks
  - edge checks
  - input failures
- Update `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs` to build the new lane inputs directly.
- Rewrite `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs` to pure dispatch over those lane inputs only.
- Update release config test helpers and assertions so run tests prove the new seam and do not inspect raw result internals directly.

Key decisions
- Keep the atomic rule input types already in use (`G3RsReleaseConfigRepo`, `G3RsReleaseConfigCrate`, `G3RsReleaseConfigEdge`, `G3RsReleaseInputFailure`).
  - Why: the defect is config-lane bag dispatch, not the rule-level atoms themselves.
  - Rejected: inventing new wrapper structs for each rule. That would widen the change without improving the seam.
- Keep this repair scoped to config only.
  - Why: file-tree and source lanes are separate packages and should not be widened into the same commit.
  - Rejected: refactoring all of `rs/release` at once.
- Add a dedicated run assertion helper if the new proving test needs exact result-shape checks.
  - Why: recent seam repairs already showed `g3rs validate` rejects direct `CheckResult` field inspection in sidecar tests.

Files to modify
- `.plans/2026-04-22-124759-rs-release-config-boundary-repair.md`
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/lib_tests/test_support.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/assertions/src/run.rs`
