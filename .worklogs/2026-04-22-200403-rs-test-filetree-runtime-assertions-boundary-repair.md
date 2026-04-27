## Summary

Repaired the remaining `g3rs-test/runtime-assertions-split` boundary defect in the `rs/test` file-tree lane. Ingestion now binds component-local file-tree facts once, and the runtime/assertions split rule consumes those facts directly instead of rebuilding cross-file lookup state from the top-level analyzed file bag.

## Decisions made

- Added file-tree input facts for:
  - `existing_file_paths`
  - per-component `source_module_names`
  - per-component `sidecar_files`
  - per-component `external_harness_files`
  - per-component `assertions_module_files`
  - Why: `g3rs-test/runtime-assertions-split` was reconstructing this information locally from `input.files`.
- Kept the top-level `files` bag for the other file-tree rules.
  - Why: this repair is scoped to `g3rs-test/runtime-assertions-split`, not a full family-wide reshape.
- Added red-first proof that the rule was still trusting `input.files` over ingestion-owned path facts.
  - The failing case was a sidecar assertions-module path present in `existing_file_paths` but absent from `input.files`; the old rule still reported "sidecar missing owned assertions module".
- Added a second proof that component-bound sidecar files now drive `g3rs-test/runtime-assertions-split` even if the sidecar file is removed from the top-level `files` bag.

## Key files for context

- [types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-types/src/types.rs)
- [file_tree_analysis.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/file_tree_analysis.rs)
- [fixtures.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/fixtures.rs)
- [violations.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/violations.rs)
- [assertions_violations.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/assertions_violations.rs)
- [rule tests](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule_tests/cases.rs)

## Next steps

- Continue the remaining Rust boundary audit from the smaller residual cases, not from config-document families like `rs/cargo`.
- Keep `g3rs-test/runtime-assertions-split` on ingestion-owned component facts. Do not reintroduce check-local `parsed_by_path` or component-local rescans of `input.files`.
