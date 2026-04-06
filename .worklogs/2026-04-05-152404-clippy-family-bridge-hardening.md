# Harden Clippy Family Bridge And Remove Dead App Rule Surface

**Date:** 2026-04-05 15:24
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_25_config_parseable/tests/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_01_max_struct_bools/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_02_max_fn_params_bools/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_08_too_many_lines_threshold/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_09_too_many_arguments_threshold/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_10_excessive_nesting_threshold/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_15_test_relaxations/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_19_cognitive_complexity_threshold/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_20_type_complexity_threshold/`

## Summary
Added family-level smoke tests that prove migrated clippy rules are actually being executed through the app-family bridge, then removed the dead app-side rule directories for the moved clippy slice so the runtime tree no longer claims those rules still live in the app crate.

## Context & Problem
After extracting the first `clippy` content slice into `g3rs-clippy-config-checks`, the package had direct tests and the app family passed overall, but there was still a trust gap:

- nothing at the app-family layer explicitly proved that `run.rs` was calling the package correctly for a migrated rule
- the old app-side rule directories for `RS-CLIPPY-CONFIG-01`, `03`, `09`, `10`, `11`, `17`, `21`, and `22` were still present on disk even though `lib.rs` no longer referenced them

That left two risks:
- the family bridge could regress silently while package tests still passed
- future work could misread the app tree and assume those rule directories were still live production code

## Decisions Made

### Add bridge smoke tests at the family orchestration layer
- **Chose:** Add two tests under `RS-CLIPPY-25`'s test module using its existing `run_family_for_tests` helper, one for a migrated-rule inventory path and one for a migrated-rule error path.
- **Why:** `run_family_for_tests` already exercises the real family `check(...)` path, including legality mapping, family routing, typed parse gating, package invocation, and result conversion. That is the right seam for testing the bridge rather than adding another bespoke harness.
- **Alternatives considered:**
  - Add more package tests only — rejected because package tests do not prove the app bridge wiring.
  - Add a brand-new clippy family-wide test module — rejected because the existing `RS-CLIPPY-25` harness already owns the family parse/bridge seam and keeps the change small.

### Remove dead app-side moved-rule directories
- **Chose:** Delete the old app runtime directories for the migrated clippy rules.
- **Why:** They were no longer referenced from `lib.rs` and had become misleading dead surface. Leaving them in place weakens architectural clarity and makes future maintenance riskier.
- **Alternatives considered:**
  - Leave them for historical reference — rejected because git history already preserves them, and the live tree should reflect the actual runtime ownership.
  - Keep just the tests — rejected because they depended on dead local rule modules and would continue lying about what the app crate owns.

## Architectural Notes
This change hardens the intended extraction boundary without widening it:

- `g3rs-clippy-config-checks` continues to own the migrated typed-content rules
- the app clippy family continues to own typed parse gating through `RS-CLIPPY-25`
- the new tests prove the app still invokes the package and maps its results back into app `CheckResult`s

Using `RS-CLIPPY-25`'s helper is intentional: that rule already sits at the boundary between parse gating and downstream content checks, so its family-level harness is the cleanest place to verify package execution.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/run.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_25_config_parseable/tests/helpers.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/fixtures.rs`
- `packages/g3rs-clippy-config-checks/crates/runtime/src/run.rs`
- `.worklogs/2026-04-05-145142-clippy-extraction-and-parser-contract-fixes.md`

## Open Questions / Future Considerations
- The family now has bridge smoke tests for one migrated threshold rule. If the clippy extraction expands materially, add another bridge test for a different migrated rule class rather than assuming one smoke test is enough forever.
- `g3rs-deny-config-checks` still needs similar hardening at the package level; its confidence is still carried mostly by app-family tests.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/run.rs` — app/package bridge for moved clippy rules.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_25_config_parseable/tests/helpers.rs` — family-level harness used to exercise real package routing.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_25_config_parseable/tests/family_bridge.rs` — new bridge smoke coverage.
- `packages/g3rs-clippy-config-checks/crates/runtime/src/run.rs` — package entrypoint for the migrated clippy slice.
- `.worklogs/2026-04-05-145142-clippy-extraction-and-parser-contract-fixes.md` — prior extraction and parser-compatibility context.

## Next Steps / Continuation Plan
1. Run an adversarial review of the clippy extraction against the current family plan, focusing on bridge coverage, parse-owner boundaries, and any leftover false-positive or dead-surface risks.
2. Add direct runtime tests inside `g3rs-deny-config-checks` so deny package behavior is exercised locally rather than only through the app family.
3. Start mapping `cargo` rules into `content package` vs `app structural` ownership before scaffolding `g3rs-cargo-config-checks`.
