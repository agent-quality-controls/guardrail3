## Summary

Fixed `g3rs-test/test-support-generic` so local canned and semantic helpers in `test_support` still count when they are reached through an alias chain like `let run = fixture_path; let again = run; again()`. The rule now follows local-call aliases recursively instead of stopping after one hop.

## Decisions made

- Fixed the shared local-helper matcher in `rule.rs`.
  - Why: both canned and semantic helper detection use the same path resolution logic, so one recursive fix covers both.
- Added one regression for each real helper class.
  - Why: the alias-chain miss affects both canned and semantic helper promotion.
- Kept the change in the rule boundary rather than changing ingestion facts.
  - Why: the parser already records the alias graph; the bug was in how the rule consumed it.

## Key files for context

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `.plans/2026-04-22-223715-rs-test-filetree-18-alias-chain-helper-fix.md`

## Next steps

- None for this fix.
