Summary

- Fixed the topology file-tree attachment contract to match the actual workspace-local package model.
- Added direct tests for attachment values, owner normalization, exact family-file fanout, and the parsed-manifest `None` branch.

Decisions made

- Removed impossible attachment states from `g3rs-topology-types`.
  - `AncestorOfRoots` and `OutsideRoots` cannot exist once ingestion is scoped to one pointed workspace crawl.
  - Keeping them would make the public input lie about the real runtime surface.
- Kept root-level workspace files attached to the workspace root.
  - This is the correct legality surface for the extracted `RS-TOPOLOGY-16` subset.
- Tightened the tests around exactness.
  - Presence-only assertions were not enough for family-file fanout and attachment semantics.

Key files for context

- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/file_tree.rs`
- `.plans/2026-04-11-152844-topology-attachment-hardening.md`

Next steps

- Build `g3rs-topology-file-tree-checks` on top of this ingestion surface.
- Keep future topology legality tests pinned to exact attachment semantics rather than rediscovering placement from paths.
