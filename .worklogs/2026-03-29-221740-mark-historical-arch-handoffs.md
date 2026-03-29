# Mark Historical Arch Handoffs

**Date:** 2026-03-29 22:17
**Scope:** `.plans/todo/check_review/test_hardening/35-arch-family-rewrite-agent-brief.md`, `.plans/todo/check_review/test_hardening/29-arch-agent-brief.md`, `.plans/2026-03-24-rs-arch-handoff.md`, `.plans/2026-03-21-160421-arch-model-walker.md`, `.plans/2026-03-21-153251-checkers-rs.md`

## Summary
Marked the stale historical `arch` handoff and architecture-plan files as explicitly superseded so they stop competing with the live `arch` README and rule plan. This closes the remaining doc-layer audit issue without rewriting historical content.

## Context & Problem
The earlier `arch` review found that several old handoff/plan files still looked active enough to mislead a cold reader, even after the current README and `.plans/todo/checks/rs/arch.md` were improved. They contained stale rule counts, obsolete paths, and older family shapes, but nothing in the file itself said “historical.”

That is a documentation bug, not just clutter. Future work can regress simply because an older brief looks concrete and current.

## Decisions Made

### Add explicit superseded headers instead of deleting history
- **Chose:** Put short “historical / superseded” notes at the top of the stale files, pointing readers at the current README and live rule plan.
- **Why:** This preserves useful historical context while removing ambiguity about what is current.
- **Alternatives considered:**
  - Delete the files — rejected because some still provide useful migration history.
  - Rewrite the old files to fully match current behavior — rejected because that would destroy the historical record and add low-value churn.

## Architectural Notes
No runtime behavior changed here. This is cold-start hygiene: the current `arch` source of truth is now easier to identify, and the stale documents are less likely to cause implementation drift.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/arch/README.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `.plans/todo/checks/rs/arch.md`
- The immediately preceding `arch` audit/fix worklogs from this session

## Open Questions / Future Considerations
- The historical files are now clearly labeled, but they still contain stale detail internally. That is acceptable as long as the superseded header remains prominent.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — current family source of truth
- `apps/guardrail3/crates/app/rs/README.md` — shared routed architecture contract
- `.plans/todo/checks/rs/arch.md` — live rule inventory
- `.plans/todo/check_review/test_hardening/35-arch-family-rewrite-agent-brief.md` — now explicitly historical
- `.plans/2026-03-24-rs-arch-handoff.md` — now explicitly historical

## Next Steps / Continuation Plan
1. No additional `arch` work is required from the original audit set unless new adversarial findings appear.
2. If future architecture cleanup changes the `arch` family again, update the current README and `.plans/todo/checks/rs/arch.md` first; leave these historical files as historical.
