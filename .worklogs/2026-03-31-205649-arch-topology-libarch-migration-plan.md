# Arch Topology Libarch Migration Plan

**Date:** 2026-03-31 20:56
**Scope:** `.plans/todo/arch-topology-libarch-migration-handoff.md`

## Summary
Added a dedicated migration handoff that sequences the current `arch` rename into `topology`, reserves the `arch` name for a later new crate-architecture family, and only then dismantles `libarch` into that new meaning.

## Context & Problem
The current `RS-ARCH` family is really topology/governance, not the new crate-architecture meaning the project now wants. At the same time, `libarch` still contains layered-library rules, some of which are obsolete while others are only useful as generalized facade/privacy architecture concepts. Without an explicit migration order, it would be easy to blend both meanings of `arch` at once and create a repo full of ambiguous docs, tests, and runtime behavior.

## Decisions Made

### Enforce a two-stage rename-then-rebuild sequence
- **Chose:** Make the plan explicitly require a full `arch` -> `topology` rename before introducing the new `arch`.
- **Why:** The user explicitly wanted to avoid fusing the meanings. This is the cleanest way to keep handoffs, rule IDs, runtime sections, and docs coherent.
- **Alternatives considered:**
  - Introduce new `arch` in parallel with old `arch` and sort it out later — rejected because it guarantees semantic drift and ambiguous references.
  - Dismantle `libarch` first and rename later — rejected because it would move rules into an already overloaded family name.

### Separate obsolete `libarch` rules from generalizable concepts
- **Chose:** Mark the layered-shape rules as deletion candidates and the facade/privacy rules as migration candidates into the future new `arch`.
- **Why:** The old `api/core/infra` package layout is no longer policy, but the broader architectural ideas about facade ownership and internal/public boundaries still matter.
- **Alternatives considered:**
  - Delete all `libarch` rules immediately — rejected because it would throw away useful generalized architecture constraints.
  - Keep `libarch` as-is indefinitely — rejected because it preserves a dead structure-specific policy surface.

## Architectural Notes
This plan draws a hard line between:

- topology: placement, workspace legality, exact membership, zone overlap, global governance
- architecture: public/internal crate boundaries, facade ownership, generalized escalation

That separation is the whole point of the migration. The plan is intentionally sequencing-focused rather than implementation-heavy because the primary risk is semantic blending, not code mechanics.

## Information Sources
- `.plans/by_family/rs/arch.md`
- `.plans/by_family/rs/libarch.md`
- `apps/guardrail3/crates/app/rs/families/arch/README.md`
- `apps/guardrail3/crates/app/rs/families/libarch/README.md`
- user direction in this session about renaming old `arch` to `topology` first and only then creating the new `arch`

## Open Questions / Future Considerations
- Whether any part of current `RS-LIBARCH-01` survives as a generalized escalation policy in the new `arch`
- Whether `libarch` disappears entirely or survives temporarily as a compatibility family during migration

## Key Files for Context
- `.plans/todo/arch-topology-libarch-migration-handoff.md` — canonical staged migration plan
- `.plans/by_family/rs/arch.md` — current old-arch planning surface that will need rename and reinterpretation
- `.plans/by_family/rs/libarch.md` — current `libarch` ownership summary and eventual dismantling input
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — current topology semantics under the `arch` name
- `apps/guardrail3/crates/app/rs/families/libarch/README.md` — current layered-library semantics and the pieces likely to survive in generalized form

## Next Steps / Continuation Plan
1. Commit this plan as a standalone checkpoint so the migration sequencing is recorded before the code rename begins.
2. Rename the current live `arch` family to `topology` across runtime/model/CLI/docs/tests/rule IDs.
3. Verify lean `topology` execution and full workspace compile before introducing any new `arch` code.
