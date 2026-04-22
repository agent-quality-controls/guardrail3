Summary
- Repaired the `rs/apparch` config lane so `g3rs-apparch-config-checks` no longer rebuilds crate indexes or rebinding state from whole-family bags.
- `g3rs-apparch-ingestion` now owns config fan-out into explicit dependency, purity, patch-bypass, and same-layer-cycle inputs.

Decisions made
- Kept the repair scoped to the config lane.
  - Rejected refactoring source checks in the same commit because the defect was localized to config dispatch.
- Preserved the public API while reducing hook-style bag inputs.
  - Rejected moving types behind new modules because that would change the package facade more than necessary.
- Kept same-layer cycles as a graph-shaped rule input.
  - Rejected forcing per-edge cycle rule inputs because cycle detection is inherently set-shaped.

Key files for context
- `.plans/2026-04-22-120631-rs-apparch-config-boundary-repair.md`
- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/run.rs`

Next steps
- Continue the Rust seam audit with the next bag-heavy package in the confirmed list.
- `rs/apparch` source lane is the natural follow-up if it still exposes oversized inputs.
