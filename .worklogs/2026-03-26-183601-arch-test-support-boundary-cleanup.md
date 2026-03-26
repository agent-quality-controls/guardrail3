# Arch Test Support Boundary Cleanup

**Date:** 2026-03-26 18:36
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/test_support/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_*_tests/*`

## Summary
Removed the semantic Cargo fixture API from the arch family’s shared `test_support` crate and moved those canned manifest helpers into the runtime-side rule test modules. The shared test support crate now stays generic, while the rule tests still have local access to the same fixture bodies through their own module scope.

## Context & Problem
`RS-TEST-18` was tightened to reject semantic fixture surfaces in `test_support`, not just route-construction imports. The arch family still exposed `CargoFixture` and `cargo_fixture(...)` from `test_support`, which encoded canned manifest policy and made shared support non-generic. The goal was to preserve test coverage without keeping the policy in the shared crate.

## Decisions Made

### Move semantic fixture data into runtime-local test modules
- **Chose:** Defined `CargoFixture` and `cargo_fixture(...)` inside each affected `rs_arch_*_tests/mod.rs` module instead of in `test_support`.
- **Why:** The validator already allows sidecar-local helpers; it rejects sibling production-module detours. Keeping the fixture data inside the rule module avoids the `RS-TEST-03` sibling-module escape and removes the `RS-TEST-18` boundary leak.
- **Alternatives considered:**
  - A crate-root runtime helper module — rejected because `RS-TEST-03` flagged it as an unwanted sibling production-module import path.
  - Keeping the enum/function in `test_support` — rejected because that is the exact semantic leak `RS-TEST-18` is meant to ban.

### Keep shared `test_support` generic only
- **Chose:** Left `entry`, `tree`, and `tree_at` in `arch/test_support/src/lib.rs`, and removed the manifest fixture enum/function/consts.
- **Why:** Those helpers are generic `ProjectTree` construction primitives and do not encode arch-family policy.
- **Alternatives considered:**
  - Splitting generic tree builders into another helper crate — unnecessary churn for no boundary gain.

## Architectural Notes
The arch family now follows the same pattern as the stricter `RS-TEST` contract wants elsewhere: generic shared support is limited to tree construction, while semantic fixture bodies live next to the rule tests that use them. That keeps policy in the rule-owned module tree and prevents the support crate from becoming a back door for canned manifests.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `apps/guardrail3/crates/app/rs/families/arch/test_support/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_01_root_classification_tests/*`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots_tests/*`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_03_no_dual_ownership_tests/*`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_04_no_zone_overlap_tests/*`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_05_scoped_arch_config_forbidden_tests/*`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_06_owner_family_enablement_coherence_tests/*`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/*`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_08_auxiliary_roots_declared_tests/*`

## Open Questions / Future Considerations
- `RS-TEST-16` still deserves a deeper pass on semantic ownership in the assertions layer, but that is separate from this arch support cleanup.
- `cargo` and `hexarch` still need the same `test_support` boundary review if they continue to expose semantic fixture surfaces.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/test_support/src/lib.rs` — generic shared tree builders only after this change.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs` — runtime test module wiring for the arch family.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_01_root_classification_tests/mod.rs` — example of the local fixture helper pattern.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_08_auxiliary_roots_declared_tests/mod.rs` — example of a rule module that does not need the semantic fixture helper.
- `.worklogs/2026-03-26-180221-tighten-rs-test-proof-boundaries.md` — prior validator-tightening context.

## Next Steps / Continuation Plan
1. Commit the arch test support boundary cleanup as a checkpoint once the worklog is staged.
2. Keep `RS-TEST-18` in place and repeat the same review for `cargo` and `hexarch` if their support crates still expose semantic fixture APIs.
3. If `RS-TEST-16` still feels too weak after the broader family pass, tighten proof-bearing assertions semantics separately from support-crate boundaries.
