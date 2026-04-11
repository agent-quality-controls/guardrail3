Goal

- Move the surviving workspace-local topology rules out of the old app family into a new `g3rs-topology-file-tree-checks` package.
- Cover only the rules that still make sense in the new pointed-workspace package model.

Approach

- Scaffold `packages/rs/topology/g3rs-topology-file-tree-checks` with the normal package layout.
- Reuse `g3rs-topology-types` as the public input surface.
- Implement these four rules:
  - nested workspaces under the pointed workspace are forbidden
  - workspace members must exactly match real child crates
  - workspace member paths must not escape the workspace root
  - workspace-local family files must be legally placed
- Keep rule logic pure and local.
- Do the comparison and placement derivation once in the runtime support layer.
- Add end-to-end topology ingestion tests that run `crawl -> ingest_for_file_tree_checks -> check`.

Key decisions

- Do not migrate old repo-global topology rules.
  - Those belonged to the old app runner and are out of scope for the workspace-local package model.
- Do not create a separate public checks input crate shape here.
  - The package should use the existing `G3RsTopologyFileTreeChecksInput` from `g3rs-topology-types`.
- Keep rule 16 workspace-local.
  - It should reason only about files inside the pointed workspace and their attached root disposition.

Files to modify

- `packages/rs/topology/g3rs-topology-file-tree-checks/**`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/mod.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
