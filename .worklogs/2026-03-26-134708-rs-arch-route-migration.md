# Route the Rust Arch Family Through FamilyMapper

**Date:** 2026-03-26 13:47
**Scope:** `apps/guardrail3/crates/app/rs/README.md`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`, `apps/guardrail3/crates/app/rs/families/arch/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/arch/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/arch/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/arch/src/test_support.rs`, `apps/guardrail3/crates/app/rs/runtime.rs`

## Summary
Migrated the `rs/arch` family onto the shared routed-root path. `arch` now consumes `RsArchRoute` from `FamilyMapper` instead of calling placement directly, and the Rust-scope README now reflects that both `arch` and `test` are on routed roots.

## Context & Problem
The prior commit established the new shared path for `rs/test`, but `rs/arch` still pulled placement facts inside the family crate. That left the repo in an intermediate state where the documented architecture existed, but one of the two explicitly problematic Rust families still owned its own root-scope feed.

The README migration plan specifically called out:
1. move `arch` off `placement::collect(...)`
2. keep families from deciding their own routed universe

So the next safe slice was to migrate `arch` while preserving its rule behavior.

## Decisions Made

### Add `classification` to `RsArchRootView`
- **Chose:** Extended `RsArchRootView` in `family_mapper` to carry `RustRootClassification`.
- **Why:** `RS-ARCH-01`, `RS-ARCH-02`, and `RS-ARCH-08` all depend on placement classification. Recomputing that classification inside `arch` would have duplicated placement semantics and defeated the boundary.
- **Alternatives considered:**
  - Recompute classification from zone candidates in `arch` — rejected because that recreates placement logic in a family.
  - Route full placement root facts into `arch` — rejected because that widens the route API instead of keeping it narrow.

### Keep config parsing local to `arch` for now
- **Chose:** `arch` still parses `guardrail3.toml` locally to derive scoped-arch failures and owner-family coherence inputs, but it now receives routed roots, overlaps, placement input failures, and the precomputed misplaced-root reporting flag from `RsArchRoute`.
- **Why:** The immediate goal was to decouple live root crawling from family input. Config parsing is family semantics; moving it in the same change would have made the migration harder to validate.
- **Alternatives considered:**
  - Push all arch config interpretation into `FamilyMapper` — rejected because mapper is supposed to project scope, not own rule semantics.
  - Leave misplaced-root reporting derivation in family config resolution too — rejected because `RsArchRoute` already carried that bit and using the route avoids duplicate policy.

### Reconstruct only the minimal root facts needed by `arch`
- **Chose:** Replaced the old placement type aliases in `arch::facts` with local `ArchRootFacts` and `ZoneOverlapFacts`, populated from `RsArchRoute`.
- **Why:** This keeps the family working with stable local facts while severing the dependency on placement’s full root-facts type.
- **Alternatives considered:**
  - Rewrite all arch rule/input types directly around route views in one pass — rejected because it would be a larger refactor with little additional architectural gain right now.

### Update `runtime.rs` and arch test support to use the routed path
- **Chose:** `runtime.rs` now calls `arch::check(&tree, &mapper.map_rs_arch())`, and arch test support constructs the same route through `placement + FamilyMapper`.
- **Why:** This ensures both production and family-unit test paths exercise the same routed entrypoint.
- **Alternatives considered:**
  - Keep test support calling a private direct-family helper — rejected because it would hide route regressions from family tests.

## Architectural Notes
After this commit, the two families named explicitly in the Rust-scope architecture plan are both on routed roots:

- `rs/test`
- `rs/arch`

That means live Rust root discovery now has one production entrypoint:

`ProjectTree -> placement -> family_selection -> FamilyMapper -> family route`

What is still not done:
- other families still use older direct `ProjectTree` entrypoints
- `runtime.rs` still carries path-based applicability/result filtering for non-migrated families
- `hexarch` still performs deep family-local app/member discovery, which is allowed, but it has not been moved to a routed-root entrypoint yet

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md` — migration plan requiring `arch` to consume typed mapped routes
- `.worklogs/2026-03-26-133815-rs-selection-mapper-test-route-slice.md` — prior routed-root slice for `rs/test`
- `apps/guardrail3/crates/app/rs/families/arch/src/facts.rs` — prior placement-coupled implementation
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — current route construction logic

## Open Questions / Future Considerations
- `arch` still parses `guardrail3.toml` internally. That is fine for now, but if route contracts get richer later, some of those config-derived signals may migrate outward.
- The README route examples were missing `classification` for `RsArchRootView`; this commit updates the doc to match the real family needs.
- `runtime.rs` still has mixed-mode behavior because not every family consumes routed inputs yet.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — current documented target state after `arch` route migration
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — route view definitions, now including arch classification
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — route projection logic for `map_rs_arch`
- `apps/guardrail3/crates/app/rs/families/arch/src/facts.rs` — route-fed arch facts and config-derived arch semantics
- `apps/guardrail3/crates/app/rs/families/arch/src/lib.rs` — `arch` family entrypoint now consuming `RsArchRoute`
- `apps/guardrail3/crates/app/rs/families/arch/src/test_support.rs` — family tests now build the routed arch entrypoint
- `.worklogs/2026-03-26-133815-rs-selection-mapper-test-route-slice.md` — prior slice that introduced shared selection/mapper and migrated `rs/test`

## Next Steps / Continuation Plan
1. Add dedicated tests for `family_mapper::map_rs_arch` and `map_rs_test`, especially around config-driven root inclusion and route shape.
2. Decide whether `runtime.rs` applicability/result filtering can be reduced now that both `arch` and `test` consume routed roots.
3. Identify the next Rust family that still owns root-entry discovery and move it onto a typed route if it shares the same scope problem.
4. Clean up remaining documentation/worktree drift only after confirming which dirty files are part of this architecture stream versus unrelated unfinished edits.
