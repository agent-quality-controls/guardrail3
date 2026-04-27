Summary

Repaired the `rs/code` source-lane boundary so ingestion now owns Rust source parsing and parse-failure routing. `g3rs-code-source-checks` now consumes prebound parsed-or-invalid source state instead of reparsing in the production run path.

Decisions made

- Added `G3RsCodeParsedSourceState` to `g3rs-code-types` and routed it through ingestion instead of leaving parse responsibility in source checks.
- Kept parsed config-surface work out of this change. This repair is limited to the source lane.
- Added red-first proving tests at the source-check run boundary:
  - valid prebound AST wins even if raw content is invalid
  - invalid prebound source routes to `g3rs-code/ast-30-input-failures`
- Fixed the follow-up self-guardrail break in `g3rs-code-ingestion` by keeping all proof-bearing pipeline assertions in the owned `crates/assertions/src/run.rs` module and compressing repetition until the file stayed under `g3rs-code/ast-09-too-many-effective-code-lines`.
- Verified the remaining `parse_rust_file(...)` hits in `g3rs-code-source-checks` are test-only `check_source(...)` helpers behind `#[cfg(test)]`, not production path reparsing.

Key files for context

- `packages/rs/code/g3rs-code-types/src/types.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/support.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run_tests/cases.rs`
- `.plans/2026-04-22-190008-rs-code-source-boundary-repair.md`

Next steps

- Continue the remaining `rs/code` cleanup only if another boundary defect is confirmed. The specific source-lane production reparsing defect fixed here is closed.
- Keep config-family surfaces intact when resuming broader Rust cleanup. This repair should not be generalized into config-document slicing.
