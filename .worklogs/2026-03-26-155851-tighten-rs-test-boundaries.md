# Tighten Rs Test Boundaries

**Date:** 2026-03-26 15:58
**Scope:** `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic_tests/boundaries.rs`, `apps/guardrail3/crates/app/rs/families/test/README.md`, `.plans/todo/checks/rs/test.md`

## Summary
Tightened `RS-TEST` to catch two concrete blind spots exposed by the rewritten cargo family: sidecar escapes through `super::...` into sibling runtime modules, and route-construction logic hidden inside `test_support`. Added regression tests for both and verified that the `rs/test` family still passes on itself while `rs/cargo` now fails `RS-TEST` for the newly enforced violations.

## Context & Problem
The cargo family had been structurally rewritten into `runtime/assertions/test_support` and passed the live `RS-TEST` validator. An adversarial pass showed that the green result was partly artificial:

- a runtime sidecar imported sibling runtime helper `lint_support` via `super::super::super::...`
- cargo `test_support` constructed routed family input via `placement + FamilyMapper`

Those both violate the current contract in `families/test/README.md`, but the validator was not catching them. The goal of this slice was to tighten `RS-TEST` itself before judging whether cargo is truly compliant.

## Decisions Made

### Treat local-boundary `super::...` sidecar escapes as real `RS-TEST-03` violations
- **Chose:** Extend sibling-module detection to resolve local-boundary paths starting with `super::` or `self::`, not just `crate::`.
- **Why:** Internal sidecars are allowed to import their owned production subtree, sibling assertions, and `test_support`, but not sibling runtime modules. The cargo family escape used a `super::...` path specifically because the old checker only looked at `crate::`.
- **Alternatives considered:**
  - Only flag imports and keep direct call escapes out of scope — rejected because a direct local-boundary call can bypass the same architectural boundary.
  - Relax the contract for nested sidecar helper imports — rejected because it would make the runtime/assertions split toothless in exactly the way the adversarial review found.

### Treat mapper/placement wiring in `test_support` as non-generic support
- **Chose:** Extend `RS-TEST-18` to reject imports of `guardrail3_app_rs_family_mapper` / `guardrail3_app_rs_placement` and direct `FamilyMapper` construction from `test_support`.
- **Why:** The contract says `test_support` owns generic builders, fixture helpers, and result helpers only. Route construction is component-specific orchestration, not generic support. Cargo had hidden that logic in `test_support`, which made the shape look compliant while keeping family-specific wiring in the wrong layer.
- **Alternatives considered:**
  - Leave `RS-TEST-18` limited to sibling runtime/assertions crate imports only — rejected because it lets family-specific orchestration hide in a nominally generic support crate.
  - Try to detect “genericness” semantically via broad heuristics — rejected because mapper/placement wiring is a crisp syntactic boundary with deterministic detection.

### Do not over-tighten `RS-TEST-16` yet
- **Chose:** Back out the attempted `RS-TEST-16` grouped-helper-file enforcement.
- **Why:** The first implementation conflated internal helper-file ownership with the existing external-harness assertions pattern and overfired on already-accepted assertions modules. The cargo assertions thin-wrapper problem is real, but the deterministic enforcement model needs a more careful design than this patch.
- **Alternatives considered:**
  - Keep the first `RS-TEST-16` tightening and rewrite the existing rule tests around it immediately — rejected because that would make this checkpoint about a half-designed contract change instead of the concrete blind spots we were actually fixing.

## Architectural Notes
This checkpoint strengthens `RS-TEST` as a validator, not the cargo family itself. The important behavioral change is:

- `rs/test` still passes on itself
- `rs/cargo` no longer gets a false green

After this patch, `rs validate .../families/cargo --family test` reports:
- one `RS-TEST-03` error for the sidecar sibling-module escape
- two `RS-TEST-18` errors for route-construction logic in `test_support`

That is the intended result: the validator now exposes the actual remaining cargo-family work instead of approving it structurally by accident.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/README.md` — live contract for sidecar imports and `test_support`
- `.plans/todo/checks/rs/test.md` — accepted RS-TEST inventory and detection reminders
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_03_allow_inventory_tests/cases.rs` — concrete sidecar boundary escape
- `apps/guardrail3/crates/app/rs/families/cargo/test_support/src/lib.rs` — concrete route-construction leakage in `test_support`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — boundary rule implementation
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs` — `test_support` rule implementation

## Open Questions / Future Considerations
- The cargo assertions crate still looks thin and wrapper-heavy. That is a real quality gap, but the current `RS-TEST-16` contract is not yet strong enough to reject it deterministically without collateral damage.
- The cargo family itself also appears to have non-`RS-TEST` issues in its own rule semantics (`RS-CARGO-14` activation breadth, workspace metadata fallbacks, `exclude` handling). Those are separate from this checkpoint.
- If more families adopt local `runtime/assertions/test_support` workspaces, they may surface the same `test_support` route-builder smell now that `RS-TEST-18` is stricter.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — tightened sibling-module boundary detection
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs` — tightened `test_support` genericness rule
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs` — regression for `super::...` sidecar escape
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic_tests/boundaries.rs` — regression for mapper/placement wiring in `test_support`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_03_allow_inventory_tests/cases.rs` — real-world failing cargo sidecar after the patch
- `apps/guardrail3/crates/app/rs/families/cargo/test_support/src/lib.rs` — real-world failing cargo `test_support` after the patch
- `.worklogs/2026-03-26-154718-arch-rs-test-proof-fixes.md` — previous RS-TEST tightening context from the arch-family migration

## Next Steps / Continuation Plan
1. Keep the validator changes isolated in this commit; do not mix them with the still-dirty cargo-family rewrite.
2. Re-run `RS-TEST` on the cargo family and treat the new three findings as the real remaining cargo compliance work.
3. Fix cargo by:
   - removing the `super::... lint_support` sidecar escape
   - moving route construction out of `cargo/test_support`
4. After cargo is structurally clean again, repeat the same adversarial loop before trusting its green status.
