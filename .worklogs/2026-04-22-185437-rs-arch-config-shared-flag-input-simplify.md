## Summary

Simplified `rs/arch` config checks by removing the redundant crate-map dependency from `RS-ARCH-CONFIG-06`. The rule now consumes only the dependency-edge fact it actually uses, and `run.rs` no longer rebuilds lookup state just to call that rule.

## Decisions Made

- Kept the existing dependency-edge type.
  - Why: it already carries every fact `RS-ARCH-CONFIG-06` reads: target resolution, crate-ness, direct-child status, and shared status.
  - Rejected: widening ingestion or types. This was a pure check-surface simplification.
- Removed the extra lookup from both runtime dispatch and rule tests.
  - Why: the local crate map was not contributing any rule decision. It only re-proved target existence that the edge had already encoded.

## Key Files For Context

- [rs_arch_06_shared_flag_required.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_06_shared_flag_required.rs)
- [run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/run.rs)
- [helpers.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_06_shared_flag_required_tests/helpers.rs)
- [plan](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-22-185337-rs-arch-config-shared-flag-input-simplify.md)

## Verification

- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-config-checks/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/arch/g3rs-arch-config-checks`

## Next Steps

- Continue the Rust package-boundary audit from the remaining source-heavy families.
- Re-check `rs/code` only if a production-path defect is found, not just test-only parsing helpers.
