Goal

- Make `g3rs-topology-ingestion` expose only the placement states that can exist in the new workspace-local package model.
- Prove those states with direct tests so future topology file-tree checks can trust the input contract.

Approach

- Update the public topology file-tree types to remove impossible attachment variants.
- Simplify runtime attachment classification to the actual workspace-local cases:
  - exact root ownership
  - nested under a discovered root
- Add tests for:
  - exact attachment values
  - owner normalization for `.cargo/*` and `.config/nextest.toml`
  - the parsed-manifest `None` branch
  - stricter exactness on file mappings and failures

Key decisions

- Do not preserve old repo-global attachment states here.
  - `AncestorOfRoots` and `OutsideRoots` are impossible once the package is scoped to one pointed workspace crawl.
  - Keeping them here would make the type less truthful and would hide broken assumptions.
- Keep root-level workspace files attached to the workspace root itself.
  - That is the correct workspace-local legality surface for future `RS-TOPOLOGY-16` checks.

Files to modify

- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-types/src/lib.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/file_tree.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/src/error.rs`
