# Layer RS-HEXARCH-01 Tests

**Date:** 2026-03-25 11:35
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/src/rs_hexarch_01_crates_exists_tests/*`, `apps/guardrail3/crates/app/rs/families/hexarch/src/tests/**`

## Summary
Started the actual layered-test decoupling in `hexarch` by splitting `RS-HEXARCH-01` tests into four real concerns: core rule tests, collector tests, orchestrator tests, and integration tests. The old mislayered rule-sidecar files were removed from `rs_hexarch_01_crates_exists_tests/`, and a new family-level `src/tests/` structure now owns the collector/orchestrator/integration layers.

## Context & Problem
The current family-local tests were giving a false sense of unit coverage. For `hexarch`, many tests stored under rule-sidecar directories were not exercising rule semantics at all; they were proving:
- project walking
- app-root discovery
- ownership boundaries across sibling rules
- broad golden-fixture attack matrices

That directly conflicts with the layered-test architecture note and the `hexarch` layered map. Continuing to “move tests into family crates” without fixing that would just preserve the wrong mental model in a different folder.

`RS-HEXARCH-01` was a good first cut because its current mislayering was very obvious:
- `discovery_scope.rs` was really about root discovery
- `nested_root.rs` / `ownership_boundaries.rs` were orchestrator ownership tests
- `golden.rs`, `broad_attacks.rs`, and `replacement_edges.rs` were integration tests
- there were effectively no true core rule tests

## Decisions Made

### Split `RS-HEXARCH-01` by test layer instead of by old file location
- **Chose:** Rewire `rs_hexarch_01_crates_exists_tests/mod.rs` so the rule-sidecar directory now contains only `core.rs`, and move the other behaviors into family-level layered test modules.
- **Why:** The point is to change what the tests claim to prove, not only where they live.
- **Alternatives considered:**
  - Keep the old file layout and just rename files — rejected because it would preserve the rule-sidecar lie.
  - Move everything into `tests/` integration harnesses immediately — rejected because the first step should preserve crate-private access for collector/orchestrator layers without widening public APIs.

### Add a family-level crate-internal layered test tree
- **Chose:** Add `src/tests/{collectors,orchestrator,integration}` and wire it from `lib.rs` as a crate-internal `#[cfg(test)] mod tests;`.
- **Why:** This keeps collector/orchestrator tests crate-private, avoids fake public test APIs, and establishes the layered shape before any later external-harness work.
- **Alternatives considered:**
  - Use only top-level Cargo `tests/*.rs` harnesses now — rejected for the first batch because collector tests still need direct crate-private access and the immediate goal was semantic relayering, not public-surface testing.

### Create true core rule tests for `RS-HEXARCH-01`
- **Chose:** Add `core.rs` with tiny typed `AppHexarchInput` tests.
- **Why:** `RS-HEXARCH-01` should prove its rule contract directly: “no top-level `crates/` entries means fail; any entries means pass.”
- **Alternatives considered:**
  - Continue letting fixture-backed tests stand in for rule semantics — rejected because that is the exact misattribution being fixed.

### Re-express discovery tests as collector assertions
- **Chose:** Replace the old `discovery_scope.rs` family-run tests with collector tests that call `facts::collect(...)` over walked trees and inspect discovered app roots.
- **Why:** The old tests were really asking “which app roots are discovered?”, not “does rule 01 fire?”
- **Alternatives considered:**
  - Move the old file unchanged into a collector folder — rejected because it still asserted final rule output instead of collector output.

## Architectural Notes
This batch is intentionally only `RS-HEXARCH-01`, not `01..06` wholesale. The goal was to prove the migration pattern first:
- rule-sidecar core tests stay tiny and typed
- collector behavior moves out of rule-sidecars
- orchestrator ownership tests move to a family layer
- golden and broad attacks move to integration

The new family-level structure is:
- `src/tests/collectors/structural_roots.rs`
- `src/tests/orchestrator/structural_ownership.rs`
- `src/tests/integration/structural_roots.rs`

This is still crate-internal. It does not widen the crate API. It is a semantic relayering first, not a public test-harness refactor.

## Information Sources
- `.plans/todo/checks/2026-03-25-rust-layered-test-architecture.md`
- `.plans/todo/check_review/test_hardening/33-hexarch-layered-test-architecture-note.md`
- `.plans/todo/check_review/test_hardening/31-hexarch-layered-test-map.md`
- `.plans/todo/check_review/test_hardening/32-hexarch-01-06-layered-migration-checklist.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/src/rs_hexarch_01_crates_exists.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/src/inputs.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/src/test_support.rs`

## Open Questions / Future Considerations
- The new collector/orchestrator/integration layers are crate-internal for now. If the family later needs explicit Cargo external harnesses, those should be added after the layered responsibilities are stable.
- `RS-HEXARCH-01` still leaves a lot of similar mislayering in `RS-HEXARCH-02..06`; those should follow this pattern next.
- `hexarch` test support still mixes fixture copying, walker execution, synthetic `ProjectTree` helpers, and dependency-fact helpers in one file. That is survivable for now, but likely needs its own substrate split later.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/src/lib.rs` — family entrypoint and new layered test root wiring
- `apps/guardrail3/crates/app/rs/families/hexarch/src/rs_hexarch_01_crates_exists.rs` — rule under migration
- `apps/guardrail3/crates/app/rs/families/hexarch/src/rs_hexarch_01_crates_exists_tests/core.rs` — new true rule-core tests
- `apps/guardrail3/crates/app/rs/families/hexarch/src/tests/collectors/structural_roots.rs` — new collector layer for rule-01 discovery behavior
- `apps/guardrail3/crates/app/rs/families/hexarch/src/tests/orchestrator/structural_ownership.rs` — new orchestrator layer
- `apps/guardrail3/crates/app/rs/families/hexarch/src/tests/integration/structural_roots.rs` — new integration layer
- `.plans/todo/check_review/test_hardening/31-hexarch-layered-test-map.md` — canonical mapping for `hexarch`
- `.plans/todo/check_review/test_hardening/32-hexarch-01-06-layered-migration-checklist.md` — concrete migration checklist for the next rules

## Next Steps / Continuation Plan
1. Apply the same split to `RS-HEXARCH-02` and `RS-HEXARCH-03`, which are the next most obviously mislayered structural rules.
2. Introduce collector-focused tests for top-level child enumeration and directional-container discovery instead of leaving those behaviors in rule-sidecars.
3. Move cross-rule ownership tests for `01..03` into shared orchestrator modules so the rule-sidecar directories stop carrying sibling-rule boundary assertions.
4. After `01..06` are relayered, reassess whether the family should add explicit external Cargo test harnesses for integration-only layers or keep the layered tree crate-internal.
