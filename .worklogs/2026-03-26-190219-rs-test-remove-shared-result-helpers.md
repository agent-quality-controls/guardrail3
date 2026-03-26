# Remove Shared Result Helpers From Rs Test Support

**Date:** 2026-03-26 19:02
**Scope:** `apps/guardrail3/crates/app/rs/families/test/test_support/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/rs_test_*.rs`

## Summary
Removed the generic `rule_files` and `finding` selectors from the shared `rs/test` `test_support` crate and moved that tiny rule-bound result-selection logic into each owned assertions module. This keeps `test_support` generic-only while preserving the family’s existing assertions API shape for the runtime sidecars.

## Context & Problem
`RS-TEST-18` had already been tightened to reject semantic helper leakage from `test_support`, and the self-hosting `rs/test` family still exposed exactly that pattern in `test_support/src/lib.rs` through shared `rule_files(...)` and `finding(...)` helpers. Those functions were being reused by every assertions module, which meant the support crate was still acting as a semantic result-helper surface rather than a generic fixture/builders surface.

The user requirement for this slice was explicitly constrained to the `rs/test` family and asked for the cleanest targeted fix without widening the support surface or reverting concurrent edits elsewhere in the family.

## Decisions Made

### Move rule-bound result selection into owned assertions modules
- **Chose:** Kept `walk(...)` and `StubToolChecker` in shared `test_support`, but inlined `rule_files(...)` and `finding(...)` implementations into each `crates/assertions/src/rs_test_*.rs` module.
- **Why:** The result selectors are rule-owned semantic helpers, not generic support. Each assertions module already owns `RULE_ID`, so the selector logic is naturally local there.
- **Alternatives considered:**
  - Add a shared `common.rs` under the assertions crate — rejected because the current `RS-TEST-03` boundary treats sibling-module helper imports inside assertions as another private backdoor.
  - Keep the helpers in `test_support` and only rename them — rejected because that would preserve the exact semantic leak the stricter boundary is meant to remove.

### Do not widen the support crate with another helper layer
- **Chose:** Reduced `test_support` instead of creating a new shared result-helper crate or runtime-local helper module.
- **Why:** The task was to eliminate the leak without introducing another shared semantic surface. The assertions modules are already the correct owned location.
- **Alternatives considered:**
  - Introduce a `result_helpers.rs` support module under `test_support` — rejected because it is just the same semantic API under a different name.
  - Move the selectors into runtime-side sidecar modules — rejected because the selectors belong with the reusable assertions surface, not the sidecars.

### Keep the refactor isolated from concurrent RS-TEST-16 work
- **Chose:** Touched only `test_support` and the assertions crate, and left the separate in-flight runtime-side `RS-TEST-16` tightening alone.
- **Why:** The family currently has concurrent edits in `crates/runtime/src/*`; mixing those into this cleanup would create review noise and risk stepping on another slice.
- **Alternatives considered:**
  - Fold the active `RS-TEST-16` fallout into this commit — rejected because that is a different behavior change owned by a different set of files and already visible in the live validator output.

## Architectural Notes
The resulting boundary is:

- `test_support` owns generic filesystem/tree helpers plus `StubToolChecker`
- each assertions module owns its own rule-scoped result selectors
- runtime sidecars keep calling the same assertions APIs they used before

This is the same direction already taken in the recent `arch`, `cargo`, and `hexarch` support-boundary cleanups, but adapted for the `rs/test` family’s self-hosting constraints. The important detail is that `rs/test` cannot solve this cleanly with an assertions-wide shared helper module because the family also validates local private-boundary escapes.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs` — previous shared semantic helper surface
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/rs_test_*.rs` — owned assertions modules that now hold the local selectors
- `apps/guardrail3/crates/app/rs/families/test/README.md` — support-boundary contract
- `.worklogs/2026-03-26-183601-arch-test-support-boundary-cleanup.md` — prior family-level pattern for moving semantic helpers out of shared support
- `.worklogs/2026-03-26-184557-cargo-test-support-boundary.md` — same cleanup pattern applied to cargo
- `.worklogs/2026-03-26-184203-hexarch-test-support-boundary-fix.md` — related runtime-local support-boundary tradeoff

## Open Questions / Future Considerations
- The live `rs validate ... --family test` run on the `rs/test` family is currently red because of separate concurrent `RS-TEST-16` tightening in many runtime sidecars. That is not caused by this support-boundary change.
- If the assertions modules accumulate more shared result-shape logic later, the correct next step is probably a macro or code-generation pattern local to each file, not a new shared helper module that recreates the same hidden boundary.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs` — generic-only support surface after this change
- `apps/guardrail3/crates/app/rs/families/test/test_support/Cargo.toml` — dependency surface reduced after removing `CheckResult`
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/rs_test_18_test_support_generic.rs` — representative assertions module now owning its own selectors
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/rs_test_03_runtime_assertions_split.rs` — representative assertions module with `run_family_with_tool(...)` plus local selectors
- `.worklogs/2026-03-26-183601-arch-test-support-boundary-cleanup.md` — prior pattern for moving semantic helpers out of shared support
- `.worklogs/2026-03-26-184557-cargo-test-support-boundary.md` — adjacent cleanup showing the same boundary in another family

## Next Steps / Continuation Plan
1. Stage only the `rs/test` assertions/test_support files plus this worklog.
2. Commit this support-boundary cleanup independently from the concurrent runtime-side `RS-TEST-16` work.
3. If the self-hosting family still wants stronger DRY guarantees later, design a self-hosting-safe assertions-helper pattern that does not reintroduce a shared semantic helper module.
