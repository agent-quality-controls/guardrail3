# Update TypeScript Family Index State

**Date:** 2026-03-30 11:58
**Scope:** `.plans/by_family/ts/README.md`

## Summary
Updated the TypeScript family index README so it no longer describes the TS family tree as placeholder-only. The index now states that every canonical TS family file has a rule inventory and that the next step is family-by-family demotion of the old `.plans/todo/checks/ts/*.md` ledgers.

## Context & Problem
After reconciling the first and second TS family waves, the by-family TS directory itself had become materially stronger than the top-level TS README claimed. The index still described the files as an initial placeholder pass, which would immediately reintroduce confusion for the next reader.

## Decisions Made

### Update the TS index instead of leaving it stale
- **Chose:** Add a short reconciliation-state block to `.plans/by_family/ts/README.md`.
- **Why:** The index should match the actual state of the family files so the by-family tree stays trustworthy as the current planning surface.
- **Alternatives considered:**
  - Leave the README untouched until the old TS ledgers are superseded — rejected because it would make the index obviously stale right after the family-plan work.

## Architectural Notes
The TS planning surface is now in the same broad shape as Rust:
- one family index under `.plans/by_family/ts/README.md`
- one file per family under `.plans/by_family/ts/*.md`

The difference is that TS is still transitional:
- family files now carry rule inventories and code mapping
- old `.plans/todo/checks/ts/*.md` files are still the detailed ledgers
- superseded banners have not been added yet

## Information Sources
- `.plans/by_family/ts/README.md`
- `.plans/by_family/ts/*.md`
- `.worklogs/2026-03-30-115529-reconcile-first-wave-ts-family-plans.md`
- `.worklogs/2026-03-30-115709-reconcile-second-wave-ts-family-plans.md`

## Open Questions / Future Considerations
- The next doc cleanup is to supersede the old TS ledgers one family at a time rather than all at once.
- TS still lacks a shared architecture README analogous to Rust’s shared `rs/README.md`.

## Key Files for Context
- `.plans/by_family/ts/README.md` — TS family index and current transition-state authority order.
- `.plans/by_family/ts/code.md` — representative live source-scan family summary.
- `.plans/by_family/ts/hexarch.md` — representative architecture family summary.
- `.plans/by_family/ts/arch.md` — representative planned-family summary.
- `.worklogs/2026-03-30-115529-reconcile-first-wave-ts-family-plans.md` — first-wave TS family reconciliation.
- `.worklogs/2026-03-30-115709-reconcile-second-wave-ts-family-plans.md` — second-wave TS family reconciliation.

## Next Steps / Continuation Plan
1. Start adding superseded banners to `.plans/todo/checks/ts/*.md` one family at a time, beginning with the families that now have the strongest by-family summaries.
2. Decide whether TS needs a shared architecture/index README beyond the family index, analogous to Rust’s shared `rs/README.md`.
3. After the old ledgers are demoted, use the by-family files as the primary surface for future TS family stabilization work.
