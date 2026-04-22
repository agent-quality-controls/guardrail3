## Summary

Fixed the `rs/release` workflow matcher so a manifest-path build for a non-publishable binary crate no longer gets credited to the only publishable binary crate in the repo. The helper now uses total binary-crate count for the single-crate shortcut, and the release package regression proves the false positive is gone.

## Decisions made

- Switched the workflow shortcut from publishable-binary count to total binary-crate count.
  - Why: the old shortcut made any build line count as a match when only one publishable binary crate existed, even if the workflow explicitly targeted another binary crate.
- Kept the fix in the shared workflow helper and the two release rule wrappers.
  - Why: both `RS-RELEASE-CONFIG-23` and `RS-RELEASE-CONFIG-24` consume the same matcher, so the root fix belongs there.
- Added the regression in `run_tests/cases.rs`.
  - Why: it exercises the real rule dispatch path and fails on the exact mis-crediting behavior.
- Ran `g3rs validate --path packages/rs/release/g3rs-release-config-checks --family release`.
  - Result: the touched package path validated cleanly, but the repo-wide CLI invocation currently trips an unrelated hooks compile error in `hook_shared_13_no_unconditional_exit_zero`.

## Key files for context

- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/workflow.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/basic.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_23_binary_release_workflow.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_24_linux_release_target.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/cases.rs`

## Next steps

- None for this fix.
