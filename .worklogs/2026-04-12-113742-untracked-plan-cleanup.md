# Untracked Plan Cleanup

## Summary

Committed the two leftover untracked plan files so the worktree is clean apart from intentional code changes. This keeps the planning artifacts that were created during the apparch exploration instead of leaving them floating outside version control.

## Decisions made

- Committed the plan files as-is instead of deleting them.
  - Why: both are real planning notes for possible future work and neither conflicts with the package migration work.
- Kept this as a separate commit from the package ID normalization.
  - Why: planning-file cleanup should not be mixed into the package-ID fix commit.

## Key files for context

- `.plans/2026-04-11-144026-apparch-rule-family.md`
- `.plans/2026-04-11-213610-apparch-nextjs-frontend.md`
- `.worklogs/2026-04-12-113742-untracked-plan-cleanup.md`

## Next steps

- If `apparch` becomes active work, re-audit these notes against the current Rust-only direction before implementing anything.
- If `apparch` is abandoned, remove the plans explicitly in a later cleanup commit instead of leaving them untracked again.
