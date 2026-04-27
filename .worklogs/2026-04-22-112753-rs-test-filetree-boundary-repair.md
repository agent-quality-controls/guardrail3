Summary

- Repaired the `rs/test` file-tree boundary so `g3rs-test-file-tree-checks` now consumes ingestion-owned analyzed file facts instead of reparsing raw source bags locally.
- Deleted the duplicate file-tree parser, moved file-tree normalization into `g3rs-test-ingestion`, and fixed one shared-parser drift that dropped calls nested inside macro arguments.

Decisions made

- Moved file-tree source analysis into ingestion instead of introducing another file-tree-specific parse facade.
  - Why: the package defect was duplicate parse ownership in the check package. The correct fix was one shared parser/analyzer in ingestion with one family input shape.
  - Rejected: keeping raw `G3RsTestSourceFile` in file-tree input and only stopping local reparsing. That would still leak parser work across the boundary.
- Extended `G3RsTestFileTreeChecksInput` with analyzed files and ingestion-owned activation/package facts.
  - Why: file-tree runtime rules need parsed AST facts, test activation, and local package inventories. Those are family facts and belong on the input, not in check-local support code.
- Fixed the shared ingestion parser to traverse macro argument expressions for all macros.
  - Why: replacing the local file-tree parser exposed a real behavioral drift. Calls inside `vec![...]` and similar wrappers were no longer recorded, which broke `g3rs-test/test-support-generic`.
  - Rejected: weakening the rule or restoring the deleted file-tree-local parser.
- Switched file-tree runtime rule tests to use ingestion fixtures.
  - Why: tests still need realistic analyzed inputs after the runtime parse layer is removed.
  - Rejected: reintroducing parser-only test helpers inside file-tree runtime.

Key files for context

- `.plans/2026-04-22-110417-rs-test-filetree-boundary-repair.md`
- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/file_tree_analysis.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/parse/body.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/violations.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`

Next steps

- Continue the same boundary cleanup on the next bag-heavy `rs/test` slice if any remain after config/file-tree/source are rechecked together.
- If more shared-parser drift appears while collapsing duplicate parsers, fix it in `g3rs-test-ingestion` rather than recreating local parser ownership in downstream check packages.
