## Goal

Remove the last redundant lookup dependency from `rs/arch` config checks. `g3rs-arch/shared-flag-required` should consume only the dependency-edge fact it actually uses, instead of requiring `run.rs` and the rule tests to rebuild a crate map.

## Approach

1. Remove the crate-map parameter from `g3rs-arch/shared-flag-required`.
2. Simplify `g3rs-arch-config-checks` `run.rs` so it dispatches `g3rs-arch/shared-flag-required` directly on each edge.
3. Rewrite the rule test helper to pass only an edge.
4. Run the `rs/arch` config package tests and validator to prove nothing else depended on the removed lookup.

## Key Decisions

- Keep the existing edge type.
  - Why: the edge already carries `resolved_target_rel`, `target_is_crate`, `is_direct_child`, and `target_shared`, which are the only facts the rule reads.
- Do not widen the ingestion/type surface.
  - Why: this is a pure simplification repair, not a new family-shape change.

## Files To Modify

- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_06_shared_flag_required.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_06_shared_flag_required_tests/helpers.rs`
