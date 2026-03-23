# Finalize Legacy Plan Moves

**Date:** 2026-03-23 13:54
**Scope:** `.plans/todo/` source-side deletions corresponding to already-archived legacy plan moves

## Summary
Finalized the archival move by staging and committing the old source-path deletions that correspond to the new files already added under `.plans/todo/legacy/`. This closes the gap left by the prior narrow `git add` so the archive move is represented coherently in git history.

## Context & Problem
The prior archive commit added the new `legacy/` tree and active-plan updates, but because only specific paths were staged, the old source-path deletions under `.plans/todo/` remained unstaged. That left the repo in an inconsistent state where archived copies existed but the original paths still appeared as pending deletions. The fix is mechanical but important for keeping the planning cleanup understandable in history.

## Decisions Made

### Commit the source-side deletions separately instead of amending
- **Chose:** Create a short follow-up commit with the corresponding deletions.
- **Why:** The prior commit is already created and the project rules prohibit amend unless explicitly requested. A small cleanup commit is the safest way to finish the move cleanly.
- **Alternatives considered:**
  - Amend the previous commit — rejected because no explicit amend request was given.
  - Leave the deletions unstaged — rejected because that would leave the archive move incomplete and confusing.

## Architectural Notes
This commit does not change planning content or code semantics. It only completes the file-move representation for the archival refactor of top-level plan material into `.plans/todo/legacy/`.

## Information Sources
- `.worklogs/2026-03-23-135318-archive-audit-plans-and-followups.md` — prior archive/extraction rationale
- `git diff --name-status -- .plans/todo .plans/todo/checks .plans/todo/legacy` — confirmed the remaining delta was only source-path deletions for moved files

## Open Questions / Future Considerations
- None for this cleanup itself. The next work should return to reviewing active plans under `.plans/todo/checks/` in small adversarial batches.

## Key Files for Context
- `.plans/todo/legacy/README.md` — explains the archive structure and where live requirements were carried forward
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` — active follow-up backlog extracted from archived audits
- `.worklogs/2026-03-23-135318-archive-audit-plans-and-followups.md` — main archival/extraction rationale

## Next Steps / Continuation Plan
1. Verify the repo is clean after this deletion-only follow-up commit.
2. Continue adversarial review against active plans under `.plans/todo/checks/` in small batches.
3. After each batch, record only concrete plan-vs-code gaps that should feed the next implementation round.
