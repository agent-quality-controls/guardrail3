# Harden Garde Audit Findings

**Date:** 2026-03-23 12:08
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/garde/discover.rs`, `apps/guardrail3/crates/app/rs/checks/rs/garde/facts.rs`, `apps/guardrail3/crates/app/rs/checks/rs/garde/parse.rs`, `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_config_01_dependency_present_tests.rs`, `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_ast_02_manual_deserialize_impl_tests.rs`, `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_ast_04_query_as_inventory_tests.rs`

## Summary
Ran an adversarial hardening pass over the newly added `rs/garde` family and fixed the first real semantic hole it exposed: `RS-GARDE-AST-02` missed generic-form `Deserialize<'de>` impls. The pass also tightened alias handling, reduced same-file name-collision risk for manual impl resolution, narrowed the test-path skip heuristic, and added family-path tests for previously under-defended branches.

## Context & Problem
Immediately after landing the new `rs/garde` family, an adversarial review surfaced that the breadth-first implementation was structurally correct but still had real semantic blind spots:
- `RS-GARDE-AST-02` could miss manual `Deserialize<'de>` impls because the trait matcher compared rendered token text too literally.
- Alias forms like `use serde::Deserialize as De` and `use sqlx::query_as as qa` were not covered.
- Same-file module/type name collisions could cause manual impl resolution to match the wrong local type.
- The family’s test-path heuristic was broad enough to let production-ish paths like `src/tests.rs` evade source analysis.

The user explicitly wants the guardrails to fail hard and to keep hardening what the audits actually find, not to stop at “good enough for breadth-first”.

## Decisions Made

### Make manual impl matching alias-aware and generic-safe
- **Chose:** Switch `RS-GARDE-AST-02` trait detection from rendered path-string comparison to last-segment ident matching, and add alias collection from `use` trees for `Deserialize` / `Validate`.
- **Why:** The rendered-path approach missed `Deserialize<'de>` and aliased imports. The family-path test immediately proved that gap was real.
- **Alternatives considered:**
  - Keep the rendered-token comparison and just special-case `<...>` stripping — rejected because alias forms would still bypass it.
  - Full name-resolution across Rust scopes — rejected for this pass because it is much heavier than needed for the concrete bypasses found.

### Track qualified local type names instead of bare identifiers
- **Chose:** Add module-stack-aware qualified names in `parse.rs` and use them in the per-file type-validation map.
- **Why:** Bare names like `Payload` are too weak when multiple inline modules define the same type name in one file. Qualified names reduce same-file misattribution and suppression.
- **Alternatives considered:**
  - Keep bare names and rely on test discipline — rejected because the explorer identified a concrete suppression vector.
  - Full crate-wide semantic resolution — rejected for now as a deeper hardening step.

### Add a global/simple-name fallback for manual impl resolution
- **Chose:** In `facts.rs`, resolve manual impl targets first by exact qualified name, then by a simple-name map only when the simple name is globally unique.
- **Why:** This improves the cross-file `Validate` / type-definition case without regressing into obviously unsafe ambiguous matching.
- **Alternatives considered:**
  - Keep resolution file-local only — rejected because it creates noisy false positives whenever the impl and type definition are separated.
  - Resolve by bare simple name always — rejected because it would recreate the collision problem in a different form.

### Tighten the garde test-path skip heuristic
- **Chose:** Stop treating generic `/test/` and `/tests.rs` path shapes as test-only in `rs/garde`.
- **Why:** Those heuristics were broad enough to become a bypass surface. The family should skip obvious integration/unit-test locations, not arbitrary production subpaths that happen to contain `test`.
- **Alternatives considered:**
  - Keep the broad heuristic — rejected because it creates a silent evasion path.
  - Parse Cargo targets or `cfg(test)` to classify every file exactly — rejected for this pass as too much infrastructure for the immediate issue.

### Strengthen family-path tests, not only rule-isolation tests
- **Chose:** Add family-level tests for:
  - missing-garde gating
  - aliased/manual `Deserialize` impls
  - aliased `query_as!`
- **Why:** The isolated rule-input tests had already passed; the real bug lived in parse/facts/orchestrator flow.
- **Alternatives considered:**
  - Keep only isolated rule tests — rejected because they would not catch source-analysis regressions.

## Architectural Notes
This pass did not change the public family structure of `rs/garde`; it hardened the internal source-analysis model:
- `discover.rs` now uses a narrower skip heuristic for source files
- `parse.rs` now tracks:
  - module stack
  - relevant `use` aliases
  - qualified local type names
- `facts.rs` now resolves manual impl targets with a layered strategy:
  - current-file exact qualified name
  - global exact qualified name
  - unique global simple-name fallback

This keeps the family within the current architecture instead of reaching for a heavier cross-crate semantic engine prematurely.

## Information Sources
- `apps/guardrail3/crates/app/rs/checks/rs/garde/parse.rs` — source-analysis logic that needed hardening
- `apps/guardrail3/crates/app/rs/checks/rs/garde/facts.rs` — manual impl target resolution
- `apps/guardrail3/crates/app/rs/checks/rs/garde/discover.rs` — test-path filtering
- `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_config_01_dependency_present_tests.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_ast_02_manual_deserialize_impl_tests.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_ast_04_query_as_inventory_tests.rs`
- `.worklogs/2026-03-23-120029-complete-garde-family.md` — initial garde family implementation
- Explorer findings from the adversarial pass, especially the bypass analysis around aliasing and same-file collisions

## Open Questions / Future Considerations
- Alias handling is still syntactic, not full Rust name resolution. Local `use` aliases inside narrower scopes and more complex import patterns may still need a deeper parser pass later.
- The simple-name fallback in `facts.rs` intentionally stops when names are ambiguous. That is safer than guessing, but some legitimate cross-file cases may still remain unproven until a deeper hardening phase.
- `query_as_unchecked!` is now treated as equivalent inventory surface via the alias-aware macro-tail matcher, but the rule still intentionally remains inventory-only rather than hard failure.

## Key Files for Context
- `AGENTS.md` — project instructions and no-shortcuts checker architecture rules
- `.plans/todo/checks/rs/garde.md` — garde rule contract
- `apps/guardrail3/crates/app/rs/checks/rs/garde/parse.rs` — garde source-analysis logic
- `apps/guardrail3/crates/app/rs/checks/rs/garde/facts.rs` — garde root and source fact collection
- `apps/guardrail3/crates/app/rs/checks/rs/garde/discover.rs` — source file discovery / skip heuristic
- `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_ast_02_manual_deserialize_impl.rs` — rule affected by the main semantic bug
- `apps/guardrail3/crates/app/rs/checks/rs/garde/rs_garde_ast_04_query_as_inventory.rs` — alias-aware macro inventory surface
- `.worklogs/2026-03-23-120029-complete-garde-family.md` — prior garde implementation checkpoint

## Next Steps / Continuation Plan
1. If continuing the breadth-first sequence, start `rs/test` next with the same standard:
   - one rule per file
   - one rule-specific test module per rule
   - family-level adversarial tests for parser/orchestrator behavior where isolated rule tests are insufficient.
2. During the deeper hardening phase later, revisit `rs/garde` for:
   - richer cross-file type/impl resolution
   - broader alias/scope resolution
   - more exhaustive family-path adversarial tests around nested modules and generated query macros.
