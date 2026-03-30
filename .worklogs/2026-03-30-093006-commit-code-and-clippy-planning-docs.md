# Commit Code And Clippy Planning Docs

**Date:** 2026-03-30 09:30
**Scope:** `.plans/todo/check_review/test_hardening/36-clippy-full-sweep-agent-brief.md`, `apps/guardrail3/crates/app/rs/families/code/FIXES.md`, `apps/guardrail3/crates/app/rs/families/code/EXPANSION.md`

## Summary
Committed the remaining untracked planning and status docs so the repository ends the sweep in a fully clean state. These files are not incidental scratch notes; they are now part of the working record for the completed `RS-CODE` and `RS-CLIPPY` hardening lanes.

## Context & Problem
After the code and clippy cleanup commits, the only remaining dirty files were three untracked Markdown documents:
- a clippy full-sweep agent brief used to scope the clippy lane
- `RS-CODE` fixes and expansion records produced during the adversarial hardening pass

Leaving them untracked would make the repo appear clean only by omission. The user explicitly asked for untracked material to be reviewed and either kept intentionally or cleaned up. These documents contain durable context, not throwaway notes.

## Decisions Made

### Keep the clippy agent brief
- **Chose:** Commit the clippy lane handoff/brief under `.plans/todo/check_review/test_hardening/`.
- **Why:** It captures the exact ownership, completion criteria, and non-negotiable constraints that shaped the successful clippy sweep.
- **Alternatives considered:**
  - Delete it as temporary coordination text — rejected because it now documents a real, completed execution plan.

### Keep `RS-CODE` fixes and expansion records
- **Chose:** Commit both `apps/guardrail3/crates/app/rs/families/code/FIXES.md` and `EXPANSION.md`.
- **Why:** `FIXES.md` records concrete implementation bugs and how to think about them; `EXPANSION.md` separates true policy ideas from correctness defects. That distinction is useful for future attacks and future rule work.
- **Alternatives considered:**
  - Leave them untracked for local reference only — rejected because they are already being used as source-of-truth documents during the hardening work.
  - Collapse them into one file — rejected because bug fixes and policy expansions are intentionally different classes of work.

## Architectural Notes
This commit does not change runtime behavior. It closes the repository hygiene loop after the cleanup sweep by ensuring that the planning/status documents which informed the work are preserved in history. That matters in this repo because guardrail work depends heavily on explicit written intent, not just code.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/FIXES.md`
- `apps/guardrail3/crates/app/rs/families/code/EXPANSION.md`
- `.plans/todo/check_review/test_hardening/36-clippy-full-sweep-agent-brief.md`
- `.worklogs/2026-03-29-233650-close-rs-code-proof-gaps.md`
- `.worklogs/2026-03-30-091513-clippy-full-sweep-cleanup.md`

## Open Questions / Future Considerations
- If `RS-CODE` adopts any items from `EXPANSION.md`, `code.md` should be updated in the same change so the contract and implementation stay aligned.
- If future family sweeps use agent briefs like the clippy one, those briefs should either be committed deliberately or deleted deliberately; they should not be left as ambiguous untracked files.

## Key Files for Context
- `.plans/todo/check_review/test_hardening/36-clippy-full-sweep-agent-brief.md` — exact ownership and done-criteria for the clippy sweep
- `apps/guardrail3/crates/app/rs/families/code/FIXES.md` — concrete `RS-CODE` correctness backlog/history
- `apps/guardrail3/crates/app/rs/families/code/EXPANSION.md` — separated policy-expansion ideas for later consideration
- `.worklogs/2026-03-30-092818-finish-clippy-owned-rs-code-tail.md` — closing context for the clippy-owned `RS-CODE` tail

## Next Steps / Continuation Plan
1. Re-run `git status --short` to confirm the tree is fully clean.
2. Re-run final repo-root validation for `code`, `clippy`, and `arch` on the committed tree.
3. Report the final commit set and the adversarial verification results.
