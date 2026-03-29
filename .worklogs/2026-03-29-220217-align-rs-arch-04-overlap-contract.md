# Align RS-ARCH-04 Overlap Contract

**Date:** 2026-03-29 22:02
**Scope:** `apps/guardrail3/crates/app/rs/placement/src/overlap.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_04_no_zone_overlap.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_04_no_zone_overlap_tests/*`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/app/rs/families/arch/README.md`, `.plans/todo/checks/rs/arch.md`

## Summary
Realigned `RS-ARCH-04` with the stricter architecture contract: illegal app/package containment is now emitted as a layout-level overlap finding even when the nested root is also ambiguous and dual-owned. The overlap collector now derives app/package pairs from actual discovered roots referenced by zone candidates, runtime expectations were updated to the current inventory contract, and docs now state that `01`, `03`, and `04` are intentionally distinct facts for the same bad cross-zone shape.

## Context & Problem
Adversarial review found that `RS-ARCH-04` was documented as a layout-level overlap rule but did not fire for the most important illegal shapes: `apps/<app>/packages/<pkg>` and `packages/<pkg>/apps/<app>`. The failure mode was structural rather than cosmetic. Placement only collected overlaps between roots already classified as `App` and `Package`, so the ambiguous nested cases never reached `RS-ARCH-04` at all. Tests had drifted to bless that under-enforcement, and runtime tests still assumed clean `arch` runs should emit no inventory infos.

The user explicitly chose the stricter contract: `RS-ARCH-04` should be correct and useful as a layout-level rule, not suppressed to avoid duplicate reporting. That required changing the overlap substrate, not just rewriting the tests.

## Decisions Made

### Derive overlap pairs from zone candidates, not final classification
- **Chose:** Build app/package overlap pairs from actual discovered roots referenced by app/package zone candidates on live roots.
- **Why:** Cross-zone nested roots are usually classified `Ambiguous`, so classification-based pairing cannot ever emit `RS-ARCH-04` for the shapes we care most about. Candidate-based pairing preserves real discovered-root semantics while allowing ambiguous nested shapes to surface the illegal containment pair.
- **Alternatives considered:**
  - Keep the current `App`-vs-`Package` collector — rejected because it permanently misses illegal nested cross-zone shapes.
  - Pair directly from zone-segment strings whether or not a root exists there — rejected because it could fabricate overlaps from hypothetical roots and increase false positives.

### Keep `RS-ARCH-04` as a pair/layout rule, not a duplicate root rule
- **Chose:** Leave `RS-ARCH-01` as ambiguous root classification, `RS-ARCH-03` as dual ownership, and let `RS-ARCH-04` independently report the illegal app/package containment pair.
- **Why:** This gives stricter and more robust enforcement. A bad nested cross-zone layout now reports the local symptom on the root and the structural cause in the layout. That is better future protection than suppressing `04` just because `01/03` already fire.
- **Alternatives considered:**
  - Suppress `04` whenever `01/03` already fire — rejected because it hides the pair-level geometry violation and makes the rule less useful.
  - Move the entire problem into `01/03` only — rejected because those are root-level facts, not layout-level facts.

### Report `RS-ARCH-04` on the nested offender path
- **Chose:** Emit the `RS-ARCH-04` result on the nested root’s `Cargo.toml`, whether the nested root is the app side or the package side.
- **Why:** This makes the result point at the contained offender instead of always pinning to the package root, which was confusing for `packages/<pkg>/apps/<app>` cases.
- **Alternatives considered:**
  - Always pin to the package root — rejected because it reports the wrong side for nested app-inside-package shapes.
  - Emit with no file — rejected because this loses actionable locality.

### Align runtime tests to the inventory contract instead of suppressing infos
- **Chose:** Update `arch` runtime tests to treat inventory info results as normal background output and only assert on live/non-inventory findings.
- **Why:** The repo already restored the inventory contract elsewhere. Making `arch` product tests demand empty result sets would incentivize hiding correct inventory output instead of checking the real live contract.
- **Alternatives considered:**
  - Change `arch` to suppress inventory in runtime tests — rejected because it would be a reporting regression, not a real fix.

## Architectural Notes
This change keeps the intended substrate split intact:
- `placement` still owns shared structural discovery and overlap pairing.
- `FamilyMapper` still routes overlap pairs into `RS-ARCH`.
- `RS-ARCH-04` remains a pure rule over one illegal pair.

The important architectural correction is that overlap pairing is now based on **actual discovered root pairs implied by zone-candidate geometry**, not on final root classification. That makes the pair substrate strict enough for the layout rule without turning `placement` into a policy engine.

The resulting contract is:
- `RS-ARCH-01`: root classification ambiguity
- `RS-ARCH-03`: dual owner-family claim on a root
- `RS-ARCH-04`: illegal app/package containment pair

Those can legitimately co-exist on the same bad nested shape.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — live family contract, especially the `RS-ARCH-04` section.
- `.plans/todo/checks/rs/arch.md` — live rule inventory and implementation notes.
- `apps/guardrail3/crates/app/rs/placement/src/classification.rs` — zone candidate and classification semantics.
- `apps/guardrail3/crates/app/rs/placement/src/overlap.rs` — shared pair collector changed in this work.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_04_no_zone_overlap_tests/illegal_nesting.rs` — attack case for the strict overlap contract.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — product-surface `arch` behavior.
- Agent audit results from this session identifying the `RS-ARCH-04` doc/implementation drift and recommending a stricter pair-level rule.

## Open Questions / Future Considerations
- `RS-ARCH-07` still needs a separate fix for malformed governed manifests fail-closed behavior. This change intentionally did not touch that.
- `RS-ARCH-06` still needs adversarial coverage for app-scoped `hexarch` overrides and same-zone nested ownership.
- The broader `arch` doc stack still contains stale historical files and dead path references; this change only clarified the `04` contract in the current README and rule plan.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/placement/src/overlap.rs` — shared overlap-pair collector; now candidate-driven instead of classification-driven.
- `apps/guardrail3/crates/app/rs/placement/src/classification.rs` — explains why nested cross-zone roots become ambiguous and why candidate-based pairing is necessary.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_04_no_zone_overlap.rs` — pure pair-level rule and result targeting.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_04_no_zone_overlap_tests/illegal_nesting.rs` — strict nested cross-zone attack case.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_04_no_zone_overlap_tests/false_positives.rs` — sibling and same-zone non-hit coverage.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — product-level inventory-vs-live expectation for `arch`.
- `.worklogs/2026-03-29-204728-restore-inventory-contracts.md` — prior inventory-contract context that informed the runtime test update.

## Next Steps / Continuation Plan
1. Fix `RS-ARCH-07` in `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs` and its tests so malformed governed app/package manifests fail closed instead of being silently accepted.
2. Add adversarial `RS-ARCH-06` tests covering app-scoped `hexarch` overrides and nested app-root ownership derivation in `rs_arch_06_owner_family_enablement_coherence_tests/`.
3. Clean the remaining `arch` docs by removing stale shape/path references and adding an explicit `RS-ARCH-08` section plus reporting-visibility wording.
