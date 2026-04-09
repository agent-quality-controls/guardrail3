# Build test AST lane

## Goal

Replace the `test` AST stubs with a real root-scoped AST lane:

- `packages/rs/test/g3rs-test-ast-checks`
- `packages/rs/test/g3rs-test-ingestion::ingest_for_ast_checks`

The resulting lane should run the planned AST rules end to end:

- `RS-TEST-01`
- `RS-TEST-04`
- `RS-TEST-05`
- `RS-TEST-06`
- `RS-TEST-07`
- `RS-TEST-08`
- `RS-TEST-16`
- `RS-TEST-17`

## Approach

1. Reuse the existing root discovery and component discovery in
   `g3rs-test-ingestion`.
   - Extend it to build root-scoped AST inputs:
     - owned root metadata
     - root-owned Rust source contents
     - component facts needed for assertions/runtime relationships

2. Implement `ingest_for_ast_checks` in `g3rs-test-ingestion`.
   - Read owned Rust files only.
   - Reuse the same ownership rules already used by the config lane.
   - Classify files into:
     - `Source`
     - `InternalSidecarMod`
     - `InternalSidecarSupport`
     - `ExternalHarness`
     - `AssertionsModule`
     - `Other`
   - Emit one `G3RsTestAstChecksInput` per owned root.

3. Build parse-once runtime support in `g3rs-test-ast-checks`.
   - Port the legacy test-family AST parser into the package runtime.
   - Keep parsing and proof-catalog construction in runtime support, not rules.
   - Use the public source bundle from `g3rs-test-types` as the only package input.

4. Port the AST rules into one-file-per-rule modules.
   - Simpler rules:
     - `RS-TEST-01`
     - `RS-TEST-04`
     - `RS-TEST-05`
     - `RS-TEST-06`
     - `RS-TEST-08`
   - Proof-catalog rules:
     - `RS-TEST-07`
     - `RS-TEST-16`
     - `RS-TEST-17`

5. Add tests at three levels.
   - rule-local tests in `g3rs-test-ast-checks`
   - ingestion tests in `g3rs-test-ingestion`
   - pipeline tests:
     - `crawl -> ingest_for_ast_checks -> g3rs-test-ast-checks::check`

## Key decisions

- Keep `RS-TEST-10` out of this lane.
  - It is the fail-closed input-failure rule and stays deferred with the mixed lane.

- Keep the AST scope root-scoped.
  - `RS-TEST-07`, `16`, and `17` need assertions proof catalogs across files.
  - One-file AST would force the wrong boundary.

- Keep parsed AST data runtime-local.
  - Public package input stays as source bundles plus component metadata.
  - Rules receive small runtime-derived inputs, not `&syn::File` bags from ingestion.

- Reuse the legacy parser and proof-catalog logic structurally, but keep the new package boundaries.
  - We want behavior parity without dragging app-owned route logic into packages.

## Files to modify

- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/roots.rs`
- new or expanded shared discovery files under:
  - `packages/rs/test/g3rs-test-ingestion/crates/runtime/src`

- `packages/rs/test/g3rs-test-ast-checks/Cargo.toml`
- `packages/rs/test/g3rs-test-ast-checks/src/lib.rs`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/Cargo.toml`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/run.rs`
- new parser/support/rule files under:
  - `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src`

- tests under:
  - `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/*/rule_tests`
  - `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests`
