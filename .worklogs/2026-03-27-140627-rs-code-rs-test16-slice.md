# Finish RS-CODE RS-TEST-16 cleanup for rules 01-11

**Date:** 2026-03-27 14:06
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_01_*` through `rs_code_11_*`, `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/rs_code_01_*` through `rs_code_11_*`, `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/common.rs`

## Summary
Finished the `RS-TEST-16` cleanup slice for the `code` family rules `01-11` by moving sidecar semantic proof into owned assertions-module helper calls. I also made the assertions helpers order-insensitive so the rewritten sidecars can assert on normalized findings without depending on incidental output order.

## Context & Problem
The `code` family was already mid-migration to the `runtime` / `assertions` / `test_support` layout, but the `01-11` rule sidecars still contained semantic proof patterns that `RS-TEST-16` flags. The immediate goal was to eliminate those direct proof sites for rules `01-11` only, without touching later rules or trying to fix unrelated compile/test debt elsewhere in the family.

## Decisions Made

### Move proof into owned assertions helpers
- **Chose:** Reworked the sidecars under rules `01-11` so they call the rule-owned assertions helpers instead of unpacking and reasoning over results directly.
- **Why:** This keeps the semantic proof in the owned assertions module, which is what `RS-TEST-16` expects.
- **Alternatives considered:**
  - Leave the sidecars as direct result inspectors — rejected because they keep the proof in the wrong layer.
  - Move only some of the checks into assertions — rejected because partial migration still leaves `RS-TEST-16` noise.

### Normalize findings before comparison
- **Chose:** Added sorting/normalization helpers in `crates/assertions/src/common.rs` and used them in the `01-11` assertions modules.
- **Why:** Some test cases compare result sets where ordering is not semantically meaningful. Normalizing the findings avoids brittle order-dependent assertions while keeping the checks exact.
- **Alternatives considered:**
  - Preserve raw ordering and update every test to match — rejected because it makes the rewritten slice more fragile than necessary.
  - Use a looser subset matcher — rejected because it would weaken the assertions too much.

## Architectural Notes
The slice keeps the ownership boundary intact:
- runtime sidecars still define the attack vectors and fixtures
- assertions modules own the semantic proof
- `common.rs` only provides comparison normalization, not family semantics

This preserves the intended `runtime -> assertions` split without widening the scope beyond the `01-11` rules.

## Information Sources
- Existing `code` family patterns under `crates/runtime/src/rs_code_01_*` through `rs_code_11_*`
- Existing assertions layout under `crates/assertions/src/rs_code_01_*` through `rs_code_11_*`
- Validator run:
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family test --inventory --verbose`

## Open Questions / Future Considerations
- Rules `12-30` still have unrelated debt and dirty working-tree state from earlier work; they were intentionally left untouched.
- `common.rs` now contains normalization helpers that may be reusable by later slices, but only if they stay clearly comparison-only.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_01_crate_level_allow_tests/bypasses.rs` — example sidecar rewritten to use owned assertions helpers.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_07_exception_comment_inventory_tests/inventory.rs` — representative inventory-style sidecar rewrite.
- `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/rs_code_01_crate_level_allow.rs` — rule-owned assertions module updated to normalize findings.
- `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/common.rs` — shared normalization/comparison helpers.
- `.worklogs/2026-03-27-132300-start-rs-code-stabilization.md` — prior checkpoint describing the `code` family migration state.

## Next Steps / Continuation Plan
1. Stage only the `01-11` `code` family runtime sidecars, their matching assertions modules, and the shared comparison helper changes.
2. Commit this slice with a short message that identifies it as the `RS-TEST-16` cleanup checkpoint.
3. If the next slice continues the `code` family migration, start with the remaining rules only after rechecking that the `01-11` validator result stays clean.
