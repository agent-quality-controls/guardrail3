# Extract Rust Family Selection and Start Routed Family Inputs

**Date:** 2026-03-26 13:38
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/placement/src/lib.rs`, `apps/guardrail3/crates/app/rs/family_selection/*`, `apps/guardrail3/crates/app/rs/family_mapper/*`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/*`, `apps/guardrail3/crates/app/rs/families/test/crates/assertions/*`

## Summary
Introduced the first concrete implementation slice of the shared Rust root-scope architecture. Rust family selection now lives in a dedicated `family_selection` crate, a new `family_mapper` crate projects placement facts into typed route views, and the `rs/test` family now consumes routed roots from placement instead of rediscovering live Cargo roots locally.

## Context & Problem
The planning work in `apps/guardrail3/crates/app/rs/README.md` established that Rust families should stop deciding for themselves which `Cargo.toml` roots are live. The immediate code smell was:

- `runtime.rs` owned family selection and enablement
- `rs/test` still performed its own live root discovery from `ProjectTree`
- the self-hosted `rs/test` family could pass locally while still depending on a family-local root universe

The user explicitly wanted implementation to start from that architecture rather than continue refining docs.

## Decisions Made

### Extract family selection into its own crate
- **Chose:** Added `apps/guardrail3/crates/app/rs/family_selection` and moved the existing runtime family-selection logic there.
- **Why:** This is the least controversial part of the new architecture and removes a clear policy blob from `runtime.rs` without changing family behavior.
- **Alternatives considered:**
  - Leave selection in `runtime.rs` until mapper and routes were fully in place — rejected because it would preserve the old boundary while claiming the new architecture existed.
  - Move selection into `family_mapper` — rejected because the user explicitly clarified that the mapper should be “literally a mapper.”

### Introduce a dedicated mapper crate with owned route views
- **Chose:** Added `apps/guardrail3/crates/app/rs/family_mapper` with `FamilyMapper`, `RsRootView`, `RsArchRootView`, `RsArchOverlapView`, `RsRootInputFailureView`, `RsArchRoute`, `RsHexarchRoute`, and `RsTestRoute`.
- **Why:** The README direction was to avoid leaking raw placement facts into families. Owned narrow route views are the practical middle ground between ids-only and full shared facts.
- **Alternatives considered:**
  - Route ids only — rejected for this slice because `rs/test` would immediately need lookup plumbing back into placement.
  - Pass full `RustRootPlacementRootFacts` into families — rejected because that would collapse the boundary again.

### Migrate `rs/test` first to routed roots
- **Chose:** Updated `guardrail3-app-rs-family-test` so `check(...)` accepts `&RsTestRoute`, and changed discovery to consume routed root views instead of scanning `ProjectTree` for all `Cargo.toml` directories.
- **Why:** `rs/test` was the family currently exhibiting the root-scope drift problem most clearly, and it was already under active architectural scrutiny.
- **Alternatives considered:**
  - Migrate `arch` first — rejected for this slice because `arch` still mixes placement facts with config-derived family behavior and would have widened the change surface.
  - Migrate both `arch` and `test` together — rejected because it would make it harder to tell whether the new shared root path was actually sound.

### Keep runtime applicability filtering in place for now
- **Chose:** Left `RustFamilyApplicability` and path-based post-filtering in `runtime.rs` for now, while still using `FamilyMapper` to build the routed `RsTestRoute`.
- **Why:** This keeps the migration incremental. `rs/test` now uses shared routed roots, but other families still depend on the old runtime path and would otherwise change behavior in the same commit.
- **Alternatives considered:**
  - Remove post-filtering immediately — rejected because only one family is on routed inputs so far.
  - Move result filtering into `FamilyMapper` — rejected because that would contradict the intended mapper boundary.

### Keep the assertions crate aligned with the new route signature
- **Chose:** Added a small helper in `guardrail3-app-rs-family-test-assertions` to build `RsTestRoute` via `placement + FamilyMapper`, then rewired all rule helpers to call `runtime::check(tree, &route, checker)`.
- **Why:** The assertions crate is the canonical test harness for the family; it needed to exercise the same routed-root entrypoint as production runtime.
- **Alternatives considered:**
  - Hand-build ad hoc routes inside each assertions module — rejected because that would duplicate route construction in 18 places.
  - Keep a compatibility overload on `runtime::check` — rejected because it would preserve the old unscoped entrypoint.

## Architectural Notes
This commit does not finish the architecture. It establishes the first real path:

`ProjectTree -> placement -> family_selection -> FamilyMapper -> RsTestRoute -> rs/test family`

Important current state:

- `family_selection` is now a standalone crate and the source of runtime family-set resolution.
- `FamilyMapper` exists and maps placement facts into owned route views.
- `rs/test` no longer decides which Cargo roots are live.
- `runtime.rs` still retains old applicability/result filtering for families that have not yet been migrated.
- `arch` and other Rust families still consume the old direct `ProjectTree` path.

This is intentionally a partial migration, not a final architecture landing.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md` — target architecture and route-shape decisions
- `.worklogs/2026-03-26-132416-rs-family-mapper-boundary-tightening.md` — prior decision record on selection vs mapper and route view shape
- `apps/guardrail3/crates/app/rs/runtime.rs` — previous inlined selection and applicability logic
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — previous family-local Cargo root discovery
- `apps/guardrail3/crates/app/rs/placement/src/*` — existing shared Rust root-scope implementation

## Open Questions / Future Considerations
- `runtime.rs` still owns path-based applicability filtering. Once more families consume routed inputs, that logic should either disappear or shrink substantially.
- `FamilyMapper::map_rs_arch` and `map_rs_hexarch` now exist, but no production family consumes them yet.
- `rs/test` route filtering currently piggybacks on existing per-root enablement logic, but broader policy consistency still needs explicit regression tests in the mapper layer.
- `Cargo.lock` was left untouched in the commit because the local diff also contained unrelated workspace churn outside this slice.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — target architecture for shared scope, selection, and mapping
- `apps/guardrail3/crates/app/rs/runtime.rs` — current runtime entrypoint after extracting family selection and routing `rs/test`
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — extracted runtime family-selection logic
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — typed route mapping and per-family root filtering
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — routed-root discovery path for `rs/test`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — `rs/test` entrypoint now consuming `RsTestRoute`
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs` — shared route construction for family assertions
- `.worklogs/2026-03-26-132416-rs-family-mapper-boundary-tightening.md` — design rationale immediately preceding this implementation slice

## Next Steps / Continuation Plan
1. Add mapper-focused regression tests around per-root family routing, especially app/package enablement and scoped file handling, in `apps/guardrail3/crates/app/rs/family_mapper`.
2. Migrate `arch` to consume `RsArchRoute` instead of collecting placement facts internally. Read `apps/guardrail3/crates/app/rs/families/arch/src/facts.rs` and peel root/overlap/input-failure sourcing out of that family first.
3. Revisit `runtime.rs` applicability filtering once at least `arch` and `test` are both routed. At that point, decide whether result filtering can be deleted for migrated families.
4. Move additional families off direct `ProjectTree` root discovery in small slices, using routed root views rather than raw placement facts.
