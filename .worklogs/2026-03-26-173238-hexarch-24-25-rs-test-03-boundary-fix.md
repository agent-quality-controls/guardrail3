# Hexarch 24/25 RS-TEST-03 Boundary Fix

**Date:** 2026-03-26 17:32
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_24_cross_app_boundary.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_24_cross_app_boundary_tests/broad_attacks.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_25_target_dependency_direction.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_25_target_dependency_direction_tests/broad_attacks.rs`, plus the small hexarch test-only compile cleanup in `rs_hexarch_15_boundary_config.rs`, `rs_hexarch_17_workspace_inherited_direction.rs`, `rs_hexarch_19_same_layer_cycles.rs`, `rs_hexarch_21_domain_purity.rs`, `rs_hexarch_22_ports_trait_dominance.rs`, and `rs_hexarch_23_adapter_pub_trait.rs`

## Summary
I removed the remaining `RS-TEST-03` sidecar boundary escapes for hexarch rules 24 and 25 without changing the rule logic. The final shape keeps the broad-attack sidecars on the assertions boundary and pushes the tree-based entrypoint into the owned assertion modules, so the sidecars no longer reach into sibling runtime modules directly.

## Context & Problem
The hexarch family had two lingering `RS-TEST-03` violations in the `broad_attacks` sidecars for rules 24 and 25. The test files were still importing runtime-side dependency facts and input helpers directly, which the tightened test rule correctly treated as a boundary escape. A first attempt to reroute through filesystem materialization changed the semantics by letting unrelated rules fire, so that approach had to be discarded.

## Decisions Made

### Keep the sidecars on the assertions boundary
- **Chose:** The `broad_attacks` sidecars now call tree-based helpers in the hexarch assertions modules instead of importing runtime rule modules.
- **Why:** The assertions crate is the allowed test-facing boundary, and it can safely delegate to runtime internals without the sidecar itself crossing module boundaries.
- **Alternatives considered:**
  - Materialize the synthetic `ProjectTree` onto disk and call `run_family(root)` - rejected because it woke up unrelated hexarch rules and changed the test meaning.
  - Keep the direct runtime imports and relax `RS-TEST-03` - rejected because that would reintroduce the loophole the validator is meant to close.

### Preserve rule-specific behavior with tree-based helpers
- **Chose:** Added `run_tree(&ProjectTree)` helpers in the 24/25 assertions modules and kept the broad-attack expectations filtered to `RS-HEXARCH-24` and `RS-HEXARCH-25`.
- **Why:** This preserves the original in-memory test shape and keeps the expected counts aligned with the specific rule under attack.
- **Alternatives considered:**
  - Rewriting the tests into filesystem fixture mutations - rejected as more invasive than needed for this boundary fix.

## Architectural Notes
The important boundary is now:
- sidecars depend on assertions modules only
- assertions modules may delegate to runtime
- runtime remains the place where the family-specific tree-level helper lives

That keeps the validator honest without duplicating route/discovery logic inside the test files. A small amount of test-only compile cleanup was also required in nearby hexarch modules so the crate could build under `-D warnings`; those changes do not alter production behavior.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` for the tightened `RS-TEST-03` intent.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_24_cross_app_boundary_tests/broad_attacks.rs` and `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_25_target_dependency_direction_tests/broad_attacks.rs` for the failing boundary imports.
- Existing hexarch assertions modules in `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/` for the allowed test-facing shape.

## Open Questions / Future Considerations
- The hexarch runtime still has test-only helpers in the production modules for 24/25. They are not required by the current sidecars, but they are harmless and may be worth deleting later if we want to remove any dead test-only scaffolding.
- Other hexarch `RS-TEST-03` failures were not part of this slice and remain to be handled separately if the family is still not fully compliant after the broader migration.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_24_cross_app_boundary.rs` - rule 24 runtime check and test helper.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_24_cross_app_boundary_tests/broad_attacks.rs` - targeted boundary attack that was rewritten.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_25_target_dependency_direction.rs` - rule 25 runtime check and test helper.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_25_target_dependency_direction_tests/broad_attacks.rs` - targeted boundary attack that was rewritten.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_24_cross_app_boundary.rs` - assertions-layer tree entrypoint for rule 24.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_25_target_dependency_direction.rs` - assertions-layer tree entrypoint for rule 25.

## Next Steps / Continuation Plan
1. If we want to keep tightening `RS-TEST-03`, delete the now-unneeded test-only helpers in the 24/25 runtime modules and rely solely on the assertions-layer tree helpers.
2. Re-run the hexarch family validator after any further cleanup to ensure the 24/25 boundary fix stays green.
3. Resume the broader hexarch RS-TEST migration separately if the remaining rule files still have sidecar boundary escapes.
