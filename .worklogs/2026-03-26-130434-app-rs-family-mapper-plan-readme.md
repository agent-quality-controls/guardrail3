# App RS Family Mapper Plan README

**Date:** 2026-03-26 13:04
**Scope:** `apps/guardrail3/crates/app/rs/README.md`

## Summary
Added and refined a new `app/rs` README that defines the target architecture for shared Rust root scope plus a typed external `FamilyMapper` layer. The document now makes the intended boundary between `placement`, `selection`, `family_mapper`, and family crates explicit.

## Context & Problem
Recent work on `rs/test` exposed the same architectural bug class that had already shown up in `arch`: families were still rediscovering Rust roots and deciding local scope on their own. That creates drift in exclusions, ownership, and applicability. The user asked for a plan document at `apps/guardrail3/crates/app/rs/README.md`, then corrected the design toward a typed `FamilyMapper` model with namespaced methods like `map_rs_test`.

The first draft of the README still had a few leaks:
- family-routing semantics were still implied in the shared root model
- raw `usize` references were used instead of typed ids
- `scoped_files` still had an ambiguous path through the architecture
- family selection and family mapping were not clearly separated

## Decisions Made

### Introduce a typed `FamilyMapper` layer
- **Chose:** Document an external typed family-mapper layer rather than generic routing.
- **Why:** The user wanted the mapper to directly map shared scope into family-specific top-level inputs, and typed per-family inputs make the ownership boundary explicit.
- **Alternatives considered:**
  - Generic untyped routing bags — rejected because they make it too easy for families to reinterpret routing locally.
  - Family-local mapping only — rejected because it recreates the drift problem the plan is meant to remove.

### Keep `placement` structural only
- **Chose:** Remove `owner_families` from the shared placement model in the plan.
- **Why:** Shared scope should expose structural facts only, not family-routing policy.
- **Alternatives considered:**
  - Keep family ownership hints in placement — rejected because it leaks mapping semantics back into the shared scope layer.

### Separate selection from mapping
- **Chose:** Describe family selection as a step before `FamilyMapper::new(...)`.
- **Why:** Picking selected families and mapping scoped roots into typed family inputs are related but distinct concerns.
- **Alternatives considered:**
  - Let `FamilyMapper` also own selected-family resolution — rejected because it overloads the mapper with another orchestration concern.

### Make scoping flow through one place
- **Chose:** Document `scoped_files` as mapper-resolved into typed mapped subsets, not as an extra raw argument families reinterpret independently.
- **Why:** A single scoping path is necessary if the new architecture is meant to eliminate drift.
- **Alternatives considered:**
  - Keep passing raw `scoped_files` directly to families — rejected because it preserves a second local scoping channel.

## Architectural Notes
The README now describes this intended pipeline:

`ProjectTree -> placement -> selection -> FamilyMapper -> family::check(...)`

And the family boundary is:
- families receive shared scope plus typed mapped input
- families may do family-local parsing, normalization, and rule fan-out
- families must not rediscover live Rust roots or reroute roots locally

The document is a plan, not an implementation. No code behavior changed in this step.

## Information Sources
- `AGENTS.md` — worklog and commit rules
- `apps/guardrail3/crates/app/rs/README.md` — target architecture document
- recent `rs/test` and `arch` architectural discussion in this session
- `apps/guardrail3/crates/app/rs/runtime.rs` — current runtime orchestration shape
- `apps/guardrail3/crates/app/rs/placement/src/lib.rs` and `roots.rs` — current shared root-scope seed

## Open Questions / Future Considerations
- The README still needs eventual implementation backing in `placement`, `selection`, and `family_mapper`.
- We still need to choose whether family inputs carry ids only or small borrowed references into shared scope.
- The plan should eventually be mirrored in tests once the mapper layer is implemented.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — the architecture plan for shared Rust scope and typed family mapping
- `apps/guardrail3/crates/app/rs/runtime.rs` — current product entrypoint that will need to become thinner
- `apps/guardrail3/crates/app/rs/placement/src/lib.rs` — current shared root-scope seed
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — live-root discovery source
- `.worklogs/2026-03-26-115739-rs-test-discovery-orchestrator-refactor.md` — prior refactor moving `rs/test` discovery into the right layer
- `.worklogs/2026-03-26-120231-rs-test-all-cargo-roots-discovery.md` — prior root-discovery change that exposed the need for shared scope

## Next Steps / Continuation Plan
1. Implement the shared scope API shape in `placement`, including typed root and failure ids.
2. Add a `selection` or equivalent layer that resolves selected families before mapping.
3. Create the `family_mapper` crate/module and define typed inputs for `rs/arch` and `rs/test` first.
4. Refactor `runtime.rs` to use shared scope plus `FamilyMapper`.
5. Remove family-local root discovery from `arch` and `test`, then add regressions proving they agree on root scope.
