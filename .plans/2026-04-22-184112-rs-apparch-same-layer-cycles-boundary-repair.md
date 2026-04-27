## Goal

Repair the remaining `rs/apparch` config boundary defect in `g3rs-apparch/same-layer-cycles` so the rule no longer reconstructs a crate index from edge payloads. Ingestion should bind the same-layer node set explicitly and the config check should consume that prebound input directly.

## Approach

1. Change the same-layer-cycles input type in `g3rs-apparch-types` so it carries:
   - a distinct crate/node set
   - edge references keyed by cargo manifest path
2. Update `g3rs-apparch-ingestion` to build those nodes and edges once when constructing `same_layer_cycles_check`.
3. Add a proving test in `g3rs-apparch-config-checks` that fails unless the rule uses the prebound node set instead of rebuilding it from edge payloads.
4. Simplify `g3rs-apparch/same-layer-cycles` to consume the prebound nodes directly.
5. Update helper fixtures and run tests to the new same-layer-cycles input shape.

## Key Decisions

- Keep this repair inside the existing same-layer-cycles lane instead of widening the whole family again.
  - Why: the defect is localized to one residual rule input shape.
- Do not touch `rs/cargo`.
  - Why: config families are allowed to pass parsed config surfaces intact; this defect is in a relation/graph lane, not a config-document lane.

## Files To Modify

- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles_tests/*`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run_tests/cases.rs`
- helper fixtures under `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/*helpers.rs` that construct `G3RsApparchSameLayerCyclesChecksInput`
