## Summary

Closed the known `test` AST lane coverage gaps and fixed the malformed-source
boundary so owned broken Rust files now reach `RS-TEST-10` through the real
pipeline. Added the missing rule-side sidecar tests for the hardest branches
and expanded AST-ingestion coverage for root-scoped file classification.

## Decisions made

- Stopped reparsing Rust source during AST ingestion.
  - Why: AST ingestion should gather and classify files only. Parsing belongs
    to `g3rs-test-ast-checks`, and keeping it there makes `RS-TEST-10`
    reachable end to end.

- Added a rule-specific sidecar test module for `RS-TEST-10`.
  - Why: the package was breaking the one-rule/one-sidecar test pattern.

- Expanded rule-local tests before widening pipeline tests.
  - Why: the attack found branch gaps in `RS-TEST-04`, `05`, `07`, `08`, `16`,
    and `17`. The correct place to pin those branches is the rule-side sidecar
    tests, not only broad pipeline smoke tests.

## Key files for context

- `.plans/2026-04-09-222615-test-ast-coverage-debt.md`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests/ast.rs`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/rs_test_10_input_failures/mod.rs`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/rs_test_10_input_failures/tests/mod.rs`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/rs_test_04_ignore_reason/tests/mod.rs`
- `packages/rs/test/g3rs-test-ast-checks/crates/runtime/src/rs_test_16_assertions_modules_prove/tests/mod.rs`

## Verification

- `cargo test --workspace -q` in `packages/rs/test/g3rs-test-ast-checks`
- `cargo test --workspace -q` in `packages/rs/test/g3rs-test-ingestion`
- `git diff --check`

## Next steps

1. Re-run adversarial `test-attack` on the `test` AST lane to find the next
   real gaps.
2. Decide whether any remaining proof-catalog branches still need dedicated
   alias / `self` / `super` parity tests.
3. Start the `test` file-tree lane once the AST lane stops moving.
