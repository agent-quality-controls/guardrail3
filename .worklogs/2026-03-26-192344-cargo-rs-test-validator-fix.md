# Cargo RS-TEST Validator Fix

**Date:** 2026-03-26 19:23
**Scope:** `apps/guardrail3/crates/app/rs/families/cargo/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/rs_cargo_*.rs`, `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_*_tests/cases.rs`

## Summary
Refactored the cargo family to satisfy the hardened `RS-TEST` validator without changing rule behavior. The sidecars no longer import the runtime crate directly, the owned assertions modules now contain real proof-bearing assertion helpers, and the family validates cleanly under `RS-TEST` with zero findings.

## Context & Problem
After the validator hardening, `cargo` still had three structural problems:
- internal sidecar files imported `guardrail3_app_rs_family_cargo as runtime` directly, which tripped `RS-TEST-03`
- per-rule assertions modules only exposed thin boolean wrappers, which tripped `RS-TEST-16`
- sidecars called those thin helpers, which still triggered `RS-TEST-07`

The requirement for this slice was to preserve behavior while moving execution ownership and proof-bearing assertions into allowed places inside the cargo family only.

## Decisions Made

### Keep execution behind owned assertion helpers
- **Chose:** Removed the runtime-crate alias from every cargo sidecar and had them call `check_results(...)` from the owned assertions surface instead.
- **Why:** This keeps the sidecars focused on scenario setup while stopping direct runtime-crate imports from crossing the validator boundary.
- **Alternatives considered:**
  - Leave the runtime alias in sidecars - rejected because it is exactly the `RS-TEST-03` violation the validator is meant to catch.
  - Move execution back into each sidecar - rejected because that would preserve the boundary leak and make the test architecture worse.

### Make assertions modules proof-bearing
- **Chose:** Converted `assert_rule_results(...)` into real assertion helpers that use `assert_eq!` / `assert!` instead of returning a plain boolean result.
- **Why:** The validator needs the assertions modules themselves to own the proof site, not just expose a thin result selector.
- **Alternatives considered:**
  - Keep the boolean wrappers and rely on the sidecars to prove the rule - rejected because that is the exact hollow pattern the validator now rejects.
  - Collapse all proof logic into a shared helper crate - rejected because the proof-bearing helper needs to stay in the owned per-rule assertions module.

### Keep shared helpers generic
- **Chose:** Left `assertions_common` as the shared execution/result-selection helper crate and removed a dead `project-tree` dependency from the cargo assertions crate.
- **Why:** The shared crate can own generic plumbing, while the per-rule assertions modules own the proof-bearing assertion logic.
- **Alternatives considered:**
  - Move everything into the assertions crate root - rejected because it would just recreate a shared semantic backdoor.
  - Keep an unused `project-tree` dependency around - rejected because the repo lints deny unused crate dependencies.

## Architectural Notes
The final cargo shape is:
- `assertions_common` owns generic `check_results(...)` and result-selection plumbing
- each `crates/assertions/src/rs_cargo_*.rs` file owns the proof-bearing assertion helper
- runtime sidecar files only build scenarios and call the owned assertion helpers

That matches the hardened `RS-TEST` contract while preserving the existing cargo rule behavior.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions_common/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/rs_cargo_*.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_*_tests/cases.rs`

## Open Questions / Future Considerations
- `cargo` now validates cleanly under `RS-TEST`, but the pattern used here may need to be repeated in other families that still expose thin result wrappers.
- If the validator tightens further, the next likely pressure point is whether other helper crates are being treated as semantic backdoors.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/rs_cargo_01_workspace_lints.rs` - representative proof-bearing assertions module.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/src/rs_cargo_03_allow_inventory.rs` - cargo-specific inventory helper that now asserts directly.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_01_workspace_lints_tests/cases.rs` - representative sidecar with the runtime alias removed.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions_common/src/lib.rs` - shared generic execution/result helper crate.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions/Cargo.toml` - dependency surface after dropping the unused `project-tree` dependency.

## Next Steps / Continuation Plan
1. Stage the cargo-family edits together with this worklog.
2. Commit the cargo RS-TEST validator fix as a clean cargo-only checkpoint.
3. If a future validator pass finds another family with the same thin-wrapper pattern, repeat this same split: generic helper crate plus per-rule proof-bearing assertions.
