Summary
- Added the first extracted `topology` package boundary for the new workspace-local model: `g3rs-topology-types` and `g3rs-topology-ingestion`.
- Implemented real file-tree ingestion for the workspace-legality subset and left config/source ingestion as explicit stubs.

Decisions made
- Kept root `Cargo.toml` failures as ingestion errors, because the pointed workspace root itself is required input for the topology lane.
- Kept descendant `Cargo.toml` read/parse failures inside the file-tree input as typed failures, so future topology checks can fail closed without dropping the rest of the workspace.
- Matched the old topology live-root exclusions in ingestion: `tests/fixtures`, `tests/snapshots`, `target`, and `.claude/worktrees` stay out of the extracted topology surface.
- Added the current workspace-local family-file matrix directly in ingestion, including the extracted test config files, instead of waiting for checks to rediscover file ownership later.

Key files for context
- `.plans/2026-04-11-145332-topology-file-tree-ingestion.md`
- `.plans/by_family/rs/topology.md`
- `.plans/todo/checks/rs/topology.md`
- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/view.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/file_tree.rs`

Next steps
- Build `g3rs-topology-file-tree-checks` on top of this input and port the four workspace-legality rules.
- Decide whether `Cargo.toml` ownership in topology file placement should be expanded to additional extracted families like `code`, `arch`, and `hexarch`.
- Keep the unrelated `.plans/2026-04-11-144026-apparch-rule-family.md` out of topology commits unless it becomes part of the same task.
