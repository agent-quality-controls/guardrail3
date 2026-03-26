# Fix Arch RS-TEST-16 Sidecar Assertions

**Date:** 2026-03-26 18:59
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_01_root_classification_tests/ambiguous_roots.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_03_no_dual_ownership_tests/dual_ownership.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_06_owner_family_enablement_coherence_tests/ownership_coherence.rs`

## Summary
Removed the last direct severity assertions from the `arch` family sidecars that were tripping the tightened `RS-TEST-16` validator. The sidecars now rely on the owned assertions helpers for file-set checking and stop inspecting `CheckResult` severity directly.

## Context & Problem
The new `RS-TEST-16` tightening is meant to keep the canonical semantic proof surface in the owned assertions crate instead of letting runtime sidecars re-assert result structure themselves. `arch` still had three sidecars that called `assert_error_files(...)` and then independently walked `error_results(...)` to check that every hit was `Severity::Error`. That was redundant and gave the sidecars a direct result-shape assertion surface that the validator now rejects.

## Decisions Made

### Keep file-set assertions in the owned assertions helper
- **Chose:** Leave `assert_error_files(...)` in the assertions crate as the canonical helper for the arch sidecars.
- **Why:** It already encodes the rule-specific hit set and keeps the reusable semantic check close to the family assertions module.
- **Alternatives considered:**
  - Rework the assertions crate to expose a more granular severity helper - rejected because the current helper already enforces the intended outcome, and the issue was the extra direct severity check in the sidecar.
  - Move the entire check back into the sidecar - rejected because that is exactly the structure `RS-TEST-16` is trying to prevent.

### Remove direct severity inspection from sidecars
- **Chose:** Deleted the `error_results(...).iter().all(|result| result.severity == Severity::Error)` assertions from the three failing sidecars.
- **Why:** Severity is already part of the validator output contract and the owned assertions helper is sufficient to prove the expected result set without duplicating the shape check in the sidecar.
- **Alternatives considered:**
  - Keep the explicit severity check and suppress the validator - rejected because that would preserve the architectural leak.

## Architectural Notes
This keeps the arch sidecars focused on scenario construction:
- build the `ProjectTree`
- call the owned assertions helper
- let the helper own the result-shape expectations

That aligns the family with the stricter `RS-TEST-16` boundary without changing any production rule logic.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs`
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/rs_arch_01_root_classification.rs`
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/rs_arch_03_no_dual_ownership.rs`
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/rs_arch_06_owner_family_enablement_coherence.rs`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_01_root_classification_tests/ambiguous_roots.rs`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_03_no_dual_ownership_tests/dual_ownership.rs`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_06_owner_family_enablement_coherence_tests/ownership_coherence.rs`
- `.worklogs/2026-03-26-183601-arch-test-support-boundary-cleanup.md`

## Open Questions / Future Considerations
- If `RS-TEST-16` tightens further, the next likely cleanup is to move additional reusable result-shape assertions into the arch assertions crate rather than keeping them in sidecars.
- `cargo` and `hexarch` still need the same post-`RS-TEST-16` review path if they continue to duplicate result-shape checks in sidecars.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/rs_arch_01_root_classification.rs` - owned file-set assertion helper for rule 01.
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/rs_arch_03_no_dual_ownership.rs` - owned file-set assertion helper for rule 03.
- `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/rs_arch_06_owner_family_enablement_coherence.rs` - owned file-set assertion helper for rule 06.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_01_root_classification_tests/ambiguous_roots.rs` - sidecar that previously duplicated severity checks.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_03_no_dual_ownership_tests/dual_ownership.rs` - sidecar that previously duplicated severity checks.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_06_owner_family_enablement_coherence_tests/ownership_coherence.rs` - sidecar that previously duplicated severity checks.

## Next Steps / Continuation Plan
1. Stage the arch sidecar edits together with this worklog.
2. Commit the arch fallout fix as a single checkpoint.
3. If the validator tightens again, revisit the arch assertions crate before adding more direct checks back into sidecars.
