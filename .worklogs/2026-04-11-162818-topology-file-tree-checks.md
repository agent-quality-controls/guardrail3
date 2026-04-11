Summary

Built the new workspace-local `topology` file-tree checks package and wired it to `g3rs-topology-ingestion`. Migrated the four surviving topology rules: nested workspaces, exact workspace membership, escaping member paths, and workspace-local family-file placement.

Decisions made

- Kept the new `topology` package scoped to the workspace-local legality subset only.
- Reused `g3rs-topology-types` as the public boundary instead of inventing another package-local input schema.
- Treated root `.cargo/*` and `.config/nextest.toml` files as legal root-sidecars for placement checks. The initial rule-16 implementation was too blunt and would have blocked valid extracted family configs.
- Tightened pipeline tests to assert exact files and counts, not just rule IDs.
- Added end-to-end coverage for nested workspaces that are excluded or not referenced, because the topology plan explicitly requires rule 11 to stay structural rather than membership-based.

Key files for context

- `.plans/2026-04-11-161204-topology-file-tree-checks.md`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

Next steps

- Move the remaining non-file-tree `hexarch` config/dependency rules out of the old app family.
- Keep `topology` focused on workspace-local file-tree legality only. Do not re-grow repo-global routing semantics into this package layer.
