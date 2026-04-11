Summary
- Hardened the workspace-local topology file-tree lane after another adversarial pass.
- Fixed the rule-12 support bug, expanded fail-closed and placement coverage, and upgraded all topology rule sidecars to exact-result assertions.

Decisions made
- Fixed the membership bug in `support.rs` instead of suppressing results in the rules. `RS-TOPOLOGY-12` now sees only real package children, while declared members that point at nested workspaces, hybrid descendants, or parse-failed descendants no longer double-fire as extra members.
- Kept `RS-TOPOLOGY-07` as a normal check result and strengthened it with exact-output tests for empty, multiple, and coexistence cases.
- Upgraded topology assertions to the same exact-result helper style already used by the stronger extracted families so topology sidecars no longer lag on parity.
- Aligned the local `nextest` placement test with the attachment shape ingestion actually emits.

Key files for context
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/assertions/src/common.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_07_required_inputs_fail_closed_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_12_declared_workspace_members_only_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_16_workspace_local_file_placement_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `.plans/2026-04-11-173110-topology-third-hardening.md`

Next steps
- No remaining blocker in the topology file-tree lane from the current migration boundary.
- Next non-file-tree migration target is still the remaining `hexarch` config/dependency slice.
