# Add RS-ARCH-06 App Scope Coverage

**Date:** 2026-03-29 22:11
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_06_owner_family_enablement_coherence_tests/ownership_coherence.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`

## Summary
Added the missing adversarial coverage for `RS-ARCH-06` app-scoped `hexarch` enablement and nested app-owned roots. The implementation did not need to change; the new tests proved the current owner derivation and effective enablement logic already behaves correctly.

## Context & Problem
The earlier `arch` audit identified `RS-ARCH-06` as under-tested. The family had coverage for global `hexarch`/`libarch` off, but not for the more interesting branch where app-scoped `hexarch` overrides differ from the global setting. That left a plausible false-green gap around how the owner app name is derived and whether nested app-owned roots inherit the correct effective enablement.

Since the logic in `facts.rs` already reconstructs owner app names from `owner_root_rel`, the right next move was to tighten the tests first rather than change code speculatively.

## Decisions Made

### Add adversarial unit coverage before touching implementation
- **Chose:** Extend the `RS-ARCH-06` family tests with app-scoped override false/true cases, including nested app-owned roots.
- **Why:** This isolates whether there is a real implementation bug or just a coverage gap. The logic is compact enough that new tests give better signal than speculative refactoring.
- **Alternatives considered:**
  - Refactor `governed_root()` immediately — rejected because there was no failing case yet.
  - Only add a top-level runtime test — rejected because the family-local matrix needed direct ownership-derivation proof too.

### Prove both override directions
- **Chose:** Cover both:
  - global `hexarch = true` with app-scoped `hexarch = false`
  - global `hexarch = false` with app-scoped `hexarch = true`
- **Why:** Both branches matter. One proves that the app-scoped override can disable an otherwise healthy app; the other proves that an app-specific enable can resurrect a root when the global default is off.
- **Alternatives considered:**
  - Test only the “false beats true” branch — rejected because it still leaves the re-enable path unproven.

### Add a product-surface runtime regression too
- **Chose:** Add a top-level `arch` runtime test for app-scoped `hexarch = false`.
- **Why:** `RS-ARCH` already had product/runtime regressions for config and fail-closed behavior. `RS-ARCH-06` now has one too, so the scoped-override contract is not only encoded in family-local tests.
- **Alternatives considered:**
  - Keep this only in the family crate — rejected because product-surface regressions have caught real `arch` drift before.

## Architectural Notes
These tests validate the current contract without changing the architecture split:
- `placement` still owns shared structural scope.
- `arch` still reconstructs effective owner-family coherence locally.
- owner app resolution via `owner_root_rel.rsplit('/')` is currently sufficient for app-scoped overrides keyed by app name.

The important outcome is that nested app-owned roots under the same app root inherit the app-scoped `hexarch` setting as intended. That reduces the practical risk around the current local reconstruction until a later shared-route cleanup happens.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs` — current owner-family coherence logic.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_06_owner_family_enablement_coherence.rs` — pure rule behavior.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — product/runtime coverage patterns for `arch`.
- `.worklogs/2026-03-29-220707-fix-rs-arch-07-governed-manifest-fail-closed.md` — immediately prior `arch` fix and continuation context.

## Open Questions / Future Considerations
- `RS-ARCH-06` now has better coverage, but the family still locally reconstructs effective owner-family facts instead of consuming them from a shared routed view. That remains an architectural hardening opportunity.
- Dead `arch_role = "auxiliary"` metadata on governed roots is still unresolved and should likely become its own tightening task.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs` — current app/package effective enablement resolution.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_06_owner_family_enablement_coherence_tests/ownership_coherence.rs` — new adversarial app-scope matrix.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — product-surface scoped-override regression.
- `.worklogs/2026-03-29-220707-fix-rs-arch-07-governed-manifest-fail-closed.md` — prior `arch` fail-closed work.

## Next Steps / Continuation Plan
1. Decide how to surface dead `arch_role = "auxiliary"` metadata on governed roots and implement that tightening in placement + `RS-ARCH`.
2. Clean stale `arch` docs/handoffs so the current README and rule plan are the obvious source of truth.
3. If further `arch` hardening is needed, move owner-family/effective-enablement facts into routed views so the family stops reconstructing them locally.
