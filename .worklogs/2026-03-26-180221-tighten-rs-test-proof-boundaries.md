# Tighten RS-TEST Proof And Boundary Detection

**Date:** 2026-03-26 18:02
**Scope:** `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/*`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site_tests/*`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove_tests/*`

## Summary
Tightened `RS-TEST` so proof-bearing assertions are resolved by fully qualified module path rather than bare function name, and so sidecar harnesses fail when they escape upward to crate-root helpers. Added regressions for both bug classes and reran the family validator on `test`, `arch`, `cargo`, and `hexarch`.

## Context & Problem
An adversarial pass over the rewritten Rust families found that `RS-TEST` was still too permissive in two concrete ways. First, `RS-TEST-07` / `RS-TEST-16` could false-green a module if another assertions module exported a same-named proof-bearing function. Second, `RS-TEST-03` still missed sidecars that escaped the owned rule boundary through `super::...` or crate-root helper paths. Those holes let `cargo` and especially `hexarch` look cleaner than they actually were.

## Decisions Made

### Qualify assertion proof by module path
- **Chose:** Track proof-bearing assertions by fully qualified module path derived from the assertions source file path.
- **Why:** Matching only on function name allowed same-name collisions across assertions modules and made the proof catalog unsound.
- **Alternatives considered:**
  - Keep bare-name matching and add more tests — rejected because the underlying identity model was wrong.
  - Infer proof through broader heuristics at call sites — rejected because that would stay ambiguous and make false positives harder to reason about.

### Fail sidecars on upward local boundary escapes
- **Chose:** Add explicit `RS-TEST-03` detection for local sidecar imports and calls that walk above the owned module boundary.
- **Why:** The existing sibling-module checks caught obvious `crate::other` and some `super::...` patterns, but crate-root helper escapes were still getting through.
- **Alternatives considered:**
  - Leave escape detection to sibling-module classification only — rejected because crate-root helpers are not sibling modules and were still bypassing the rule.
  - Ban all `self` / `super` in sidecars — rejected because legitimate local sidecar organization needs bounded local navigation.

### Treat the new failures as validator wins, not family regressions
- **Chose:** Keep the stricter rule behavior and let `hexarch` fail.
- **Why:** The newly surfaced `hexarch` findings are exactly the boundary leaks the attack review called out earlier.
- **Alternatives considered:**
  - Weaken the rule until all rewritten families stayed green — rejected because that would preserve the loophole the adversarial review found.

## Architectural Notes
The proof catalog in `rs/test` now behaves more like a true symbol table: exported assertions are identified by module-qualified path, and proof propagation respects local relative imports instead of flattening everything by final segment name. `RS-TEST-03` is also now explicitly about ownership depth, not just sibling-module references, which better matches the contract in the test-family README and plan.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/README.md` — active test-family contract
- `.plans/todo/checks/rs/test.md` — mirrored rule plan and stricter ownership intent
- `.worklogs/2026-03-26-155851-tighten-rs-test-boundaries.md` — earlier RS-TEST tightening checkpoint
- `.worklogs/2026-03-26-172253-tighten-rs-test-route-boundaries-and-runtime-test-entrypoints.md` — prior route-boundary hardening
- `.worklogs/2026-03-26-173240-hexarch-rs-test-21-23-helper-cleanup.md`
- `.worklogs/2026-03-26-173242-hexarch-rs-test-03-boundary-cleanup.md`
- `.worklogs/2026-03-26-173238-hexarch-24-25-rs-test-03-boundary-fix.md`
- Local adversarial review against `arch`, `cargo`, and `hexarch` family rewrites

## Open Questions / Future Considerations
- `RS-TEST-16` is still too weak to enforce that assertions modules own most of the semantic checking; it now avoids same-name false greens but still allows thin wrappers.
- `RS-TEST-18` still allows family-specific semantic constants in `test_support`; this checkpoint did not tighten that contract yet.
- `hexarch` now fails `RS-TEST-03` on real crate-root/runtime helper escapes and needs a follow-on cleanup pass.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — proof catalog construction and shared RS-TEST orchestration
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — sidecar/assertions boundary enforcement
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site.rs` — proof-site detection against owned assertions
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs` — RS-TEST-03 attack regressions
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site_tests/qualified_assertions.rs` — same-name proof collision regression
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove_tests/qualified_assertions.rs` — same-name proof collision regression for assertions modules
- `.worklogs/2026-03-26-172253-tighten-rs-test-route-boundaries-and-runtime-test-entrypoints.md` — earlier RS-TEST hardening context
- `.worklogs/2026-03-26-173242-hexarch-rs-test-03-boundary-cleanup.md` — immediate prior `hexarch` cleanup context

## Next Steps / Continuation Plan
1. Fix the newly exposed `hexarch` `RS-TEST-03` failures in `crates/runtime/src/dependency_facts_tests/cycle_collection.rs` and `crates/runtime/src/rs_hexarch_20_dev_dependency_direction_tests/broad_attacks.rs` by routing those sidecars through owned module helpers instead of crate-root/runtime helpers.
2. Tighten `RS-TEST-18` so `test_support` cannot expose family-semantic constants and canned policy data; expect `arch`, `cargo`, and `hexarch` follow-on rewrites when that lands.
3. Tighten `RS-TEST-16` further so sidecars cannot keep the substantive result-shape assertions while assertions modules provide only token proof-bearing wrappers.
