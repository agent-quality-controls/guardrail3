# Add Todo Audit And Check Inventory Plans

**Date:** 2026-03-22 14:27
**Scope:** `.plans/todo/**`

## Summary
Committed the remaining active planning tree under `.plans/todo`, including audit writeups, check inventories, code-fix notes, and the current checker architecture document.

## Context & Problem
After committing the top-level archive and the by-file references, the final untracked planning material was the active todo tree. This is the most operational part of the planning corpus and includes the rule inventories we are actively using for the Rust family migration work.

## Decisions Made

### Commit the full `.plans/todo` tree as one operational planning group
- **Chose:** Batch `.plans/todo/**` into a single commit.
- **Why:** These files form one coherent working set: audits, check inventories, and pending implementation notes. Splitting them further would add history churn without improving clarity much.
- **Alternatives considered:**
  - Separate audit docs from checks docs — rejected because the audit findings and check inventories directly inform each other.
  - Leave legacy TS todo material out — rejected because the user asked to commit all of the exposed planning documents, and this commit is archival/organizational rather than a statement of active product scope.

## Architectural Notes
This commit preserves the current planning substrate that the new Rust checks architecture is being built from. It includes the checker architecture document, family inventories, and earlier audit passes that explain why certain rule families exist.

## Information Sources
- `git status --short`
- `find .plans/todo -type f | sort`
- `.worklogs/2026-03-22-142625-plans-top-level-archive.md`
- `.worklogs/2026-03-22-142710-plans-by-file-reference.md`

## Open Questions / Future Considerations
- Some todo documents still cover old TypeScript scope, but they are being preserved as planning history rather than current direction.
- After this commit, the worktree should be clean of the exposed `.plans` backlog.

## Key Files for Context
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — active checker architecture
- `.plans/todo/checks/rs/*` — Rust rule inventories
- `.plans/todo/checks/hooks/*` — hook inventories
- `.plans/todo/audit/*` — prior audit findings and gap analysis
- `.plans/todo/NEW_CHECKS.md` — broader outstanding check ideas

## Next Steps / Continuation Plan
1. Re-check `git status` to confirm the exposed planning backlog is fully committed.
2. Continue product work on the next Rust family (`rs/clippy` or `rs/deny`) without the repository being cluttered by historical untracked plans.
