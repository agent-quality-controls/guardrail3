# Hexarch Test Support Boundary Fix

**Date:** 2026-03-26 18:42
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/test_support.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/hexarch/test_support/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/test_support/Cargo.toml`, and hexarch runtime test call sites that now use the runtime-local helper module

## Summary
I finished the hexarch support-boundary refactor by keeping the external `test_support` crate generic only and moving the family-specific fixture wrapper into the runtime crate's private test module. The last regression was a fixture-root path shift caused by moving the helper from `hexarch/test_support` to `hexarch/crates/runtime`; correcting the relative path restored the fixture copy behavior and brought the full hexarch test suite back to green.

## Context & Problem
`RS-TEST-18` now rejects semantic fixture APIs in `test_support`, including canned wrapper shapes and zero-arg fixture accessors. Hexarch had been exposing `RUST_APPS`, `INNER_HEX*`, `hexarch_fixture()`, and `copy_fixture()` from the shared support crate, which made `test_support` a semantic backdoor instead of a generic helper surface. The goal here was to remove those APIs from the shared crate without breaking the very large hexarch fixture suite.

When I first moved the wrapper into the runtime crate, the suite started failing broadly because the copied fixture root path was still written for the old `test_support` crate location. The runtime-local helper was resolving to `apps/guardrail3/crates/tests/fixtures/...` instead of `apps/guardrail3/tests/fixtures/...`.

## Decisions Made

### Keep semantic fixture wrappers runtime-local, not shared
- **Chose:** Removed the hexarch-specific fixture wrapper APIs from `hexarch/test_support/src/lib.rs` and kept only the generic tree/filesystem helpers there.
- **Why:** This satisfies the stricter `RS-TEST-18` boundary without broadening the shared support crate.
- **Alternatives considered:**
  - Keep the wrapper in `test_support` - rejected because the whole point of the stricter rule is to stop that semantic leakage.
  - Move fixture semantics into a sibling helper crate - rejected because that would preserve a second shared semantic surface.

### Preserve test ergonomics with a runtime-local helper module
- **Chose:** Added a private `test_support` module under `crates/runtime/src/` containing the hexarch-specific fixture wrapper plus the generic filesystem/tree helpers that the runtime tests already use.
- **Why:** The runtime crate can own its own test-only helper surface without violating the shared support boundary.
- **Alternatives considered:**
  - Rewrite all tests to inline the fixture path logic - rejected because it would create churn with no architectural gain.
  - Move the fixture path knowledge into each test module - rejected because the shared runtime-local helper keeps the code smaller and easier to maintain.

### Fix the root path after the helper move
- **Chose:** Updated the runtime-local `GOLDEN_REL` path to account for the helper living one directory deeper than the old external crate.
- **Why:** The helper copy logic itself was fine; only the relative fixture root changed. Without this fix, the test suite copied from a non-existent `crates/tests/fixtures/...` location.
- **Alternatives considered:**
  - Change the fixture layout in the repo - rejected because the fixture tree is already shared and the failure was caused by the helper relocation, not by the fixture content.

## Architectural Notes
The final shape is:
- shared `hexarch/test_support` is generic only
- runtime-local `crate::test_support` owns the family-specific fixture wrapper and fixture-copy root
- runtime-side sidecars consume the runtime-local helper, not the shared support crate

That keeps `RS-TEST-18` satisfied while preserving the existing hexarch test behavior. The only semantic knowledge left in shared support is the generic tree/filesystem plumbing.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs` for the stricter support-boundary rule
- `apps/guardrail3/crates/app/rs/families/hexarch/test_support/src/lib.rs` for the shared support surface that was reduced
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/test_support.rs` for the runtime-local helper and the corrected fixture root
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` for the runtime crate wiring
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_*_tests/**` for the callers switched over to the runtime-local helper

## Open Questions / Future Considerations
- `cargo run -- rs validate ... --family test` is still blocked in the wider workspace by unrelated `cargo` crate compile errors outside the hexarch slice.
- If `RS-TEST-18` tightens further, the remaining runtime-local wrapper API may need to be reduced again, but it is no longer part of the shared support crate.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/test_support.rs` - runtime-local helper module with the corrected fixture-root path.
- `apps/guardrail3/crates/app/rs/families/hexarch/test_support/src/lib.rs` - generic shared support only after this refactor.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` - runtime crate module wiring and test-only imports.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/Cargo.toml` - dev-dependency wiring for the runtime-local helper.
- `.worklogs/2026-03-26-183324-hexarch-test-support-wrapper.md` - previous step that introduced the wrapper split.
- `.worklogs/2026-03-26-180221-tighten-rs-test-proof-boundaries.md` - RS-TEST boundary tightening that motivated this cleanup.

## Next Steps / Continuation Plan
1. Commit this hexarch boundary checkpoint so the shared support crate stays generic only.
2. If the workspace compile blocker is cleared later, rerun `cargo run -- rs validate ... --family test` to confirm the validator sees this slice cleanly end-to-end.
3. Apply the same support-boundary pattern to any other families that still expose semantic fixture APIs from shared test-support crates.
