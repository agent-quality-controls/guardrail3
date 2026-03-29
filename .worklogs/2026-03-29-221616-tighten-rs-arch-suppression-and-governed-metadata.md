# Tighten RS-ARCH Suppression And Governed Metadata

**Date:** 2026-03-29 22:16
**Scope:** `apps/guardrail3/crates/app/rs/placement/src/roots.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots_tests/enablement_matrix.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_08_auxiliary_roots_declared_tests/golden.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/app/rs/families/arch/README.md`, `.plans/todo/checks/rs/arch.md`

## Summary
Closed the last live `arch` tightening gaps from the audit: governed roots can no longer hide behind dead `arch_role = "auxiliary"` metadata, and `RS-ARCH-02` no longer goes silent when misplaced-root reporting is inactive. The current `arch` source-of-truth docs were updated to match those behaviors and now include an explicit `RS-ARCH-08` section.

## Context & Problem
After fixing `RS-ARCH-04`, `RS-ARCH-07`, and the `RS-ARCH-06` coverage gap, two real issues remained:

1. Governed roots under `apps/*` or `packages/*` could still carry `arch_role = "auxiliary"` metadata that did nothing and produced no finding.
2. `RS-ARCH-02` emitted nothing at all when reporting was inactive, making “clean” indistinguishable from “not enforcing.”

These were not speculative architecture ideas anymore. They were concrete tightening opportunities identified in the earlier adversarial review, and they both made the family easier to bypass or misread.

## Decisions Made

### Treat governed-root `arch_role` as a fail-closed input error
- **Chose:** When a governed root declares any `arch_role` metadata, placement records an input failure and `RS-ARCH-07` reports it.
- **Why:** `arch_role = "auxiliary"` is only meaningful outside governed zones. On an app/package root it is dead metadata and should be surfaced as invalid architecture input, not ignored.
- **Alternatives considered:**
  - Extend `RS-ARCH-08` to report governed misuse as an info/warn — rejected because dead metadata on an active governed root is invalid, not a harmless confirmation.
  - Add a brand new rule ID just for governed metadata misuse — rejected because `RS-ARCH-07` already owns malformed/invalid required input integrity.

### Make `RS-ARCH-02` suppression explicit
- **Chose:** When misplaced-root reporting is inactive, `RS-ARCH-02` now emits an inventory/info summary instead of disappearing.
- **Why:** A silent rule makes a clean report ambiguous. The user needs to know whether there were no misplaced roots or whether the rule was simply inactive.
- **Alternatives considered:**
  - Keep full silence — rejected because it weakens trust in the report.
  - Emit an error/warn when inactive — rejected because inactivity itself is not a violation.

### Update current-source docs, not historical debris
- **Chose:** Fix the family README and live rule plan to reflect the new `02`, `07`, and `08` behavior, including explicit source-of-truth wording in the README.
- **Why:** The current docs are what future work should follow. Historical handoffs are still messy, but tightening the live contract first is the highest-signal fix.
- **Alternatives considered:**
  - Try to clean every stale historical file in this slice — rejected because it would sprawl the change and not improve current behavior.

## Architectural Notes
The governed-metadata fix stays inside the shared placement substrate, which is the right place for it:
- placement already parses live manifests for structural architecture purposes
- `FamilyMapper` already routes placement input failures into `RS-ARCH`
- `RS-ARCH-07` already owns fail-closed reporting

That means we tightened the guardrail without adding a second family-local crawl or parser pass.

The `RS-ARCH-02` change is product-surface only: it changes visibility, not enforcement semantics.

## Information Sources
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — live root parsing and metadata handling.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots.rs` — misplaced-root reporting and inventory behavior.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs` — fail-closed coverage.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_08_auxiliary_roots_declared_tests/golden.rs` — non-hit coverage for governed metadata misuse.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — top-level runtime regressions.
- `.worklogs/2026-03-29-220217-align-rs-arch-04-overlap-contract.md`
- `.worklogs/2026-03-29-220707-fix-rs-arch-07-governed-manifest-fail-closed.md`
- `.worklogs/2026-03-29-221125-add-rs-arch-06-app-scope-coverage.md`

## Open Questions / Future Considerations
- Historical `arch` handoff files are still stale and could still mislead cold readers, even though the current README/plan are now better.
- `arch` still locally reconstructs owner-family/effective-enablement facts instead of consuming a richer routed view. The new tests reduce the risk, but the architectural cleanup is still worth doing later.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — structural validation for live manifests and governed metadata misuse.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots.rs` — inactive-reporting visibility contract.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs` — governed metadata fail-closed coverage.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — product-surface regressions for `arch`.
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — current family source of truth.
- `.plans/todo/checks/rs/arch.md` — live rule inventory.

## Next Steps / Continuation Plan
1. If desired, clean or explicitly archive the stale historical `arch` handoff docs so they stop looking current.
2. Consider moving owner-family/effective-enablement facts into shared routed views to reduce family-local policy reconstruction.
3. Re-run a full adversarial `arch` audit after any future placement/mapper refactor, because that is where most of the remaining risk now lives.
