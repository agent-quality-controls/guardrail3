Summary

Fixed two real topology filetree bugs and removed the fake config/source public contract from the topology packages. The topology family now presents an honest filetree-only package surface for the pointed-workspace model.

Decisions made

- Kept topology filetree-only.
  - Rejected keeping empty config/source types and `NotImplemented` ingestion entrypoints.
- Tightened `RS-TOPOLOGY-FILETREE-12` to treat workspace membership as exact membership of real child packages only.
  - Nested workspaces, hybrid descendants, and packages inside nested workspaces no longer suppress extra-member findings.
  - Still suppresses extra-member findings for unresolved descendants with parse/read failures so fail-closed `07` remains authoritative there.
- Broadened `RS-TOPOLOGY-FILETREE-13` escape detection.
  - Added Windows-drive and backslash-aware absolute path detection instead of only POSIX `/` and `..`.
- Updated stale end-to-end expectations that had been codifying the old underreporting behavior.

Key files for context

- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_12_declared_workspace_members_only_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/rs_topology_13_member_paths_must_not_escape_root_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/src/error.rs`

Next steps

- Audit `hexarch` with the same standard and remove any fake public lanes there too.
- If repo-global topology governance is still wanted, design it as a separate package instead of expanding this pointed-workspace topology package back toward the old app model.
