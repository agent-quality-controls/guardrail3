# Family Ingestion Rename Plan

## Summary

Wrote the migration plan for fixing ingestion package naming and structure.
The target is one ingestion package per family, with three `ingest_for_*_checks`
entrypoints, and no package names that claim to be config-only when they are not.

## Decisions made

- Keep checks packages lane-specific.
  - `config-checks`, `ast-checks`, and `file-tree-checks` names are accurate.

- Rename ingestion packages to `g3rs-{family}-ingestion`.
  - The current `*-config-ingestion` names are stale and misleading.

- Fold `g3rs-code-ast-ingestion` back into `g3rs-code-ingestion`.
  - `code` should follow the same family-ingestion contract as the other families.

- Split the work into two phases.
  - Phase 1 is mechanical rename/fold-over work.
  - Phase 2 is real implementation of the remaining `code` config and file-tree lanes.

## Key files for context

- `.plans/2026-04-09-151200-family-ingestion-rename-migration.md`
- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md`
- `packages/rs/code/g3rs-code-ast-ingestion/`
- `packages/rs/cargo/g3rs-cargo-config-ingestion/`

## Next steps

1. Rename the existing `g3rs-*-config-ingestion` package directories to `g3rs-*-ingestion`.
2. Fold `g3rs-code-ast-ingestion` into `g3rs-code-ingestion`.
3. Update all Cargo package names, path dependencies, and facade/runtime crate names.
4. Keep behavior unchanged during the rename pass.
5. After the rename, build real `code` config ingestion for `RS-CODE-07` and `RS-CODE-12`.
