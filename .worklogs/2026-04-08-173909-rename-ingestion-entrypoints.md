# Rename Ingestion Entry Points

**Date:** 2026-04-08 17:39
**Scope:** `packages/rs/*/*-config-ingestion`, `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md`, `.plans/by_family/rs/deps.md`

## Summary
Renamed the current ingestion API entry points from `ingest_config` / `ingest_ast` / `ingest_file_tree` to `ingest_for_config_checks` / `ingest_for_ast_checks` / `ingest_for_file_tree_checks`. Updated tests and current plan notes to match.

## Context & Problem
The previous names sounded like “ingest config” as a noun, not “produce input for config checks.” The user wanted the API to say what the function is for, not what file kind it happens to touch.

## Decisions Made

### Rename entry points to say what they produce
- **Chose:** `ingest_for_config_checks`, `ingest_for_ast_checks`, `ingest_for_file_tree_checks`
- **Why:** the function name now points at the target checks lane instead of the source artifact
- **Alternatives considered:**
  - keep `ingest_config` / `ingest_ast` / `ingest_file_tree` — rejected because the wording is ambiguous
  - switch to plain `ingest()` now — rejected because the existing packages are still split by lane only in the return type, not by package name

## Architectural Notes
This is a naming cleanup, not a behavior change. The current `*-config-ingestion` packages still expose three entry points. The rename just makes the current transition shape read correctly until dedicated AST and file-tree ingestion packages exist.

## Information Sources
- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md`
- `.plans/by_family/rs/deps.md`
- existing ingestion runtimes under `packages/rs/*/*-config-ingestion`
- `.worklogs/2026-04-07-161057-split-ingestion-entrypoints.md`

## Open Questions / Future Considerations
- When lane-specific ingestion packages become the default, decide whether their public API should collapse to plain `ingest()`
- Old worklogs still mention the previous names and should stay that way as historical records

## Key Files for Context
- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md` — current package-pipeline wording
- `.plans/by_family/rs/deps.md` — current deps plan using the renamed API
- `packages/rs/fmt/g3rs-fmt-config-ingestion/crates/runtime/src/run.rs` — representative current ingestion runtime
- `.worklogs/2026-04-07-161057-split-ingestion-entrypoints.md` — prior decision record for the earlier naming

## Next Steps / Continuation Plan
1. Build `packages/rs/code/g3rs-code-ast-ingestion` as the first dedicated AST ingestion package.
2. Reuse the new naming there so the public entry point reads as producing AST checks input.
3. Verify the new package against the existing `g3rs-code-ast-checks` contract with package-local tests.
