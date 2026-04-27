## Summary

Fixed a real `g3rs-test/owned-sidecar-shape` boundary bug. The owned-sidecar rule was rebuilding sidecar ownership from the whole analyzed file bag even though ingestion already prebound sidecar facts on each component, which let a lossy file bag produce a false "ad hoc cfg(test) module declaration" result.

## Decisions made

- Added a red-first rule test proving the bug:
  - when `component.sidecars` already names `src/foo_tests/mod.rs`
  - and the top-level `input.files` bag is intentionally lossy
  - the rule must not accuse `src/foo.rs` of an ad hoc sidecar declaration
- Fixed the bug in the rule, not in fixtures:
  - `g3rs-test/owned-sidecar-shape` now validates owned sidecars from `component.sidecars`, `component.sidecar_files`, and `component.source_module_names`
  - `#[cfg(test)] mod ...` declarations now check against prebound sidecar mod paths instead of reconstructing from `input.files`
- Restored the independent direct scan for forbidden `src/**/tests/` trees:
  - that path is about raw layout detection, not sidecar ownership rebinding
- Updated rule-only fixtures to use `with_sidecar(...)` when they intend to model a valid prebound owned sidecar

## Key files for context

- [rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule.rs)
- [cases.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule_tests/cases.rs)
- [run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/run.rs)
- [rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-file-tree-checks/crates/assertions/src/rs_test_02_owned_sidecar_shape/rule.rs)
- [file_tree_analysis.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/file_tree_analysis.rs)
- [facts.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/components/facts.rs)

## Next steps

- Continue the remaining Rust audit from the next production-path check package that still rebuilds local state from oversized family inputs.
- Keep distinguishing between:
  - prebound ownership facts that checks should consume directly
  - direct path-shape scans that are still legitimately local to a file-tree rule
