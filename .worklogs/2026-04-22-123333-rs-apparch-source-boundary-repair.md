Summary
- Repaired the `rs/apparch` source lane so `g3rs-apparch-source-checks` no longer rebuilds crate maps from a whole source bag.
- `g3rs-apparch-ingestion` now owns source fan-out into explicit io-traits and types-public-surface inputs.

Decisions made
- Kept the repair scoped to `apparch` source only.
  - Rejected mixing it with `release` because the seam defect was already confirmed locally in `apparch`.
- Used per-crate source inputs as the atomic rule contract.
  - Rejected preserving the whole source bag and hiding lookup logic in helpers because that leaves the defect intact.
- Left AST walking in ingestion.
  - Rejected moving item filtering or crate binding into rule files because that would weaken the package boundary again.

Key files for context
- `.plans/2026-04-22-122746-rs-apparch-source-boundary-repair.md`
- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_tests/basic.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/assertions/src/run.rs`

Next steps
- `rs/apparch` config and source lanes are both repaired.
- The next remaining Rust seam should be chosen from the next live bag-heavy package, likely outside `apparch`.
