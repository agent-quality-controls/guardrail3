Summary
- Staged and pushed the remaining plan-file cleanup after the G3TS init and G3RS version work.
- This commit is intentionally separate from the code change.

Decisions made
- Kept the cleanup separate so the init/version implementation stays reviewable as its own commit.
- Included the remaining debug plan because the user explicitly asked to stage and push the remaining worktree state.

Key files for context
- `.plans`
- `.plans/2026-05-18-103035-g3ts-validate-repo-debug.md`

Verification
- Pre-commit hook runs on commit.

Next steps
- None for this cleanup.
