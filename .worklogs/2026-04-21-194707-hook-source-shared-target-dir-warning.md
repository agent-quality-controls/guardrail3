## Summary

Added a new Rust hook source warning that recommends a shared `CARGO_TARGET_DIR` when the pre-commit hook runs cargo. The rule lives in the existing hook source lane, explains the duplicate-build problem it solves in multi-workspace monorepos, and stays quiet when a hook does not run cargo at all.

## Decisions made

- Kept the rule in `hooks/source`, not `arch`.
  - Why: this is a hook execution policy about how cargo is invoked, not a package-structure rule.
  - Rejected: `arch`, because `arch` owns crate and source-tree structure.

- Made the rule `Warn` and cargo-scoped.
  - Why: shared target-dir setup is broadly beneficial but not universal enough for a hard failure.
  - The rule returns no result when a hook does not execute cargo.

- Required real coverage, not bare string presence.
  - The rule warns when cargo runs without either:
    - a persistent `export CARGO_TARGET_DIR=...`, or
    - an inline/env assignment on the cargo command itself.
  - Rejected: raw substring checks, because comments, echoes, and post-command assignments would be false positives.

- Closed one false-positive hole before commit.
  - Initial helper accepted `CARGO_TARGET_DIR=...` anywhere on a cargo line.
  - Fixed to require the assignment before the cargo invocation, and added a proving test.

## Key files for context

- `.plans/2026-04-21-194142-hook-source-shared-target-dir-warning.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/assertions/src/hook_rs_17_shared_target_dir_present/rule.rs`

## Verification

- `cargo test -q --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml`
- `g3rs validate --path packages/rs/hooks/g3rs-hooks-source-checks`

## Adversarial review

- Local test-attack pass found one real false-positive risk:
  - `CARGO_TARGET_DIR=...` appearing after `cargo ...` on the same line was treated as valid coverage.
- Fixed:
  - inline assignment detection now requires the assignment before the cargo command.
  - added a direct regression test for the post-command case.
- No remaining concrete blocker found in this rule scope.

## Next steps

- If this becomes a broader policy surface later, the next tightening step would be validating the target-dir shape itself, such as requiring a repo-local absolute path.
- That should stay a hook rule, not an `arch` rule.
