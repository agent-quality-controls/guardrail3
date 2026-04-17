Summary
- Cleaned `packages/rs/release/g3rs-release-repo-root-checks` to the current internal package shape and brought it to `No findings.`
- Removed the old local test support path, moved owned sidecar helpers into rule-local modules, and mirrored the owned runtime shape in the assertions crate.

Decisions made
- Switched runtime from the local `crates/types` facade to `g3rs-release-types` directly so the boundary matches the rest of the cleaned release family instead of preserving an internal reach-through.
- Deleted `crates/runtime/src/test_support.rs` rather than keeping a shared helper bag. Each rule-local sidecar now builds its own typed input in `rule_tests/helpers.rs`.
- Reshaped the assertions crate from flat files into owned `<rule>/rule.rs` modules so the proof surface matches the runtime rule ownership model.
- Feature-gated the root and local types facade exports instead of leaving them globally visible.
- Kept the cleanup package-local. No rule changes were needed.

Key files for context
- `packages/rs/release/g3rs-release-repo-root-checks/Cargo.toml`
- `packages/rs/release/g3rs-release-repo-root-checks/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/lib.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/rs_release_repo_root_01_release_plz_workflow/rule.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/rs_release_repo_root_02_publish_dry_run_workflow/rule.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/rs_release_repo_root_03_registry_token/rule.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/assertions/src/common.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/assertions/src/rs_release_repo_root_01_release_plz_workflow/rule.rs`

Next steps
- Continue with `packages/rs/release/g3rs-release-source-checks` as the next dirty release package root.
- Keep the sweep package-by-package and stop only if `g3rs-release-source-checks` exposes a real rule contradiction.
