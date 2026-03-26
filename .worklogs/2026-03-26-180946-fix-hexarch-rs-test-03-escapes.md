# Fix Hexarch RS-TEST-03 Escapes

**Date:** 2026-03-26 18:09
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts_tests/cycle_collection.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_20_dev_dependency_direction.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_20_dev_dependency_direction_tests/broad_attacks.rs`

## Summary
Removed the three `hexarch` `RS-TEST-03` boundary escapes that were surfaced by the tightened `rs/test` family. The offending sidecars now call owner-module test helpers through `super::super` instead of reaching crate-root orchestration helpers.

## Context & Problem
After `RS-TEST-03` started rejecting upward local boundary escapes, `hexarch` failed on three real leaks:
- `dependency_facts_tests/cycle_collection.rs` reached a crate-root helper that built routes and collected dependency facts.
- `rs_hexarch_20_dev_dependency_direction_tests/broad_attacks.rs` reached crate-root family `check(...)` and `family_route_for_tests(...)`.

Those helpers were structurally convenient but violated the rule family’s own sidecar boundary contract.

## Decisions Made

### Move test entrypoints into the owned production modules
- **Chose:** Add `#[cfg(test)]` helper entrypoints inside `dependency_facts.rs` and `rs_hexarch_20_dev_dependency_direction.rs`.
- **Why:** Sidecars are allowed to reach their owned production module subtree. Moving the helpers there fixes the escape without weakening `RS-TEST-03`.
- **Alternatives considered:**
  - Relax `RS-TEST-03` for crate-root helpers — rejected because that would reopen the loophole the adversarial pass found.
  - Push these checks into assertions immediately — rejected because the issue here was boundary ownership, not proof-site ownership.

### Keep route construction local to the owner module helper
- **Chose:** Let the new owner-module test helpers construct the route internally.
- **Why:** The problem was the sidecar escaping to crate root, not the existence of a test-only route builder itself.
- **Alternatives considered:**
  - Thread a route from each sidecar fixture — rejected as noisy and not materially better for the local test intent.

## Architectural Notes
This keeps `hexarch` aligned with the stronger `RS-TEST-03` model: internal sidecars may use their own production module test surface, but they may not bypass that boundary by importing crate-root orchestration helpers. The route-building logic still exists, but now sits behind owner-module helpers rather than crate-root shortcuts.

## Information Sources
- `.worklogs/2026-03-26-180221-tighten-rs-test-proof-boundaries.md` — the checkpoint that surfaced these `hexarch` failures
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — current sidecar boundary enforcement
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — previous crate-root helper surface
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs` — owned dependency-facts module
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_20_dev_dependency_direction.rs` — owned rule module for the dev-dependency direction tests

## Open Questions / Future Considerations
- `hexarch` is clean again under the current `RS-TEST` contract, but `RS-TEST-18` still does not ban family-semantic constants in `test_support`.
- `RS-TEST-16` still allows relatively thin assertions modules as long as they contain some proof-bearing function.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs` — dependency-family fact collection plus new test-only owner helper
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts_tests/cycle_collection.rs` — sidecar now calling only into its owned module
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_20_dev_dependency_direction.rs` — rule module plus new test-only owner helper
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_20_dev_dependency_direction_tests/broad_attacks.rs` — sidecar now using owned rule helper
- `.worklogs/2026-03-26-180221-tighten-rs-test-proof-boundaries.md` — previous step that exposed the failures

## Next Steps / Continuation Plan
1. Rerun the adversarial RS-TEST review after any future `RS-TEST-18` or `RS-TEST-16` tightening; `hexarch` is the most likely family to regress first.
2. Tighten `RS-TEST-18` next so `test_support` stops carrying family-semantic fixture constants across `arch`, `cargo`, and `hexarch`.
3. Tighten `RS-TEST-16` after that so runtime sidecars cannot keep most of the semantic result-shape assertions while assertions modules provide only token proof-bearing wrappers.
