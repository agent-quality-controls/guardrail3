## Goal

Repair the remaining `rs/arch` source-boundary defect by moving `#[path]` analysis for `RS-ARCH-SOURCE-09` into ingestion. `g3rs-arch-source-checks` should no longer receive raw `source_files` just to let one rule parse them locally.

## Approach

1. Add an ingestion-owned site type for `#[path]` module declarations.
2. Change `G3RsArchSourceChecksInput` to carry `path_attr_sites` instead of raw `source_files`.
3. Extend `g3rs-arch-ingestion` source collection to parse each Rust file once and emit those sites.
4. Rewrite `RS-ARCH-SOURCE-09` to consume prebound `path_attr_sites`.
5. Update rule tests and ingestion pipeline tests to the new source-lane shape.

## Key Decisions

- Drop `source_files` from the source lane entirely.
  - Why: after `RS-ARCH-SOURCE-09` moves to ingestion, no source rule needs the raw file bag.
- Keep `facade_surfaces` as-is.
  - Why: they are already an ingestion-owned source fact set and are used by the other source rules.

## Files To Modify

- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `packages/rs/arch/g3rs-arch-types/src/lib.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr_tests/*`
