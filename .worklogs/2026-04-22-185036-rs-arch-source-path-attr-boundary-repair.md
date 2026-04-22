## Summary

Repaired the remaining `rs/arch` source-boundary defect by moving `#[path]` analysis for `RS-ARCH-SOURCE-09` into ingestion. The source lane now carries explicit `path_attr_sites` instead of raw Rust source file bags for that rule.

## Decisions Made

- Removed `source_files` from `G3RsArchSourceChecksInput`.
  - Why: once `RS-ARCH-SOURCE-09` moved to ingestion, no source rule still needed raw file content.
  - Rejected: keeping the raw file bag and only changing the rule. That would leave an oversized source input around for no owner.
- Added ingestion-owned `G3RsArchPathAttrSite`.
  - Why: the rule needs one local fact per `#[path]` use, plus enough context to apply the test-sidecar exemption.
  - Rejected: passing whole parsed AST files through family types. The rule only needs the site facts.
- Kept facade-surface analysis untouched.
  - Why: that part of the source lane was already ingestion-owned and correctly shared across the other source rules.

## Key Files For Context

- [g3rs-arch-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-types/src/types.rs)
- [g3rs-arch-ingestion/crates/runtime/src/source.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source.rs)
- [g3rs-arch-source-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run.rs)
- [rs_arch_09_no_path_attr.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr.rs)
- [plan](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-22-184727-rs-arch-source-path-attr-boundary-repair.md)

## Verification

- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-types/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-source-checks/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/arch/g3rs-arch-types`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/arch/g3rs-arch-ingestion`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/arch/g3rs-arch-source-checks`

## Next Steps

- Continue the Rust package-boundary audit from the remaining relation and source lanes.
- Prioritize packages where checks still rebuild maps or parser semantics locally instead of consuming ingestion-owned facts.
