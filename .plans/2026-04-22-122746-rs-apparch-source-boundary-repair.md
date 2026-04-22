Goal
- Repair `rs/apparch` source checks so the source lane consumes ingestion-owned atomic inputs instead of rebuilding crate indexes from the whole source bag inside `g3rs-apparch-source-checks`.

Approach
- Read the live source rule signatures and group them by owned source facts.
- Add a proving test in `g3rs-apparch-source-checks` that fails while `run.rs` still depends on whole-bag dispatch.
- Narrow `g3rs-apparch-types` source input into explicit per-crate source lanes:
  - one io crate plus its public trait items
  - one types crate plus its public behavioral items
- Move source fan-out and item grouping into `g3rs-apparch-ingestion`.
- Reduce `g3rs-apparch-source-checks::run` to pure dispatch over those precomputed inputs.
- Keep the rule files pure and local. Do not widen config-lane scope in this change.
- Run tests and `g3rs validate` for the touched `apparch` source slice.

Key decisions
- Keep this repair within `apparch` source only.
  - Alternative rejected: combine it with a broader `release` repair. That would mix two unrelated seams.
- Use one crate plus owned item list as the atomic source input.
  - Alternative rejected: keep `G3RsApparchSourceChecksInput` bag plus hide rebinding in a helper. That preserves the same defect.
- Leave source extraction in ingestion.
  - Alternative rejected: move more AST-level detail into rule files. That would regress the package boundary.

Files to modify
- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/*source*_tests/*`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/assertions/src/*` if the proving test needs assertion helpers
