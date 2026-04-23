Goal:
Fix RS-TEST-FILETREE-18 so module-alias helper calls like `h::fixture_path()` and `h::any_rule()` resolve to local helpers instead of being treated as generic public helpers.

Approach:
- Add red-first regressions in `rule_tests/cases.rs` using valid nested-module Rust with `use self::helpers as h;`.
- Patch the helper-resolution logic in `rule.rs` so imported aliases are resolved for qualified paths, not only for single-segment calls.
- Keep the change local to the rule/helper boundary and avoid unrelated parser or ingestion edits.
- Run the `g3rs-test-file-tree-checks` package tests and validate the package path with `g3rs validate`.

Key decisions:
- The fix belongs in the local helper-resolution logic because the parser already provides the call path segments and import aliases.
- The regression should exercise both canned path/string helpers and semantic finding helpers.

Files to modify:
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs`
- `.worklogs/<dated>-rs-test-filetree-18-module-alias-helper-resolution.md`
