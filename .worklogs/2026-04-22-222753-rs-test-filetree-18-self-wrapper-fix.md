## Summary

Fixed `g3rs-test/test-support-generic` so `test_support` helper wrappers reached through `self::` are still classified as local canned or semantic helpers. The rule now treats local-boundary qualified call paths as local helper use, which closes the production-path miss without changing ingestion facts.

## Decisions made

- Fixed the rule-local helper expansion instead of changing parser facts.
  - Why: the parser already exposed the call-path shape; the bug was the rule only considering bare calls.
- Added red regressions for both canned and semantic helper wrappers.
  - Why: both branches shared the same bare-call blind spot.
- Kept the fix limited to local-boundary qualified paths.
  - Why: that covers the concrete `self::...` wrapper repro without broadening helper matching unnecessarily.

## Key files for context

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`

## Next steps

- None for this fix.
