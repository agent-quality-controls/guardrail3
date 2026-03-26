# Route Rs Cargo Through Mapper

**Date:** 2026-03-26 14:27
**Scope:** `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/lib.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`, `apps/guardrail3/crates/app/rs/families/cargo/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/cargo/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/cargo/src/discover.rs`, `apps/guardrail3/crates/app/rs/families/cargo/src/test_support.rs`, `apps/guardrail3/Cargo.lock`

## Summary
Moved `rs/cargo` onto a typed mapper route so the family no longer decides which `Cargo.toml` roots are in scope by scanning `ProjectTree` itself. The cargo family still builds the same manifest snapshots and rule facts, but it now does that work only for routed roots from the shared Rust orchestration layer.

## Context & Problem
After routing `arch`, `test`, `hexarch`, `code`, and `garde`, the remaining straightforward Cargo-root family still doing local scope discovery was `rs/cargo`. Its `discover.rs` started by collecting every visible `Cargo.toml` path from the tree and then derived workspace roots, workspace members, standalone packages, and policy roots from that local crawl.

That contradicted the current `apps/guardrail3/crates/app/rs/README.md` architecture:

`ProjectTree -> placement -> family_selection -> FamilyMapper -> typed family route -> family facts`

`cargo` was a good next slice because its semantics are already organized around manifest snapshots and do not depend on staged-file scoping or extra tool surfaces.

## Decisions Made

### Add a simple cargo route rather than a larger shared abstraction
- **Chose:** Introduce `RsCargoRoute { roots: Vec<RsRootView> }` and `map_rs_cargo()`.
- **Why:** `cargo` only needs owned manifest roots. It does not need overlaps, staged files, or family-specific projections beyond a routed root set.
- **Alternatives considered:**
  - Reuse a more generic root-only route type immediately for `release` too — rejected for now because `release` has extra repo-level surfaces and is better handled once its needs are inspected directly.
  - Leave `cargo` on raw `ProjectTree` until `release` is also ready — rejected because `cargo` is already cleanly separable and keeping it raw only preserves unnecessary duplication.

### Keep cargo-family snapshot parsing intact, but source it from routed roots
- **Chose:** Keep the existing `CargoSnapshot` pipeline and all downstream rule fan-out, but replace `dirs_with_file("Cargo.toml")` discovery with iteration over `route.roots`.
- **Why:** The architectural problem was ownership discovery, not the family’s manifest parsing logic. Reusing the snapshot model reduces migration risk.
- **Alternatives considered:**
  - Rewrite cargo facts around mapper-provided precomputed manifest data — rejected because that would move family semantics out of the family and make this slice much larger than needed.

### Route test helpers through placement + mapper too
- **Chose:** Update `cargo` test support to build a route via `placement` + `FamilyMapper` before calling `check`.
- **Why:** The tests should exercise the same entry boundary as production. Otherwise the family would still have a hidden local-path API in tests.
- **Alternatives considered:**
  - Keep a test-only raw-tree compatibility entrypoint — rejected because it leaves the old scope boundary alive in another code path.

## Architectural Notes
This makes `cargo` follow the same boundary as the other migrated Rust families:

`ProjectTree -> placement -> FamilyMapper -> RsCargoRoute -> cargo discover/facts -> rule inputs`

What changed:
- `runtime.rs` now dispatches `cargo` through `mapper.map_rs_cargo()`
- `FamilyMapper` now projects routed roots for `cargo`
- `cargo/src/discover.rs` no longer performs live root discovery from the tree

What did not change:
- the manifest snapshot model
- workspace/member parsing
- policy-root derivation
- rule implementations

The remaining architectural gap in this lane is now primarily `release`, plus the broader cleanup of policy split between `family_selection`, `FamilyMapper`, and `runtime.rs`.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md` — architecture target for mapper-driven family input
- `apps/guardrail3/crates/app/rs/runtime.rs` — current Rust runtime dispatch
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` and `rs.rs` — routed family inputs
- `apps/guardrail3/crates/app/rs/families/cargo/src/discover.rs` — prior local root discovery
- `apps/guardrail3/crates/app/rs/families/cargo/src/test_support.rs` — test entrypoint shape
- `.worklogs/2026-03-26-142241-route-rs-code-and-garde-through-mapper.md` — previous routed-family migration in this sequence

## Open Questions / Future Considerations
- `release` still crawls Cargo roots locally and also owns repo-level release config/workflow discovery.
- `FamilyMapper` still applies per-root family enablement policy instead of being a pure projection layer.
- `runtime.rs` still keeps post-family applicability filtering, so the Rust orchestration boundary is improved but not yet fully normalized.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — current architecture contract
- `apps/guardrail3/crates/app/rs/runtime.rs` — runtime dispatch and residual applicability logic
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — routed input shapes
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — mapper implementation
- `apps/guardrail3/crates/app/rs/families/cargo/src/lib.rs` — routed cargo family entrypoint
- `apps/guardrail3/crates/app/rs/families/cargo/src/discover.rs` — route-aware manifest snapshot collection
- `apps/guardrail3/crates/app/rs/families/cargo/src/test_support.rs` — test helper path through placement + mapper
- `.worklogs/2026-03-26-142241-route-rs-code-and-garde-through-mapper.md` — prior routed-source-family checkpoint

## Next Steps / Continuation Plan
1. Inspect `rs/release` and define the minimal routed input it needs to stop rediscovering Cargo roots locally.
2. Migrate `rs/release` to that route and update its test support to use placement + mapper.
3. Once `release` is routed, reassess `runtime.rs` and `FamilyMapper` against the README and remove the remaining duplicate applicability/policy logic.
4. Add dedicated mapper/selection tests so these route contracts stop being enforced only indirectly through family tests.
