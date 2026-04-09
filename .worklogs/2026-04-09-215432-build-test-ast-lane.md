## Summary

Implemented the real `test` AST lane. `g3rs-test-ast-checks` now parses root-scoped source bundles, builds assertions proof catalogs, runs the AST rules, and emits a direct `RS-TEST-10` parse-failure result so the lane does not fail open. `g3rs-test-ingestion` now implements `ingest_for_ast_checks` and emits owned root-scoped AST inputs.

## Decisions made

- Kept the AST scope root-scoped.
  - Why: `RS-TEST-07`, `16`, and `17` depend on assertions proof catalogs across files, so one-file inputs would force the wrong boundary.

- Moved direct Rust parse failures into the AST lane with `RS-TEST-10`.
  - Why: without that, `g3rs-test-ast-checks::check(...)` had no error channel and would fail open on malformed owned source.
  - Rejected silent skip behavior and rejected pushing parse validation entirely into ingestion as the only failure boundary.

- Fixed test-root discovery at the ingestion layer instead of patching individual rules.
  - Why: workspace members like `crates/runtime` and `crates/assertions` must collapse back to one logical test root.
  - This fix now benefits both config and AST ingestion.

- Kept parser-owned file types and only derived orchestration facts in ingestion.
  - AST ingestion reads and classifies source files, but the AST checks runtime still owns parse-once analysis and proof-catalog construction.

## Key files for context

- `.plans/2026-04-09-204323-test-config-and-ast-packages.md`
- `.plans/2026-04-09-213458-build-test-ast-lane.md`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/roots.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests/ast.rs`

## Verification

- `cargo test -q` in `packages/rs/test/g3rs-test-types`
- `cargo test --workspace -q` in `packages/rs/test/g3rs-test-config-checks`
- `cargo test --workspace -q` in `packages/rs/test/g3rs-test-ast-checks`
- `cargo test --workspace -q` in `packages/rs/test/g3rs-test-ingestion`

## Next steps

- Send adversarial `test-attack` rounds against the new `test` AST lane.
- Decide whether the remaining deferred `test` rules stay mixed or split further.
- Build the `test` file-tree lane once that mixed-boundary decision is locked.
