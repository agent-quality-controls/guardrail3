Summary
- Verified the repo-level G3RS gate and full non-staged G3RS validation for every adopted workspace.
- Fixed the only deterministic failure by updating the stale lockfile for `packages/rs/test/g3rs-test-file-tree-checks` after the shared workspace crawl package rename.

Decisions made
- Kept the fix limited to the affected `Cargo.lock` because the workspace manifests were already correct.
- Did not change validator logic because `g3rs validate-repo` passed and direct workspace validation passed once the lockfile matched the renamed dependency graph.
- Left `scripts/verify/__pycache__/_lib.cpython-312.pyc` unstaged because it is an unrelated generated file.

Key files for context
- `packages/rs/test/g3rs-test-file-tree-checks/Cargo.lock`
- `packages/rs/test/g3rs-test-file-tree-checks/Cargo.toml`
- `.githooks/pre-commit`

Verification
- `apps/guardrail3-rs/target/release/g3rs validate-repo` passed.
- Full loop over 180 adopted workspaces using `apps/guardrail3-rs/target/release/g3rs validate --path <workspace>` passed.

Next steps
- None for this fix.
