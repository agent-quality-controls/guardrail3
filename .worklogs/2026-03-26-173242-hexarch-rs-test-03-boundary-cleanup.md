# Hexarch RS-TEST-03 Boundary Cleanup

**Date:** 2026-03-26 17:32
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config_tests/**`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_17_workspace_inherited_direction.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_17_workspace_inherited_direction_tests/**`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_19_same_layer_cycles.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_19_same_layer_cycles_tests/**`

## Summary
I removed the remaining direct sibling-production-module imports from the scoped `hexarch` `RS-TEST-03` sidecars for rules 15, 17, and 19. The tests now call small helpers owned by their respective rule modules, so the sidecars stay within the owning module boundary while preserving the existing rule behavior.

## Context & Problem
`RS-TEST-03` had been tightened to reject sidecars that reached into sibling production modules such as `dependency_facts`, `inputs`, and shared route-construction helpers. That caught the `hexarch` rule-15/17/19 tests because they still built inputs or selected facts directly from sibling modules instead of asking their own rule module for a prepared test path. The goal here was to keep the checks themselves unchanged while moving the test harnesses back behind the rule-module boundary.

## Decisions Made

### Test-only helpers live in the owned rule module
- **Chose:** Added small `#[cfg(test)]` helper functions in the three rule modules so the sidecars only import their own module.
- **Why:** This preserves the rule boundary that `RS-TEST-03` is enforcing without changing the rule logic or the shape of the prod inputs.
- **Alternatives considered:**
  - Let sidecars continue importing `dependency_facts` and `inputs` directly - rejected because that is exactly the pattern `RS-TEST-03` should block.
  - Move helpers into `test_support` - rejected because it would reintroduce a shared helper layer that can grow into a backdoor.

### Keep behavior stable, not just tests green
- **Chose:** The new helpers internally construct the same facts/inputs and invoke the same rule `check(...)` functions.
- **Why:** The point is boundary enforcement, not changing rule semantics.
- **Alternatives considered:**
  - Rewrite rule logic to use a different test path - rejected because that would make the test change semantic instead of structural.

## Architectural Notes
The test helpers are intentionally narrow:
- rule 15 exposes a boundary-config helper that takes raw fields and builds the `BoundaryConfigFacts` internally
- rule 17 exposes audit helpers that collect dependency facts and invoke sibling rules internally
- rule 19 exposes cycle helpers that collect cycle facts internally and convert them to rule inputs

That keeps the sidecar files thin and avoids direct sibling-module imports in test code. The broader hexarch family still has unrelated failing `RS-TEST-24/25` assertions in the full suite, but those are outside this scoped cleanup.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_17_workspace_inherited_direction.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_19_same_layer_cycles.rs`
- their corresponding `*_tests/**` directories
- the current `RS-TEST-03` contract in `apps/guardrail3/crates/app/rs/families/test/README.md`

## Open Questions / Future Considerations
- The full unfiltered `hexarch` family test run still fails on unrelated `RS-TEST-24/25` cases. Those are not part of this scope, but they block a clean full-family green run.
- If the team wants the whole hexarch family test binary to compile without temporary lint suppression, the next cleanup is in the unrelated rule clusters that currently trigger dead-code lints.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config.rs` - rule-15 boundary-config logic and its new test helper.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config_tests/missing_config.rs` - sidecar converted to the module-owned helper.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_17_workspace_inherited_direction.rs` - rule-17 logic plus new audit helpers.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_17_workspace_inherited_direction_tests/*.rs` - sidecars that now call the owned helper instead of sibling modules.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_19_same_layer_cycles.rs` - rule-19 logic plus new cycle helpers.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_19_same_layer_cycles_tests/*.rs` - sidecars that now call the owned helper instead of sibling modules.
- `.worklogs/2026-03-26-172253-tighten-rs-test-route-boundaries-and-runtime-test-entrypoints.md` - prior checkpoint that established the runtime-owned test-entrypoint pattern.

## Next Steps / Continuation Plan
1. Commit the scoped hexarch test-boundary cleanup.
2. If the objective is a fully green unfiltered hexarch test binary, tackle the unrelated `RS-TEST-24/25` failures in separate work.
3. Otherwise, stop here and treat this change as a boundary-only fix for `RS-TEST-03` in rules 15, 17, and 19.
