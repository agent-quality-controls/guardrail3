# Hexarch RS-TEST Fallout Fix

**Date:** 2026-03-26 20:51
**Scope:** `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/*`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/*`

## Summary
Removed the last `RS-TEST` fallout from `hexarch` after the validator tightened proof-site and sidecar-boundary checks. The family now stays inside its owned assertions modules, and the remaining proof helpers were made direct enough for the current validator to recognize them.

## Context & Problem
The current `RS-TEST` rules were rejecting `hexarch` for two reasons:
1. several cross-rule sidecars imported sibling assertions modules instead of staying inside their own owned assertions module
2. a number of owned assertion helpers were still thin wrappers around shared summary helpers, so the validator did not always treat test calls as real proof sites

There was also one semantic leakage case in `rs_hexarch_04_loose_files_tests/ownership.rs`, where a sidecar was asserting sibling-rule ownership directly. That was removed because the same scenario is already covered in the owning rule's tests.

## Decisions Made

### Keep sidecars on one owned assertions module
- **Chose:** rewrote cross-rule sidecars to import only their own owned assertions module and use explicit rule ids for cross-rule checks.
- **Why:** this satisfies `RS-TEST-03` without changing the fixture scenarios.
- **Alternatives considered:**
  - keep sibling imports and rely on validator exceptions — rejected because that was the original violation
  - move those checks into separate global tests — rejected because this family is intentionally self-hosted and should stay local

### Make proof helpers direct
- **Chose:** converted the hexarch assertion wrappers that were still delegating to generic summary helpers into direct set/count comparisons.
- **Why:** the tightened validator only trusts direct proof-bearing calls reliably; direct comparisons remove the ambiguity.
- **Alternatives considered:**
  - loosen the validator again — rejected because the stricter behavior is the point of the refactor
  - leave the wrappers thin and hope proof propagation would be enough — rejected because it still produced `RS-TEST-07` warnings

### Remove the duplicate rule-05 ownership assertion from rule 04
- **Chose:** kept the `rule 04` symlink scenario but dropped the sibling `rule 05` assertion from that sidecar.
- **Why:** `rule 05` already covers that ownership case, and the extra check was the remaining `RS-TEST-16` semantic leakage.
- **Alternatives considered:**
  - keep both assertions in one sidecar — rejected because it remained an ownership leak
  - move the duplicate check into `rule 05` only — not needed because the coverage already exists

## Architectural Notes
The important boundary now is:
- runtime sidecars set up scenarios
- owned assertions modules carry the proof
- sibling rule ownership checks are expressed through the current sidecar's own assertions module, not by importing peers

The `rs_hexarch_21_domain_purity` helper also had to be corrected so title-set checks allow duplicate findings with the same title. The earlier count-vs-title-length check was too strict and broke a valid test case.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- current `RS-TEST` validator output for `apps/guardrail3/crates/app/rs/families/hexarch`
- hexarch runtime sidecars under `crates/runtime/src`
- hexarch owned assertions modules under `crates/assertions/src`

## Open Questions / Future Considerations
- The hexarch assertions modules still contain a lot of shared helper surface. They are now direct enough for the current validator, but the API is larger than ideal.
- If `RS-TEST` tightens again, the next likely pressure point is more proof-site wrappers in the same style, especially in the domain-purity family.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_01_crates_exists.rs` — direct file-set proof helper used by the root crates checks.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_11_root_workspace_doesnt_include_apps.rs` — direct title/file helper for workspace-boundary checks.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_21_domain_purity.rs` — direct proof helper for the last warning-bearing family.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_04_loose_files_tests/ownership.rs` — removed the duplicated sibling ownership assertion.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_20_dev_dependency_direction_tests/*` — representative cross-rule sidecars rewritten to use only owned assertions helpers.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_21_domain_purity_tests/*` — the remaining warning-bearing test set that was closed out.

## Next Steps / Continuation Plan
1. Stage only the hexarch files touched in this fix.
2. Commit this checkpoint with the worklog included.
3. If `RS-TEST` tightens again, start with the same pattern: owned assertions first, then sidecar import cleanup, then direct proof helpers.
