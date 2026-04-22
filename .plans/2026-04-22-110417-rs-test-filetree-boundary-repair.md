Goal

- Move Rust source parsing and assertions proof-catalog derivation out of `g3rs-test-file-tree-checks` and into `g3rs-test-ingestion` so file-tree rules consume ingestion-owned facts instead of reparsing raw source files.

Approach

- Extend `g3rs-test-types` with file-tree analyzed-file and root-analysis data that can cross the family boundary without leaking raw parser work into checks.
- Add file-tree source analysis to `g3rs-test-ingestion`:
  - parse owned files once
  - accumulate parse failures
  - derive assertions proof catalogs once
  - compute activation facts and local package inventories once
- Change `ingest_for_file_tree_checks` to emit analyzed file-tree facts instead of raw file-content bags.
- Remove local parse and root analysis from `g3rs-test-file-tree-checks/support.rs`.
- Update file-tree rules to consume the new ingestion-owned facts directly.
- Add tests first to prove:
  - malformed file-tree source files are reported without aborting the whole root
  - valid files in the same root still get checked

Key decisions

- Reuse the `g3rs_test_types::ast` types introduced in the source-boundary repair.
  - Why: the file-tree slice needs the same parsed Rust facts; duplicating another AST surface would repeat the same facade problem.
  - Rejected: keeping a private parse layer in file-tree checks.
- Keep file-tree-specific root normalization in ingestion, not in `g3rs-test-types`.
  - Why: the types crate should expose facts, not compute them.
- Preserve current rule behavior.
  - Why: this is a package-boundary repair and bug fix, not a rule-inventory change.

Files to modify

- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-types/src/lib.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run_tests/*`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/file_tree_analysis.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/parse/*`
- file-tree rule files that currently depend on local parsed/root-analysis helpers
