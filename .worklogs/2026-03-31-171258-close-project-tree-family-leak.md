# Close Remaining Family ProjectTree Leak

**Date:** 2026-03-31 17:12
**Scope:** `apps/guardrail3/crates/domain/project-tree/src/lib.rs`, shared Rust structure/placement/ownership/legality/mapper crates, `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions_common/*`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/*`

## Summary
Removed the remaining effective `ProjectTree` leak from Rust family code. The fix replaces the `RsProjectSurface -> ProjectTree` deref bridge with an explicit read-only `ProjectTreeView` trait for shared pre-family stages, keeps family/runtime ingress on `RsProjectSurface`, and cleans the remaining family assertion helpers that still referenced the domain tree type.

## Context & Problem
The previous "strict migration" pass was not actually strict. Rust families no longer imported `ProjectTree` directly in most places, but `RsProjectSurface` still deref-coerced into `ProjectTree`, so family code still effectively had raw-tree access and shared helpers still required the domain type.

That broke the intended architecture in two ways:
- the type boundary was fake, because families could still satisfy any `&ProjectTree` API through deref
- I reported the migration as done even though the real capability boundary had not changed

This pass exists to close that gap for real and leave a concrete state that matches the architecture language in the plans.

## Decisions Made

### Replace deref-based compatibility with an explicit read-only tree trait
- **Chose:** add `ProjectTreeView` to the domain tree crate and make shared pre-family crates consume `&dyn ProjectTreeView`.
- **Why:** shared structure/placement/ownership/legality/mapper code still needs generic tree queries, but families must no longer gain raw `ProjectTree` capability by type coercion.
- **Alternatives considered:**
  - Keep `Deref<Target = ProjectTree>` on `RsProjectSurface` — rejected because it preserves the exact leak we were trying to remove.
  - Rewrite all families immediately to stop using any surface/tree object at all — rejected for this pass because the current plan still allows local discovery inside the routed legal surface; the urgent bug was the fake raw-tree boundary.

### Keep family ingress on `RsProjectSurface`
- **Chose:** keep family runtimes on `RsProjectSurface`, but make that surface implement `ProjectTreeView` explicitly rather than behave like a `ProjectTree`.
- **Why:** this matches the current plan language: shared legality runs first, mapper slices legal local surfaces, and families operate inside those routed legal surfaces.
- **Alternatives considered:**
  - Revert family APIs back to `ProjectTree` — rejected because it collapses the boundary again.
  - Add more compatibility wrappers around test helpers only — rejected because it would hide the real mismatch instead of fixing it.

### Clean the remaining test/assertion crates to the same boundary
- **Chose:** switch cargo assertion helpers and the last hexarch assertion aliases off `guardrail3_domain_project_tree::ProjectTree`.
- **Why:** even if those were test-only, claiming "families no longer touch domain tree" would still have been false while they remained.
- **Alternatives considered:**
  - Leave the test-only aliases in place — rejected because the user explicitly asked for strict separation, not runtime-only separation.

## Architectural Notes
- `ProjectTree` remains the runtime/orchestrator snapshot type.
- `ProjectTreeView` is now the shared read-only interface consumed before family execution.
- `RsProjectSurface` is an owned routed slice and implements `ProjectTreeView`, but it no longer derefs into the domain tree.
- Shared stages now depend on generic tree-view capability instead of the concrete tree type:
  - `rs/placement`
  - `rs/ownership`
  - `rs/structure`
  - `rs/legality`
  - `rs/family_mapper`
- Rust family crates no longer import `guardrail3_domain_project_tree` directly.

## Information Sources
- `AGENTS.md`
- `.plans/todo/checks/rs/arch.md`
- `.plans/by_family/rs/arch.md`
- `.worklogs/2026-03-31-164230-strict-rs-surface-migration.md`
- local code inspection with `rg` over family crates for `guardrail3_domain_project_tree`, `ProjectTree`, and `RsProjectSurface`

## Open Questions / Future Considerations
- Families still do local discovery against `RsProjectSurface`. That matches the current routed-surface plan, but if the architecture later tightens further, the next step would be replacing surface reads with even narrower family-specific routed facts.
- Runtime/orchestrator/hook crates still legitimately use the real `ProjectTree`; that is intentional and unchanged.
- Full workspace `cargo test --no-run` still has the unrelated TypeScript missing-module failure outside this Rust migration lane.

## Key Files for Context
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs` — `ProjectTreeView` trait and canonical tree queries
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — `RsProjectSurface` capability boundary, no more deref bridge
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — mapper now depends on `ProjectTreeView`
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — shared root discovery over the tree-view trait
- `apps/guardrail3/crates/app/rs/legality/src/lib.rs` — legality stage over the tree-view trait
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions_common/src/lib.rs` — family test helper corrected to surface-only
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/Cargo.toml` — last family assertion dependency cleanup
- `.worklogs/2026-03-31-164230-strict-rs-surface-migration.md` — prior incorrect checkpoint this work supersedes

## Next Steps / Continuation Plan
1. If the boundary is tightened again, design family-specific routed fact payloads so families stop reading from `RsProjectSurface` directly and consume only precomputed local slices.
2. Re-run the same family-side grep whenever a migration claims strict separation: `rg "guardrail3_domain_project_tree|ProjectTreeView" apps/guardrail3/crates/app/rs/families`.
3. Keep future test-support and assertion crates on the same surface boundary so test-only helpers do not silently reintroduce domain-tree dependencies.
