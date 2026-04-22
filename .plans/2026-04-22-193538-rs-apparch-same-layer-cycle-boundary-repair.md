Goal

Repair `rs/apparch` same-layer cycle checks so the rule no longer reconstructs crate identity from a separate crate bag. The cycle input should be self-contained enough that cycle reporting cannot be dropped by mismatched `crates` and `edges` bags.

Approach

- Add a red test in `g3rs-apparch-config-checks` proving the current rule can miss a real same-layer cycle when the first sorted node in the cycle is absent from `input.crates`.
- Change the `g3rs-apparch-types` same-layer cycle input from:
  - separate `crates`
  - separate `edges`
  to prebound same-layer cycle edges that carry both `from` and `to` crate identity.
- Update `g3rs-apparch-ingestion` config fanout to emit those prebound same-layer cycle edges directly.
- Simplify `rs_apparch_config_06_same_layer_cycles.rs` so it builds the graph from bound edges only and never reconstructs `crates_by_path`.
- Update affected helpers/tests and re-run `apparch` package tests plus `g3rs validate`.

Key decisions

- Keep the graph walk in the rule.
  - The defect is local crate-map reconstruction, not the cycle algorithm itself.
- Use prebound edges instead of a precomputed cycle list.
  - This removes the bad bag split while preserving the rule-owned cycle semantics.
- Do not widen this into a config-family surface refactor.
  - This is a relation-shaped config input, not parsed-config interpretation.

Files to modify

- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run_tests/cases.rs`
