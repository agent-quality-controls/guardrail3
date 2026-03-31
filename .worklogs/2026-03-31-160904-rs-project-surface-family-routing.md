# Route Families Through `RsProjectSurface` (No Raw `ProjectTree` Entrypoints)

**Date:** 2026-03-31 16:09
**Scope:** `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/{lib.rs,views.rs}`, Rust family runtime entry `lib.rs` files, related family test helpers/callers, `apps/guardrail3/crates/app/hooks/mod.rs`, hooks/test Cargo manifests

## Summary
Implemented the mapper/runner surface contract so Rust family runtime entrypoints no longer accept `&ProjectTree` directly. Runners now build and pass `RsProjectSurface` instances from mapped routes (global or workspace-local shape), and family/test callsites were updated accordingly.

## Context & Problem
The architecture target in this cycle was explicit: families should consume mapper/legality-shaped inputs, not direct `ProjectTree` handles from runtime. Runtime still passed raw tree references into all families, which kept rule execution coupled to full-tree access and broke the intended structure-legality-mapper boundary.

## Decisions Made

### Introduced and used `RsProjectSurface` as family runtime input
- **Chose:** make family runtime `check(...)` entrypoints take `&RsProjectSurface` instead of `&ProjectTree`.
- **Why:** removes direct `ProjectTree` runtime ingress to families and enforces the route-surface contract at API level.
- **Alternatives considered:**
  - Keep `&ProjectTree` and rely on convention only — rejected because it does not enforce architecture.
  - Move to a brand-new trait abstraction in one step — rejected for this checkpoint as a bigger refactor than needed.

### Build surfaces in runners from route facts
- **Chose:** update `runners.rs` to construct surfaces per family:
  - global families: full-surface (`from_tree`)
  - workspace-local families: route-root + family-file scoped surfaces (`from_route_scope`)
- **Why:** keeps family input construction centralized in runtime/mapper boundary rather than rediscovery in family logic.
- **Alternatives considered:**
  - Keep one full surface for all families — rejected for local families because it weakens route slicing.
  - Push surface construction down into each family — rejected because that recreates duplicated discovery logic.

### Align tests and helper callsites with surface-based signatures
- **Chose:** patch unit-test helpers and direct test callsites to wrap trees with `RsProjectSurface::from_tree(...)`.
- **Why:** prevents test-only drift back to old `ProjectTree`-based entrypoints.
- **Alternatives considered:**
  - Add overloaded compatibility entrypoints for `ProjectTree` — rejected to avoid preserving dual API contracts.

### Keep build green for feature-sliced binaries
- **Chose:** gate runner helper functions with feature cfgs to avoid dead-code errors under lean feature builds (for example `family-clippy` only).
- **Why:** project uses strict linting (`-D`), so helpers must not compile as unused in minimal feature combinations.
- **Alternatives considered:**
  - Add blanket `allow(dead_code)` — rejected in favor of feature-accurate compilation boundaries.

## Architectural Notes
- Family API boundary is now: `runner route -> RsProjectSurface + route -> family check`.
- Local family routing remains mapper-driven; families keep content/policy logic, not structural legality ownership.
- `RsProjectSurface::from_route_scope` now correctly retains only scoped files when scoped sets are provided.
- `app/hooks/mod.rs` was aligned so hooks-shared also receives `RsProjectSurface`.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-03-31-125334-rs-legality-first-local-family-migration.md`
- `.worklogs/2026-03-31-140740-rs-structure-legality-routing.md`
- `.worklogs/2026-03-31-153735-enforce-arch-owned-placement-and-local-family-routing.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`

## Open Questions / Future Considerations
- `RsProjectSurface` currently still exposes `tree()`; if we want harder isolation, next step is replacing that with a narrower query interface and migrating family internals to it.
- While applying formatting, `cargo fmt --all` failed on an unrelated TS missing module path; Rust formatting/checks were validated via targeted compile/test commands instead.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — surface construction and family dispatch wiring
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — `RsProjectSurface` implementation and scoping behavior
- `apps/guardrail3/crates/app/rs/family_mapper/src/lib.rs` — exported surface type
- `apps/guardrail3/crates/app/rs/families/*/src/lib.rs` and `.../crates/runtime/src/lib.rs` — family entrypoints now consuming surfaces
- `apps/guardrail3/crates/app/hooks/mod.rs` — hooks-shared entrypoint call alignment
- `.worklogs/2026-03-31-153735-enforce-arch-owned-placement-and-local-family-routing.md` — immediate prior checkpoint

## Next Steps / Continuation Plan
1. Decide whether to remove `RsProjectSurface::tree()` and migrate family internals to a strict surface query API (no raw tree escape).
2. Add explicit runtime tests that assert each family runner receives only intended route-shaped files.
3. Split any remaining structural legality checks still encoded as family-local “input failures” into shared legality + arch reporting where applicable.
