# Tighten Rust Family Mapper Boundaries

**Date:** 2026-03-26 13:24
**Scope:** `apps/guardrail3/crates/app/rs/README.md`

## Summary
Refined the Rust scope/mapping architecture plan in `app/rs/README.md` so the layer boundaries are explicit. The document now states that `family_selection` owns all family enablement and implication decisions, `FamilyMapper` is a pure mapping layer, and family routes should carry narrow owned views rather than raw placement facts.

## Context & Problem
The previous README draft was directionally better than family-local root discovery, but it still leaked responsibilities across layers. In particular, the mapper was described as doing applicability work, and the route examples carried full `RustRootFact` payloads. That would have recreated the same architectural problem in a new place: families would still effectively depend on placement internals, and the mapping layer would become another policy blob instead of a narrow projection step.

The user clarified two design constraints:
- selection must fully resolve enablement before mapping
- file scoping should be a generic mapper capability for every family, not a test-only special case

That made the route-shape question the main architectural decision to settle.

## Decisions Made

### Separate `family_selection` from `FamilyMapper` completely
- **Chose:** `family_selection` now owns requested-family resolution, enabled-family filtering, and implied-family expansion. `FamilyMapper` now owns only per-family route projection from shared scope into typed routes.
- **Why:** This makes the mapper literally a mapper, which is the cleaner conceptual boundary and matches the user’s intent. It also prevents configuration/applicability logic from drifting into mapping implementations.
- **Alternatives considered:**
  - Let `FamilyMapper` perform config-driven applicability during mapping — rejected because that mixes family selection with route projection and makes mapping semantically stateful.
  - Keep selection and mapping in one module with separate functions — rejected because the design goal here is to create a hard boundary that families cannot quietly blur again.

### Use narrow owned route views instead of ids-only or full shared facts
- **Chose:** The README now recommends route-local owned views such as `RsRootView` and `RsArchRootView`.
- **Why:** This keeps family inputs precise without forcing families to reach back into shared scope. It is the strongest practical boundary that does not create awkward lookup plumbing or lifetime-heavy borrowed APIs.
- **Alternatives considered:**
  - Ids only — rejected because families would still need a lookup path back into shared scope for common fields like `rel_dir`, which would reintroduce the leak indirectly.
  - Borrowed views — rejected for now because they make the API and lifetimes more fragile without adding enough architectural value at the planning stage.
  - Full copied `RustRootFact` values — rejected because that would make placement’s internal shape the de facto family API.

### Make file scoping a generic mapper responsibility
- **Chose:** `scoped_files` remains in the mapper layer as a generic mapped-family capability and appears on all example Rust family routes.
- **Why:** The user explicitly wanted this to apply to every family. Keeping it generic avoids reintroducing family-local file filtering logic.
- **Alternatives considered:**
  - Keep `scoped_files` only on `RsTestRoute` — rejected because that would imply test-specific special handling.
  - Keep raw `scoped_files` outside routes and let families filter locally — rejected because that weakens the whole purpose of external typed mapping.

## Architectural Notes
This README now describes a stricter pipeline:

`ProjectTree -> placement -> family_selection -> FamilyMapper -> family route -> family orchestrator -> rule inputs`

The important effect is that families should eventually stop receiving either the whole shared scope or raw placement facts. They should receive only the shaped inputs they actually own. That makes `placement` evolvable without silently changing family contracts.

The route examples are still illustrative, not the final code API. In particular, overlap and root-input-failure entries may still want narrow view types rather than bare ids depending on how hard the final boundary should be.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md` — active architecture plan being refined
- `.worklogs/2026-03-26-130434-app-rs-family-mapper-plan-readme.md` — prior README checkpoint
- User clarifications in this session about:
  - `family_selection` fully resolving enablement
  - `FamilyMapper` being pure mapping
  - file scoping applying to every family

## Open Questions / Future Considerations
- Whether overlaps and root-input-failure data should be ids-only or narrow owned views in final route structs.
- Whether every family really needs `scoped_files` on its route type, or whether some families should receive a common optional file-scope wrapper.
- How much of current `runtime.rs` should move into `family_selection` vs `family_mapper` vs a higher-level Rust orchestration crate.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — current target architecture for shared Rust scope and family mapping
- `apps/guardrail3/crates/app/rs/runtime.rs` — current runtime entrypoint that still owns logic slated for migration
- `apps/guardrail3/crates/app/rs/placement/src/lib.rs` — existing shared placement entrypoint
- `.worklogs/2026-03-26-130434-app-rs-family-mapper-plan-readme.md` — previous planning checkpoint for this document
- `.worklogs/2026-03-26-132416-rs-family-mapper-boundary-tightening.md` — this record of the stricter boundary decision

## Next Steps / Continuation Plan
1. Re-review `apps/guardrail3/crates/app/rs/README.md` once more specifically for the remaining route-shape question around overlap/failure ids versus narrow view types.
2. Translate the README plan into concrete crates or modules under `apps/guardrail3/crates/app/rs/`, starting with `family_selection/` and `family_mapper/` scaffolding.
3. Refactor `apps/guardrail3/crates/app/rs/runtime.rs` so family selection happens once before any family-specific mapping.
4. Migrate one family first, preferably `arch` or `test`, to validate that the route-view design actually works in code and does not force families to reach back into placement internals.
