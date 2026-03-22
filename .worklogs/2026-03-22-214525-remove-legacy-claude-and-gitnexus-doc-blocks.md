# Remove Legacy CLAUDE And GitNexus Doc Blocks

**Date:** 2026-03-22 21:45
**Scope:** `AGENTS.md`, `CLAUDE.md`

## Summary
Removed the embedded GitNexus managed documentation blocks from the handoff docs and deleted `CLAUDE.md` entirely. This stops the recurring “dirty `CLAUDE.md`” problem caused by auto-updated GitNexus stats touching a file that should no longer exist in the active workflow.

## Context & Problem
`CLAUDE.md` kept reappearing as a modified file even when no one intentionally changed it. The actual diff showed that the only mutation was inside the `<!-- gitnexus:start --> ... <!-- gitnexus:end -->` block, where GitNexus refreshed repository stats. Since `CLAUDE.md` was already obsolete and `AGENTS.md` was the real source of truth, keeping both the file and the duplicated GitNexus blocks was pure churn.

## Decisions Made

### Delete `CLAUDE.md` instead of keeping it as a historical stub
- **Chose:** remove `CLAUDE.md` from the repo.
- **Why:** the user explicitly asked for it gone, and leaving a deprecated stub around invited more accidental dirt and misleading references.
- **Alternatives considered:**
  - Keep `CLAUDE.md` as a short static pointer — rejected because it would still be redundant once `AGENTS.md` is the only handoff doc.
  - Keep it and only strip the GitNexus block — rejected because the user asked to remove the file itself.

### Remove the GitNexus blocks from the handoff docs
- **Chose:** strip the whole managed GitNexus section from `AGENTS.md` and delete the same section by deleting `CLAUDE.md`.
- **Why:** duplicated auto-managed blocks create noisy dirty state and do not belong in the human-maintained handoff doc. If GitNexus is needed, it can be queried directly from CLI/MCP instead of living as mutable stats in markdown.
- **Alternatives considered:**
  - Keep the block only in `AGENTS.md` — rejected because the user asked to remove the GitNexus blocks.
  - Regenerate and commit the block periodically — rejected because that preserves the same churn.

### Remove stale `CLAUDE.md` wording from `AGENTS.md`
- **Chose:** delete or neutralize the remaining lines in `AGENTS.md` that treated `CLAUDE.md` as a legacy reference.
- **Why:** once `CLAUDE.md` is gone, the source-of-truth doc should not keep pointing at a nonexistent file.
- **Alternatives considered:**
  - Leave the references as historical commentary — rejected because they would immediately become misleading.

## Architectural Notes
This is a documentation hygiene change only. No checker code or rule behavior was modified. The practical result is that the repo’s handoff flow is now simpler:
- `AGENTS.md` is the only handoff/source-of-truth doc
- there is no duplicate legacy handoff file
- there are no embedded auto-managed GitNexus stats blocks in the handoff docs

## Information Sources
- `git diff -- CLAUDE.md` — confirmed the recurring dirt was only the GitNexus stats block.
- `AGENTS.md` — current source-of-truth handoff document.
- `.worklogs/2026-03-22-213530-complete-code-family.md` — latest implementation checkpoint before this cleanup.
- `.worklogs/2026-03-22-203352-finish-rust-check-test-hardening.md` — recent context on intentionally excluding unrelated dirty files from scoped commits.

## Open Questions / Future Considerations
- There are still older docs, plans, tests, and help text elsewhere in the repo that mention `CLAUDE.md` or old `R49` behavior. This cleanup intentionally did not sweep those broader historical references.
- If the repo later wants GitNexus usage guidance, that guidance should live in a stable hand-maintained doc, not an auto-mutating stats block.

## Key Files for Context
- `AGENTS.md` — the remaining source-of-truth handoff document.
- `.worklogs/2026-03-22-213530-complete-code-family.md` — previous implementation checkpoint.
- `.worklogs/2026-03-22-203352-finish-rust-check-test-hardening.md` — recent repo-state context.

## Next Steps / Continuation Plan
1. Leave the broader historical `CLAUDE.md` references alone unless the user asks for a wider documentation cleanup sweep.
2. Move on to the next Rust family implementation/audit task without carrying `CLAUDE.md` forward.
3. If dirty doc state recurs, check for other auto-managed markdown blocks before assuming a human edit happened.
