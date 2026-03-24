# Prune Legacy TS Docs And Note Cargo Target Policy

**Date:** 2026-03-24 15:12
**Scope:** `.plans/todo/legacy/**`, `.plans/parallel-cargo-and-agent-builds.md`, `.plans/todo/generate/rs/release.md`

## Summary
Removed the stale TypeScript planning files that had been moved out of the active roadmap but were still sitting in the legacy tree. Added a standalone note for stable per-family Cargo target directories and tightened the Rust release generator plan so it spells out the validation-root release-domain contract.

## Context & Problem
The user asked to sort the entire dirty worktree into reasonable commit groups. The worktree contained one cluster that was purely documentation and did not belong to any specific Rust family implementation batch:
- deleted legacy TS planning documents that were already superseded by the Rust-only direction
- a new operational note about parallel Cargo builds and stable `CARGO_TARGET_DIR` usage
- a small clarification in the Rust release generator spec

Keeping these docs mixed into family hardening commits would make later history harder to read and would blur whether a given commit changed behavior or just planning/supporting guidance.

## Decisions Made

### Commit the legacy TS pruning separately
- **Chose:** Put the `.plans/todo/legacy/**` removals in this docs-only batch.
- **Why:** Those deletions are policy cleanup, not part of any active Rust family implementation.
- **Alternatives considered:**
  - Fold the deletions into a later family commit — rejected because they have no coupling to the family code changes.
  - Leave them uncommitted — rejected because the user explicitly asked to sort and commit the whole dirty tree.

### Treat Cargo target-dir guidance as repository operational doctrine
- **Chose:** Commit `.plans/parallel-cargo-and-agent-builds.md` in the same docs batch.
- **Why:** It is cross-family operating guidance for parallel Rust verification, not family-specific checker behavior.
- **Alternatives considered:**
  - Attach it to a cargo-family commit — rejected because it governs all family verification, not only `rs/cargo`.
  - Leave it as an untracked scratch note — rejected because it is useful repository guidance and already written.

### Keep the release generator clarification with the docs batch
- **Chose:** Include the small `.plans/todo/generate/rs/release.md` clarification here.
- **Why:** The change is a contract clarification only; there is no release-family code batch in the current dirty tree to pair it with.
- **Alternatives considered:**
  - Hold it for a later release-focused commit — rejected because no such commit exists in this worktree slice.

## Architectural Notes
- The repo direction remains Rust-only; deleting legacy TS plan files makes that current direction more visible.
- The Cargo target-dir note records operational policy for parallel agent work:
  - stable family target dirs
  - no competing shared `target/debug` users
  - narrow commands before broad commands
- The release generator spec now states that release-domain enablement is controlled by the validation root's Rust config contract.

## Information Sources
- `AGENTS.md` — current repo direction and worklog requirements
- `.plans/todo/typescript/**` and `.plans/todo/legacy/**` state from prior session work
- `.plans/todo/generate/rs/release.md`
- local dirty worktree inventory from `git status --short` and `git diff --stat`

## Open Questions / Future Considerations
- The Rust validation cutover and family hardening work still need several code commits after this docs cleanup batch.
- The parallel Cargo build note is operational guidance; if the tool grows its own build-coordination automation later, the note may need to move or shrink.

## Key Files for Context
- `.plans/parallel-cargo-and-agent-builds.md` — stable family-target guidance for parallel Rust verification
- `.plans/todo/generate/rs/release.md` — release generator contract, now clearer about validation-root ownership
- `AGENTS.md` — repository direction and commit/worklog rules
- `.worklogs/2026-03-24-145614-freeze-rust-validation-cutover-spec.md` — recent Rust runtime planning context

## Next Steps / Continuation Plan
1. Commit the hooks family hardening batch with its updated hook docs and briefs.
2. Commit the cargo family hardening batch with the reconciled multi-root code and plan updates.
3. Continue through the remaining family batches until the Rust worktree is clean.
