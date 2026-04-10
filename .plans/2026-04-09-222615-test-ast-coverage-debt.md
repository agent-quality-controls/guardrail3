## Goal

Close the known `test` AST lane gaps from the latest adversarial review so the
lane works end to end and the hardest rule branches are actually pinned by
tests.

## Approach

1. Fix the malformed-source boundary in `g3rs-test-ingestion`.
   - Stop reparsing owned Rust files during AST ingestion.
   - Let ingestion classify and read files only.
   - Let `g3rs-test-source-checks` own Rust parse failures so malformed owned
     source produces `RS-TEST-10` through the real lane.

2. Restore the package test pattern for `RS-TEST-10`.
   - Add a rule-specific `tests/` sidecar for `rs_test_10_input_failures`.
   - Keep the rule tiny and test it directly.

3. Expand missing AST rule coverage where the attack found real holes.
   - `RS-TEST-04`: strong reason, weak reason, previous-line/same-line comment,
     `#[ignore = "..."]`, `cfg_attr(..., ignore)`, aggregate counts.
   - `RS-TEST-05`: blank `expected = ""` and `cfg_attr(..., should_panic)`.
   - `RS-TEST-16`: sidecar direct semantic proof and sidecar delegated proof.
   - `RS-TEST-17`: local helper propagation.
   - `RS-TEST-07`: alias/glob owned-assertions proof paths.
   - `RS-TEST-08`: `assert_matches!` / `debug_assert!` paths.

4. Expand ingestion and pipeline coverage.
   - Add malformed-source pipeline proof for `RS-TEST-10`.
   - Add direct AST-ingestion classification assertions for source, sidecar mod,
     sidecar support, external harness, assertions module, fixture exclusion,
     and `Other`.

## Key decisions

- Fix the parse-failure bug at the ingestion/checks boundary.
  - Ingestion should not pre-parse Rust source for the AST lane.
  - That keeps ownership simple: ingestion gathers files, AST checks parse.

- Add tests at the rule sidecar level before widening pipeline tests.
  - This keeps branch coverage close to the rule and matches the repo pattern.

## Files to modify

- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests/ast.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_10_input_failures/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_10_input_failures/tests/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_04_ignore_reason/tests/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_05_should_panic_expected/tests/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_08_weak_matches_assert/tests/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_16_assertions_modules_prove/tests/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/tests/mod.rs`
