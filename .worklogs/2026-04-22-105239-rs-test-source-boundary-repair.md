Summary

- Repaired the `rs/test` source-check boundary so `g3rs-test-source-checks` no longer reparses raw file bags.
- Source parsing, parse-failure accumulation, and assertions proof-catalog derivation now live in `g3rs-test-ingestion`, and the affected `rs/test` packages are green under both `cargo test` and `g3rs validate`.

Decisions made

- Moved source-analysis facts into `g3rs-test-types` and grouped the AST-facing types under `g3rs_test_types::ast`.
  - Why: the source-check runtime still needs typed parsed facts, but the crate-root facade must stay small and explicit.
  - Rejected: keeping a flat crate-root export list. It tripped the facade import-count rule.
- Added public fixture builders in `g3rs-test-ingestion-runtime::fixtures` and removed the local `support.rs` workaround from the source-check assertions crate.
  - Why: assertions modules must not import local private code or use `#[path]` escapes.
  - Rejected: keeping a private helper module in the assertions crate. The validator correctly rejected that shape.
- Kept the bug proof at the ingestion pipeline layer, but rewrote the regression test to assert through the existing assertions helpers instead of counting `CheckResult` ids directly.
  - Why: the bug was "multiple parse failures and valid files must survive the pipeline", not "this sidecar test may own semantic result assertions".

Key files for context

- [plan](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/2026-04-22-101842-rs-test-source-boundary-repair.md)
- [types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-types/src/types.rs)
- [lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-types/src/lib.rs)
- [source_analysis.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis.rs)
- [fixtures.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/fixtures.rs)
- [run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run.rs)
- [support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs)
- [rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs)
- [rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-source-checks/crates/assertions/src/rs_test_07_real_proof_site/rule.rs)

Next steps

- Apply the same "ingestion owns parsing, checks consume facts" repair to `g3rs-test-file-tree-checks`, which still carries its own local parse layer.
- Audit other Rust families that still expose parser-heavy flat facades and group their internal AST types behind narrower public modules where needed.
