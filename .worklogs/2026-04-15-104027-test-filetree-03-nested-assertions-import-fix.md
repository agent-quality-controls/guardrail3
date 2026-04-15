# Summary
Fixed a contradiction in `RS-TEST-FILETREE-03`. The rule already required nested shared assertions files like `assertions/x/rule.rs`, but it still rejected sidecars that imported that same nested path.

# Decisions Made
- Fixed the rule instead of bending the package around it. Rejected a package-only workaround because the bug lived in the rule's path check.
- Added a direct failing rule test first for `foo/rule_tests -> assertions/foo/rule.rs`. Rejected looser coverage because this bug was about one exact nested path shape.
- Kept the change narrow. Rejected relaxing other sidecar boundary checks or changing the expected assertions file layout.

# Key Files For Context
- `.plans/2026-04-15-103803-test-filetree-03-nested-assertions-import-fix.md`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/tests/mod.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/helpers.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs`

# Next Steps
- Return to `packages/rs/clippy/g3rs-clippy-config-checks` and fix the remaining package-side `test` findings.
- Add `crates/test_support`, move generic builders there, and keep semantic proof in the assertions crate.
- Then rerun `guardrail3-rs validate --path packages/rs/clippy/g3rs-clippy-config-checks --family test`.
