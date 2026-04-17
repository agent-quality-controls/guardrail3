Summary

Normalized `packages/rs/test/g3rs-test-source-checks` to the current package shape and moved all rule-side proof assertions into the sibling assertions crate. The package now validates with `No findings.` and its workspace tests pass.

Decisions made

- Reused the `g3rs-test-file-tree-checks` package shape so test-family packages stay structurally consistent instead of evolving two different cleanup patterns.
- Converted every source-rule sidecar from `tests/mod.rs` into owned `rule_tests/` directories and pointed each rule file at that owned sidecar.
- Added one assertions module per rule and moved result-proof assertions there so rule tests no longer depend on the deleted local `test_helpers` bag.
- Split `parse/mod.rs` into `analysis`, `body`, `helpers`, and `types`, and collapsed the large parser structs into nested records instead of waiving the large-type warnings.

Key files for context

- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/analysis.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/body.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/types.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_16_assertions_modules_prove/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_17_external_harnesses_use_assertions/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/assertions/src/lib.rs`

Next steps

- Clean `packages/rs/topology/g3rs-topology-types`.
- Clean `packages/rs/topology/g3rs-topology-file-tree-checks`.
- Clean `packages/rs/topology/g3rs-topology-ingestion`.
- Run a fresh full package-root validate sweep and confirm every package root is clean.
