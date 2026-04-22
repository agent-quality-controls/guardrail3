Goal

- Move Rust source parsing and assertions proof-catalog derivation out of `g3rs-test-source-checks` and into `g3rs-test-ingestion` so source rules consume ingestion-owned facts instead of reparsing raw file bags.

Approach

- Extend `g3rs-test-types` with source-analysis data types that can cross the family boundary.
- Move the current `parse` analysis implementation from `g3rs-test-source-checks` into `g3rs-test-ingestion`.
- Change `ingest_for_source_checks` to:
  - parse and analyze each owned file once
  - compute assertions proof catalogs once
  - emit typed analyzed-file facts and proof-catalog facts in `G3RsTestSourceChecksInput`
- Remove root-wide parsing and catalog construction from `g3rs-test-source-checks/support.rs`.
- Update source-check rules to consume the new analyzed facts directly.
- Add tests first to prove:
  - source-checks no longer need raw file content parsing to run
  - source ingestion emits the parsed facts and proof catalog that current rules need

Key decisions

- Keep the family-local Rust AST analysis in `rs/test` ingestion for now.
  - Why: the immediate bug is package-boundary ownership, not parser sharing across families.
  - Rejected: creating a new standalone Rust test-source parser package in this step. That is larger than the proven bug.
- Move parsed structures into `g3rs-test-types`.
  - Why: source checks need typed facts without depending on ingestion internals.
  - Rejected: leaving parsed types private to ingestion and exposing ad hoc wrapper bags.
- Preserve current rule behavior.
  - Why: this is a boundary repair, not a rule-inventory change.

Files to modify

- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run_tests/source.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/*`
- rule files in `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/`
