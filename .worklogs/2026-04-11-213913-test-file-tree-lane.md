## Summary

Built the `test` family file-tree lane in packages and wired it through `g3rs-test-ingestion`. The new lane now owns `RS-TEST-02`, `RS-TEST-03`, `RS-TEST-10`, and `RS-TEST-18`, with end-to-end pipeline tests and follow-up hardening from an adversarial pass.

## Decisions made

- Added a dedicated `packages/rs/test/g3rs-test-file-tree-checks` package instead of widening the source lane.
  - The remaining rules are structural and import-boundary checks, not source-only checks.
- Reused the existing `syn` parser and proof-catalog support inside the file-tree runtime.
  - This kept the rule logic aligned with the source lane while letting ingestion stay responsible for structural discovery.
- `g3rs-test-ingestion::ingest_for_file_tree_checks(...)` now emits one input per owned test root.
  - This matches the family activation model and keeps rule inputs local.
- Fixed activation at the runtime boundary instead of suppressing findings rule by rule.
  - Inactive roots with only `assertions` or `test_support` no longer emit `RS-TEST-02`, `03`, `10`, or `18`.
- Fixed parser macro traversal instead of special-casing `RS-TEST-18`.
  - Generic macros like `vec![fixture_path()]` now expose nested helper calls to the semantic-helper and canned-fixture checks.

## Key files for context

- `.plans/2026-04-11-210619-test-file-tree-lane.md`
- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests/file_tree.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/support.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`

## Next steps

- Audit old app `RS-TEST` references and switch anything still bridging to the new file-tree lane.
- Decide whether any leftover old app `test` structure helpers can be deleted now that the package lane exists.
- Keep attacking `RS-TEST-03` and `RS-TEST-18` boundary cases if new family migrations expose more helper-indirection tricks.
