Goal

Fix the two real topology filetree bugs and remove the fake config/source public contract from the topology package family.

Approach

1. Add failing rule tests for:
   - `g3rs-topology/declared-workspace-members-only` wrongly accepting a member that only matches a nested workspace path
   - `g3rs-topology/member-paths-must-not-escape-root` missing non-POSIX absolute path forms
2. Fix the member-pattern logic in `support.rs` so `ExtraWorkspaceMember` is suppressed only for real child packages, not arbitrary descendant roots.
3. Broaden the escape predicate in `support.rs` so absolute path forms are classified as escapes, not ordinary extra members.
4. Remove fake config/source lane types from `g3rs-topology-types`.
5. Remove fake config/source ingestion functions and error variants from `g3rs-topology-ingestion`.
6. Update any README text and re-exports to match the honest filetree-only contract.
7. Verify with package tests and `git diff --check`.

Key decisions

- Keep topology as a filetree-only family for the pointed-workspace package model.
- Do not replace fake lanes with new stubs. Remove them.
- Treat nested workspace matches as non-authoritative for workspace membership exactness.

Files to modify

- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_12_declared_workspace_members_only_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_13_member_paths_must_not_escape_root_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-types/src/lib.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/src/error.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/src/lib.rs`
- `packages/rs/topology/g3rs-topology-ingestion/src/lib.rs`
- `packages/rs/topology/g3rs-topology-ingestion/README.md`
