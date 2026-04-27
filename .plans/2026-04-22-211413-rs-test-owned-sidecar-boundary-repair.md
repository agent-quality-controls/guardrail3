## Goal

Repair `g3rs-test/owned-sidecar-shape` so the file-tree check consumes ingestion-owned sidecar ownership facts instead of reconstructing sidecar ownership from the whole analyzed file bag.

## Approach

- Add a red test in the owned-sidecar rule suite proving the current rule false-fires when:
  - `component.sidecars` already contains the correct owned sidecar
  - the top-level `input.files` bag is intentionally lossy
- Refactor `rs_test_02_owned_sidecar_shape/rule.rs` to:
  - derive owned sidecar validation from `input.components[*].sidecars`
  - use component-local `source_module_names` and `sidecar_files` where needed
  - keep flat sidecar file checks and source-file `#[cfg(test)] mod ...` shape checks unchanged unless they depend on the old bag reconstruction
- Keep the fix narrow:
  - no new family lane
  - no unrelated type churn beyond what the existing prebound facts already support

## Key decisions

- Use the already existing `G3RsTestComponentFileTreeFacts.sidecars` contract instead of adding another sidecar-owned input type.
  - Rejected: inventing a new lane when the needed ingestion fact already exists.
- Treat this as a bug fix, not a broad refactor.
  - The bug is check-local rebinding from `input.files`, not the overall `rs/test` file-tree design.

## Files to modify

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule_tests/cases.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/assertions/src/rs_test_02_owned_sidecar_shape/rule.rs` if a negative assertion helper is needed
- `.worklogs/<timestamp>-rs-test-owned-sidecar-boundary-repair.md`
