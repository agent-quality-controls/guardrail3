# Refactor Cargo RS-TEST-16 Proof Boundary

**Date:** 2026-03-26 19:10
**Scope:** `apps/guardrail3/crates/app/rs/families/cargo/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions_common/**`, `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/**`, `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_*_tests/**`

## Summary
Refactored the `cargo` family so the stricter `RS-TEST-16` boundary no longer sees sidecars doing their own result-shape assertions. The semantic result matching now lives in owned assertions helpers, while runtime sidecars are reduced to scenario setup plus helper calls.

## Context & Problem
The new `RS-TEST-16` tightening exposed one remaining cargo-family issue: the `rs_cargo_03_allow_inventory` sidecar still directly inspected `CheckResult` shape and inventory/title data. The broader goal was to keep `test_support` generic and move semantic fixture/result checks into the assertions layer, without breaking the cargo family’s existing test coverage.

## Decisions Made

### Move shared result selection out of `test_support`
- **Chose:** Kept `apps/guardrail3/crates/app/rs/families/cargo/test_support/src/lib.rs` generic-only and moved semantic lint/result fixtures into the rule-local runtime test modules.
- **Why:** `test_support` is now meant to be reusable plumbing only. Rule-specific expected result sets belong with the rule they validate.
- **Alternatives considered:**
  - Keep the semantic fixture bodies in `test_support` with different names - rejected because that preserves the boundary leak.
  - Move them into one shared assertions helper module inside the same crate - rejected because `RS-TEST-03` now treats local private helper imports inside assertions as another boundary leak.

### Introduce a helper crate for shared matcher logic
- **Chose:** Added `crates/assertions_common` to hold the generic `ExpectedRuleResult` matcher and result filtering helpers, and made assertions modules import that helper crate instead of `crate::common`.
- **Why:** The validator now rejects `crate::common` as a local private backdoor from assertions modules. A separate helper crate keeps the code shared without violating the local import boundary.
- **Alternatives considered:**
  - Duplicate the matcher into every assertions module - rejected because it would be noisy and unnecessary.
  - Keep `mod common;` inside the assertions crate - rejected because it is exactly what `RS-TEST-03` now flags.

### Keep proof-bearing assertions local to each assertions module
- **Chose:** The owned assertions modules now contain the actual `assert_eq!` proof logic, while the helper crate only provides reusable matching/data helpers.
- **Why:** This keeps the proof site in the owned assertions module so the runtime sidecars can call into it cleanly, while still avoiding direct result-shape assertions in the sidecars themselves.
- **Alternatives considered:**
  - Make the helper crate itself own the proof macros - rejected because it would move the semantic boundary away from the owned assertions module.
  - Put the `assert_eq!` back in the runtime sidecars - rejected because that violates the tightened `RS-TEST-16` intent.

## Architectural Notes
The final structure is:
- helper crate for generic result selection and matching
- owned assertions module per rule for proof-bearing semantics
- runtime sidecars reduced to scenario setup and helper calls

That keeps `test_support` generic, keeps assertions reusable, and avoids reintroducing direct `result.title` / `result.inventory` / `result.file` checks in the sidecars.

## Information Sources
- Existing cargo family patterns in `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_*_tests/**`
- The cargo assertions modules in `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/**`
- The `rs/test` family’s tightened proof-boundary behavior and validator output from this session

## Open Questions / Future Considerations
- `RS-TEST-07` still reports warnings for cargo sidecars even though the family test suite passes and `rs validate` exits successfully. That looks like a validator proof-site recognition gap rather than a cargo-family correctness issue.
- If we want zero warnings, the next step is probably validator work, not another cargo-family refactor.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions_common/src/lib.rs` - shared matcher and expectation types.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/lib.rs` - owned assertions module surface after removing local shared plumbing.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/rs_cargo_03_allow_inventory.rs` - representative rule module with proof-bearing inventory assertions.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_03_allow_inventory_tests/cases.rs` - representative sidecar reduced to scenario setup plus helper calls.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_*_tests/mod.rs` - test module plumbing for the rewritten cargo sidecars.
- `.worklogs/2026-03-26-184557-cargo-test-support-boundary.md` - prior cargo boundary cleanup that moved semantic TOML fixtures out of `test_support`.

## Next Steps / Continuation Plan
1. Commit this cargo-family checkpoint as its own slice with the worklog staged alongside it.
2. If the goal is to remove the remaining `RS-TEST-07` warnings, investigate the validator’s proof-site detection rather than widening cargo sidecars again.
3. Keep `test_support` generic in future cargo work; any new semantic fixture data should stay in the assertions/runtime rule-local surface.
