## Summary

Fixed `RS-TEST-FILETREE-18` so `test_support` cannot hide canned helper and semantic helper usage behind a local alias like `let run = fixture_path; run()`. Added red tests for both alias-routed cases and fixed the rule to resolve called local aliases instead of only direct helper names.

## Decisions made

- Fixed the rule instead of ingestion.
  - Why: the parser already exposes `local_call_aliases`; the bug was that the rule ignored them.
- Checked alias use through the actual called identifier.
  - Why: the first attempt incorrectly filtered out alias variables because they are shadowed by definition. The correct question is whether a called bare name resolves through `local_call_aliases` to a local canned or semantic helper.
- Fixed both canned and semantic helper paths together.
  - Why: they were the same alias-blind defect in the same rule.

## Key files for context

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `.plans/2026-04-22-221213-rs-test-support-alias-helper-fix.md`

## Next steps

- Continue the remaining `rs/test` attack findings if any new concrete wrong-result cases remain.
- Keep rule fixes focused on existing parsed facts instead of rebuilding lookup state in checks.
