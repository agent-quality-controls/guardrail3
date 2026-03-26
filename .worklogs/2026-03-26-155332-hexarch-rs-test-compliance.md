# Complete Hexarch RS Test Compliance Migration

**Date:** 2026-03-26 15:53
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/*`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/*`, `apps/guardrail3/crates/app/rs/families/hexarch/test_support/src/lib.rs`, `apps/guardrail3/Cargo.lock`

## Summary
Refactored the entire `rs/hexarch` family into the RS-TEST-compliant three-crate workspace shape: `runtime`, `assertions`, and generic `test_support`. Production code and rule sidecars now live under `crates/runtime/src`, proof-bearing reusable test helpers live under `crates/assertions`, and the family passes both its own cargo test suite and the library-enforced `RS-ARCH` / `RS-TEST` validators at `0 errors, 0 warnings`.

## Context & Problem
The handoff in `.plans/todo/rs-test-compliance-handoffs/hexarch.md` described a family that still used the older single-crate layout with mixed production code, assertion semantics, and fixture helpers in one root. The RS-TEST validator was specifically failing on:
- sidecar module ownership shape
- assertions code reaching local private paths
- proof-bearing assertions being mixed into generic test support
- tests that looked structurally correct but were not recognized as using owned proof sites

The family was already functionally rich and heavily tested, so the goal was not to change hexarch rule semantics. The goal was to preserve behavior while making the family architecture legible to the library’s own test-family checks.

## Decisions Made

### Split hexarch into runtime / assertions / test_support crates
- **Chose:** Turn `families/hexarch` into a workspace with `crates/runtime`, `crates/assertions`, and `test_support`.
- **Why:** This is the shape the RS-TEST family expects and the same pattern already proven in compliant migrated families.
- **Alternatives considered:**
  - Keep the single crate and try to silence RS-TEST findings locally — rejected because that would fight the validator instead of matching the intended architecture.
  - Move only a handful of files and leave mixed helpers in place — rejected because RS-TEST findings were caused by the mixed boundaries themselves.

### Keep production rule/test locality inside runtime
- **Chose:** Move all production files and owned `*_tests/mod.rs` sidecars under `crates/runtime/src`.
- **Why:** RS-TEST requires owned sidecar shape relative to the production module, not a detached integration-style layout.
- **Alternatives considered:**
  - Move tests into integration tests or top-level harnesses — rejected because that breaks owned sidecar traceability and the family already used the rule-local sidecar model.

### Move only proof-bearing semantics into assertions
- **Chose:** Keep raw fixture/tempdir/tree helpers in `test_support`, but move reusable family execution and result assertion helpers into `crates/assertions`.
- **Why:** RS-TEST-18 wants generic support crates, while RS-TEST-03 and RS-TEST-16 want proof-bearing assertions to live in owned assertions modules/crates.
- **Alternatives considered:**
  - Keep `run_family` and result filters in `test_support` — rejected because that keeps semantic proof sites in the generic support layer.
  - Push every helper into assertions, including pure filesystem helpers — rejected because that makes assertions crates do generic setup work they should not own.

### Use rule-local assertions modules instead of one flat assertion API
- **Chose:** Give each rule its own module inside `crates/assertions` and make runtime sidecars import their owned module.
- **Why:** This aligns the proof site with the owning rule and avoids a giant shared test API that RS-TEST reads as underspecified.
- **Alternatives considered:**
  - Re-export all helpers at the assertions crate root — rejected because RS-TEST still treated many proof sites as ambiguous and because the root API blurred ownership.

### Fix RS-TEST warnings by making proof sites explicit, not by weakening checks
- **Chose:** For tests where the validator still failed to recognize helper-only proof, rewrite the test bodies to assert directly on `errors_by_id(...).is_empty()` or exact result sets.
- **Why:** This keeps the tests strong while making the proof obvious to the RS-TEST analyzer.
- **Alternatives considered:**
  - Add allowances or special cases to RS-TEST — rejected because the task was to make hexarch compliant with the existing library checks.
  - Leave helper-only wrappers and accept residual warnings — rejected because the target state was `0 errors, 0 warnings`.

## Architectural Notes
Hexarch now matches the intended family layout:

`family workspace`
`-> crates/runtime` for production rules, facts, inputs, and sidecar tests
`-> crates/assertions` for owned proof-bearing test helpers
`-> test_support` for generic fixture and filesystem setup

Two practical nuances mattered during the migration:
- The golden fixture path in `test_support` had to be corrected after moving the support crate deeper into the workspace tree.
- Some runtime tests use dependency-fact types defined in the runtime crate. Those tests now use runtime-local test helper exports when necessary, while assertions-facing proof stays in the assertions crate.

This migration preserves the rule inventory and rule file structure. The change is about family boundaries and proof ownership, not about rewriting hexarch rules into a different behavioral model.

## Information Sources
- `.plans/todo/rs-test-compliance-handoffs/hexarch.md` — direct migration requirements and expected validator end state
- `apps/guardrail3/crates/app/rs/families/test/...` — compliant specimen for runtime/assertions/test_support split
- `.worklogs/2026-03-26-141017-route-rs-hexarch-through-mapper.md` — most recent hexarch worklog before this compliance pass
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/*` — migrated runtime family code
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/*` — new owned assertions surface
- `apps/guardrail3/crates/app/rs/families/hexarch/test_support/src/lib.rs` — generic fixture and tree helpers

## Open Questions / Future Considerations
- `hexarch` itself is now compliant, but other Rust families still need the same RS-TEST compliance migration pattern.
- The family-specific handoff file served its purpose for this slice; future family migrations should keep using the same concrete verify loop: family tests, `RS-ARCH`, then `RS-TEST`.
- `Cargo.lock` moved because the workspace/member graph changed. If other family migrations happen in parallel, they should expect another lockfile churn pass.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/Cargo.toml` — family workspace root and member layout
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — runtime family entrypoint and public test helper exports
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/lib.rs` — owned assertions surface
- `apps/guardrail3/crates/app/rs/families/hexarch/test_support/src/lib.rs` — generic fixture/test support boundary
- `.plans/todo/rs-test-compliance-handoffs/hexarch.md` — migration checklist that drove this pass
- `.worklogs/2026-03-26-141017-route-rs-hexarch-through-mapper.md` — prior hexarch architectural checkpoint

## Next Steps / Continuation Plan
1. Commit this hexarch migration as a standalone slice so later family migrations do not get mixed into the same history.
2. If another family gets the same RS-TEST compliance task, reuse the same order:
   read the handoff, split workspace, migrate runtime, split assertions from test_support, then verify `cargo test`, `RS-ARCH`, and `RS-TEST`.
3. Keep using `CARGO_TARGET_DIR=target/<family>` per family to avoid cross-agent cargo lock contention while these family-by-family migrations continue.
