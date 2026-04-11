Goal

Close the remaining topology issues from the latest adversarial review:

- `RS-TOPOLOGY-12` must not fire for nested workspaces or descendant input failures
- `RS-TOPOLOGY-07` needs stronger unit and pipeline coverage
- `RS-TOPOLOGY-16` needs more branch and family coverage, including member-root nested placement
- rerun adversarial review until no concrete blocker remains

Approach

- Add failing unit tests first for:
  - nested workspace does not also trigger `RS-TOPOLOGY-12`
  - descendant input failure does not also trigger `RS-TOPOLOGY-12`
  - empty/multiple/coexisting `RS-TOPOLOGY-07`
  - exact-root illegal placement branches for `RS-TOPOLOGY-16`
  - nested-under-member-root placement for `RS-TOPOLOGY-16`
- Add failing ingestion pipeline tests for:
  - unreadable descendant fail-closed
  - stale-read descendant fail-closed
  - member-root nested placement
- Fix `collect_actual_children()` to scope membership exactness to real package children only.
- Keep package-level semantics workspace-local. Do not widen the package boundary to old repo-global topology.

Files to modify

- `.plans/2026-04-11-173110-topology-third-hardening.md`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/test_support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_07_required_inputs_fail_closed_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_11_no_nested_workspaces_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_12_declared_workspace_members_only_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_16_workspace_local_file_placement_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
