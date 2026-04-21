## Goal

Add a Rust hook source warning that recommends a repo-shared `CARGO_TARGET_DIR` in pre-commit hooks, with a concrete explanation of the build-cache problem it solves in multi-workspace monorepos.

## Approach

- Add one new source rule module under `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src`.
  - Detect whether the selected pre-commit script contains at least one real `cargo` command.
  - Detect whether the script sets `CARGO_TARGET_DIR` in a persistent way before those cargo commands run.
  - Emit a warning inventory result when present and a warning finding when missing.
- Add the paired assertions module under `packages/rs/hooks/g3rs-hooks-source-checks/crates/assertions/src`.
- Add owned sidecar tests that prove:
  - missing when cargo commands run without shared target-dir setup
  - present for `export CARGO_TARGET_DIR=...`
  - present for top-level `CARGO_TARGET_DIR=... cargo ...`
  - ignore comments and echoed strings
  - do not fire when the hook has no cargo commands
- Wire the rule into the source runner.
- Run package tests, formatter, validator, and a local adversarial review.

## Key decisions

- Keep this in `hooks/source`, not `arch`.
  - Why: this is a hook execution contract about how the script invokes cargo.
  - Rejected: `arch`, because `arch` owns package structure, not hook runtime policy.
- Make it `Warn`, not `Error`.
  - Why: the setup is broadly beneficial for monorepos with multiple Cargo workspaces, but not universally mandatory enough for a hard failure.
- Scope to scripts that actually execute `cargo`.
  - Why: a hook with no cargo commands has no build-cache problem to solve.

## Files to modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule_tests/mod.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/assertions/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/assertions/src/hook_rs_17_shared_target_dir_present/rule.rs`
