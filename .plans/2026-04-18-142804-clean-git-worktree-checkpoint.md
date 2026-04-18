## Goal

Make the repository git-clean without losing any live work by converting the current working tree into a checkpoint commit.

## Approach

1. Confirm the dirty tree is a coherent working snapshot.
   - Verify active validation still passes on representative roots.
   - Confirm the old `apps/guardrail3` tree is already archived under `legacy/` and the current diff is tracked deletion cleanup, not an accidental move.

2. Preserve everything instead of reverting.
   - Stage the full current working tree, including tracked deletions, modified package files, and untracked plans/worklogs.
   - Commit as a checkpoint so no active work is lost.

3. Verify the result.
   - Check `git status --short` is empty after commit.

## Key decisions

- Use a checkpoint commit instead of selective revert.
  - Reason: the dirty tree is large and internally coherent, and the user asked to clean git rather than discard work.
- Keep the old app deletion in the commit.
  - Reason: the archived replacement already exists under `legacy/`, and leaving `apps/guardrail3` as a tracked deletion keeps the worktree permanently dirty.

## Files to modify

- `.worklogs/2026-04-18-142804-clean-git-worktree-checkpoint.md`
- the current tracked and untracked worktree state via git staging
