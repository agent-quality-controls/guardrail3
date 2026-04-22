## Summary

Repaired the `rs/garde` source lane so ingestion owns Rust source reading, parsing, cross-file garde analysis, and source-check input derivation. `g3rs-garde-source-checks` now consumes analyzed garde sites only and no longer carries a production parser or filesystem layer.

## Decisions Made

- Moved garde source analysis into `g3rs-garde-ingestion`.
  - Why: source checks were reparsing raw file bags and owning analysis that belongs in ingestion.
  - Rejected: keeping a second parser inside source checks. That duplicates semantics and keeps the boundary wrong.
- Narrowed `G3RsGardeSourceChecksInput` to analyzed source lanes.
  - Why: the source lane should receive one local opportunity for a rule to fire, plus ingestion-owned input failures.
  - Rejected: preserving `Vec<G3RsSourceFile>` in source checks. That would keep parsing and cross-file normalization in the check package.
- Rewrote source-check rule tests to use direct analyzed fixtures.
  - Why: once production parsing moved into ingestion, the old parser-driven unit fixtures became the wrong test shape.
  - Rejected: keeping hidden `support/test_support` parser code under `#[cfg(test)]`. That kept the same duplicated parser architecture alive in tests.

## Key Files For Context

- [g3rs-garde-types/src/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/garde/g3rs-garde-types/src/lib.rs)
- [g3rs-garde-ingestion/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs)
- [g3rs-garde-ingestion/crates/runtime/src/source_analysis/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/source_analysis/run.rs)
- [g3rs-garde-source-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run.rs)
- [g3rs-garde-source-checks/crates/runtime/src/support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/support.rs)
- [plan](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-22-180632-rs-garde-source-boundary-repair.md)

## Verification

- `cargo test -q --manifest-path packages/rs/garde/g3rs-garde-types/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/garde/g3rs-garde-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/garde/g3rs-garde-source-checks/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/garde/g3rs-garde-types`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/garde/g3rs-garde-ingestion`
- `cargo run -q --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml -- validate --path packages/rs/garde/g3rs-garde-source-checks`

## Next Steps

- Review the next remaining Rust family for the same defect pattern: production checks rebuilding parser or graph semantics from oversized inputs.
- Keep the config-family rule intact while doing that review: ingestion selects and parses config surfaces, config checks interpret them.
