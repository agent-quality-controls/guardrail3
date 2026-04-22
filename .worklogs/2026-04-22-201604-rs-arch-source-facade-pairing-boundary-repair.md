Summary
- Repaired the `rs/arch` source lane so `g3rs-arch-source-checks` no longer rebuilds a local facade lookup from a bag input.
- Ingestion now binds each source crate to its optional `lib.rs` facade surface, and source checks dispatch directly over that prebound pair.

Decisions made
- Added `G3RsArchLibFacadeChecksInput` instead of keeping separate `crates` and `facade_surfaces` bags in `G3RsArchSourceChecksInput`.
  - Why: `RS-ARCH-SOURCE-02` and `RS-ARCH-SOURCE-08` both want one atomic unit: one source crate plus its bound `lib.rs` surface, if any.
  - Rejected: leaving `run.rs` to rebuild `facade_map`, because that kept pairing logic in the check package.
- Split source inputs into `lib_facade_checks` and `mod_facade_surfaces`.
  - Why: `RS-ARCH-SOURCE-04` naturally operates on `mod.rs` surfaces only, so the source lane no longer needs to filter that in checks.
- Put the proof on `run.rs` with an owned `run_tests` sidecar.
  - Why: the validator correctly rejected the first `lib.rs` sidecar attempt because it escaped the owned module boundary to call `check`.

Key files for context
- [packages/rs/arch/g3rs-arch-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-types/src/types.rs)
- [packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source.rs)
- [packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run.rs)
- [packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run_tests/cases.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run_tests/cases.rs)
- [packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/run.rs)
- [packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source_tests/pipeline.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source_tests/pipeline.rs)

Next steps
- Re-run the remaining Rust-boundary audit and pick the next confirmed local-rebinding defect rather than applying the pattern blindly to config families.
- Keep config-family parsed document surfaces intact unless the family subject is inherently a derived graph relation.
