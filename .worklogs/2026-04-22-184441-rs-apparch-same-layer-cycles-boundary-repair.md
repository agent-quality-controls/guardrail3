## Summary

Repaired the remaining `rs/apparch` config-boundary defect in `RS-APPARCH-CONFIG-06`. Ingestion now binds the same-layer node set explicitly, and the rule consumes node paths plus prebound crate metadata instead of reconstructing a crate index from edge payloads.

## Decisions Made

- Kept the repair inside the existing same-layer-cycles lane.
  - Why: the defect was localized to one residual rule input shape, not the whole family.
  - Rejected: another broad `apparch` family reshape. The family was already mostly repaired.
- Changed same-layer edges to path refs plus a distinct crate set.
  - Why: the rule needs adjacency plus crate metadata. Those should be separate, ingestion-owned facts.
  - Rejected: keeping crate data duplicated on every edge and rebuilding a map inside the rule.
- Added a proving run test for the prebound node set.
  - Why: the previous rule could silently slide back to edge-driven rebinding.

## Key Files For Context

- [g3rs-apparch-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-types/src/types.rs)
- [g3rs-apparch-ingestion/crates/runtime/src/run/config.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs)
- [rs_apparch_config_06_same_layer_cycles.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles.rs)
- [run_tests/cases.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run_tests/cases.rs)
- [plan](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-22-184112-rs-apparch-same-layer-cycles-boundary-repair.md)

## Verification

- `cargo test -q --manifest-path packages/rs/apparch/g3rs-apparch-types/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/apparch/g3rs-apparch-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/apparch/g3rs-apparch-config-checks/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/apparch/g3rs-apparch-types`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/apparch/g3rs-apparch-ingestion`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/apparch/g3rs-apparch-config-checks`

## Next Steps

- Continue the Rust package-boundary audit from the remaining source-heavy families.
- Focus on packages where checks still reconstruct parser or graph semantics from oversized inputs instead of consuming ingestion-owned facts.
