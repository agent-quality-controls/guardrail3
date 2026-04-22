## Goal

Fix the `rs/release` workflow matcher bug where a release step with `--manifest-path .../Cargo.toml` can be credited to the wrong binary crate when more than one binary crate exists.

## Approach

1. Add a regression test in `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/cases.rs` that proves a manifest-path build for a non-publishable binary crate does not satisfy the publishable crate.
2. Tighten the matcher in `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/workflow.rs` so the single-binary shortcut is based on the total number of binary crates, not just publishable binary crates.
3. Run the package test suite and `g3rs validate` for the touched package to confirm the fix and guard against collateral regressions.

## Key decisions

- Fix the helper in `support/workflow.rs`, not the rule wrappers.
  - Reason: the bug is in the shared workflow identity check, and both release rules consume that helper.
- Keep the change minimal.
  - Reason: the root cause is the publishable-only shortcut, so switching to the total binary count is enough.
- Prove the bug with a red test before the code change.
  - Reason: the workflow helper currently over-matches a manifest-path build when only one publishable binary crate exists.

## Files to modify

- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/basic.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/workflow.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_23_binary_release_workflow.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_24_linux_release_target.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/cases.rs`
- `.worklogs/2026-04-22-*.md`
