# Fix Arch Runtime Fail-Closed

**Date:** 2026-03-26 22:03
**Scope:** `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/README.md`

## Summary
Fixed the main end-to-end `arch` bug found in the adversarial audit: `guardrail3 rs validate --family arch` no longer aborts on malformed `guardrail3.toml`, and instead reports `RS-ARCH-07` fail-closed findings alongside any placement findings. I also removed `RS-ARCH-02` misplaced-root reporting policy from `FamilyMapper` so that `arch` owns that rule policy itself instead of splitting it across the mapper and the family.

## Context & Problem
The `RS-ARCH` family tests were green, but the actual product runtime contradicted the family contract:
- `runtime.rs` parsed `guardrail3.toml` up front and aborted the whole run on parse failure
- `arch` tests expected malformed config to fail closed as findings, not as a top-level runtime error

The adversarial review also found a smaller but real architecture drift:
- `FamilyMapper` still computed `RsArchRoute.reporting_enabled`
- `arch` itself reparsed config to decide owner-family coherence

That meant the family README and the shared Rust scope plan were both overstating the decoupling. `arch` rule policy was still partly living in the mapper.

## Decisions Made

### Let arch-only runs continue without parsed config
- **Chose:** in `runtime.rs`, if config parsing fails and the requested family set is exactly `[Arch]`, continue with `config = None` instead of aborting.
- **Why:** this preserves the current product behavior for broad runs while fixing the specific end-to-end `arch` fail-closed contract that the family already tests for.
- **Alternatives considered:**
  - make all Rust runs continue on malformed config — rejected for this checkpoint because broader runtime behavior is still covered by existing integration expectations
  - leave the abort in place and weaken `arch` tests — rejected because the family contract is right and the product path was wrong

### Move misplaced-root reporting policy into the arch family
- **Chose:** removed `reporting_enabled` from `RsArchRoute` and moved `RS-ARCH-02` reporting enablement computation into `arch` facts/config resolution.
- **Why:** whether misplaced roots should report is `arch` rule policy, not shared route-mapping policy.
- **Alternatives considered:**
  - keep `reporting_enabled` in the mapper — rejected because it keeps `arch` policy split across two layers
  - move all `arch` config policy into family selection immediately — rejected because that is a broader architecture refactor than this bug fix

### Add runtime-level coverage where the bug actually lived
- **Chose:** added a regression in `runtime_tests.rs` asserting that malformed `guardrail3.toml` still produces `RS-ARCH-02` and `RS-ARCH-07` during an `arch`-only run.
- **Why:** family-local tests were insufficient because they bypassed the top-level runtime parse path.
- **Alternatives considered:**
  - add a workspace-root CLI integration test — rejected because that test location is not a reliable cargo target in this workspace shape

## Architectural Notes
This checkpoint narrows, but does not completely eliminate, the policy split in the Rust runtime stack.

After this change:
- `FamilyMapper` maps shared root scope into `RsArchRoute` data only
- `arch` computes its own misplaced-root reporting enablement from config
- `runtime.rs` still owns a special-case decision for malformed config on `arch`-only runs

That last point is the remaining compromise. It fixes the user-visible bug while avoiding a larger runtime behavior change for all families at once.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/arch/README.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `apps/guardrail3/crates/app/rs/runtime.rs`
- `apps/guardrail3/crates/app/rs/runtime_tests.rs`
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs`
- `.worklogs/2026-03-26-205357-tighten-rs-test-validator-holes.md`
- `.worklogs/2026-03-26-214746-tighten-arch-and-hexarch-readme-wording.md`

## Open Questions / Future Considerations
- The broader Rust runtime still aborts on malformed config for non-`arch` runs. That may be correct or may need a fuller fail-closed redesign later.
- `FamilyMapper` still owns per-family/root applicability for other families. This checkpoint only removed the `arch`-specific misplaced-root reporting policy from it.
- The next `arch` architecture pass should revisit whether more config/applicability work should move out of `runtime.rs` and `family_mapper`.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/runtime.rs` — top-level Rust runtime dispatch and config handling.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — runtime-level regression coverage for `arch`.
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — shared route mapping, now without `arch` misplaced-root policy.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs` — `arch` config resolution and misplaced-root reporting enablement.
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust scope plan and route-shape contract.
- `.worklogs/2026-03-26-212058-document-arch-and-hexarch-readmes.md` — the doc checkpoint immediately before this implementation fix.

## Next Steps / Continuation Plan
1. Commit this runtime/mapper fix and keep the repo clean.
2. Return to the broader `arch` audit and decide whether the remaining runtime special-case should become a more general fail-closed runtime policy.
3. After `arch`, run the same architecture audit on `hexarch`, especially around local discovery and `assertions_common`.
