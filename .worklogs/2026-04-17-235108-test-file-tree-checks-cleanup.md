Summary

Normalized `packages/rs/test/g3rs-test-file-tree-checks` to the current package shape and finished the last source splits needed to remove all family findings. The package now validates with `No findings.` and its workspace tests pass.

Decisions made

- Moved sidecar proof assertions into the sibling assertions crate and removed local runtime test helpers, so rule sidecars only own module-local setup while semantic proof stays shared.
- Split `parse` into `analysis`, `body`, `helpers`, and `types` so the parser stays readable without changing rule behavior.
- Split `rs_test_03_runtime_assertions_split` support into dedicated helper modules so the rule file and violation collector stay under source thresholds.
- Replaced the large flat parser structs with nested records for signature/body/assertion facts instead of waiving them, preserving the same parsed facts while removing the large-type warnings.

Key files for context

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/parse/analysis.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/parse/body.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/parse/types.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/violations.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/assertions_violations.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/assertions/src/lib.rs`

Next steps

- Clean `packages/rs/test/g3rs-test-source-checks`.
- Clean `packages/rs/topology/g3rs-topology-types`.
- Clean `packages/rs/topology/g3rs-topology-file-tree-checks`.
- Clean `packages/rs/topology/g3rs-topology-ingestion`.
- Run a full package-root validate sweep and confirm only the previously accepted parser warnings remain.
