## Goal

Extract the remaining `arch` rules into packages by adding the missing file-tree lane for:

- `RS-ARCH-01`
- `RS-ARCH-03`
- `RS-ARCH-07`

The end state is:

- `packages/rs/arch/g3rs-arch-file-tree-checks` exists and owns those rules
- `packages/rs/arch/g3rs-arch-ingestion::ingest_for_file_tree_checks(...)` is real
- package tests prove the lane end to end
- `arch` package READMEs no longer describe the `code` family

## Approach

1. Reuse the existing `arch` ingestion fact model instead of designing a second structural model.
   - Extend `g3rs-arch-types` so file-tree inputs can carry:
     - crate nodes
     - module-directory facts
   - Keep dependency edges in config and facade/source content in source.

2. Port the old app semantics directly for the remaining rules.
   - `RS-ARCH-01`: crate has facade entry point
   - `RS-ARCH-03`: module directory requires `mod.rs`
   - `RS-ARCH-07`: crate complexity thresholds force split
   - Do not redesign messages or thresholds unless the package model requires it.

3. Implement file-tree ingestion in `g3rs-arch-ingestion`.
   - Reuse current crate-node discovery
   - Add module-layout collection equivalent to the old app facts
   - Emit one `G3RsArchFileTreeChecksInput`

4. Add tests before or alongside the fix where a stub currently blocks coverage.
   - Rule-local tests for `01`, `03`, `07`
   - Ingestion pipeline tests proving:
     - missing facade
     - missing `mod.rs` / forbidden `foo.rs` + `foo/`
     - complexity breach

5. Fix package docs only where they are factually wrong.
   - `g3rs-arch-config-checks/README.md`
   - `g3rs-arch-ingestion/README.md`

## Key decisions

- Keep `RS-ARCH-01`, `03`, and `07` in file-tree.
  - Reason: they check structural layout and counts, not source-body semantics and not config content.

- Reuse existing crate-node semantics from the current ingestion code.
  - Reason: `has_lib_rs`, `has_main_rs`, and complexity counters already exist there.

- Port the old app module-layout collector into package ingestion with only the minimum boundary changes.
  - Reason: `RS-ARCH-03` depends on those exact facts, and this avoids silent semantic drift.

## Files to modify

- `packages/rs/arch/g3rs-arch-types/src/lib.rs`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-ingestion/README.md`
- `packages/rs/arch/g3rs-arch-config-checks/README.md`
- new package: `packages/rs/arch/g3rs-arch-file-tree-checks/**`
