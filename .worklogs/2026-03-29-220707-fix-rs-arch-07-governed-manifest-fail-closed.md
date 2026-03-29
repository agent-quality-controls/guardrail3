# Fix RS-ARCH-07 Governed Manifest Fail Closed

**Date:** 2026-03-29 22:07
**Scope:** `apps/guardrail3/crates/app/rs/placement/src/roots.rs`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/app/rs/families/arch/README.md`, `.plans/todo/checks/rs/arch.md`

## Summary
Fixed the main `RS-ARCH-07` enforcement gap: malformed governed app/package manifests now fail closed instead of silently passing. The change lives in shared placement so `arch` still consumes routed placement input rather than becoming its own parser/crawler.

## Context & Problem
Adversarial review found that `RS-ARCH-07` only surfaced unreadable-present `Cargo.toml` files and malformed auxiliary-role metadata. Governed roots inside `apps/` and `packages/` were not parsed at all during placement because `arch_role` is only relevant outside governed zones. That meant malformed governed manifests were still classified structurally and flowed through `arch` as if nothing was wrong.

This was a real contract violation. The current `arch` README and rule plan both say required active inputs must fail closed, and the user explicitly asked to fix that gap after the audit. The existing family test was even codifying the bad behavior by asserting that malformed app-owned manifests should not emit `RS-ARCH-07`.

## Decisions Made

### Parse every eligible live root once in placement
- **Chose:** Parse all eligible live `Cargo.toml` files in placement, regardless of whether the root is governed or auxiliary/other.
- **Why:** `RS-ARCH-07` depends on placement input failures routed through `FamilyMapper`. If only out-of-zone roots are parsed, governed parse failures can never become fail-closed findings.
- **Alternatives considered:**
  - Add a second parse pass inside the `arch` family — rejected because it duplicates discovery/parsing and weakens the shared placement contract.
  - Restrict fail-closed behavior to unreadable files only — rejected because it keeps the spec violation and leaves a bypassable malformed-manifest hole.

### Keep `arch_role` extraction scoped to out-of-zone roots
- **Chose:** Split parsing from `arch_role` extraction. Governed roots are now parsed for syntax validity, but `arch_role` is still only interpreted for non-governed roots.
- **Why:** This closes the fail-closed hole without changing the architecture meaning of governed roots or inventing new metadata semantics there.
- **Alternatives considered:**
  - Start interpreting `arch_role` on governed roots too — rejected because that is a separate tightening question and would conflate this bug fix with new policy.

### Add runtime-level proof, not just family-local proof
- **Chose:** Add a top-level `arch` runtime regression for malformed governed manifests in `runtime_tests.rs`.
- **Why:** The earlier audit already showed that family-local tests can miss product-surface gaps. This fix needed proof that `guardrail3 rs validate --family arch` surfaces `RS-ARCH-07`, not just that the family crate does.
- **Alternatives considered:**
  - Rely only on the family test suite — rejected because that was exactly how the previous gap persisted.

## Architectural Notes
The fix keeps the intended substrate clean:
- `placement` still owns eligible live root discovery and input integrity.
- `FamilyMapper` still forwards placement failures into `RS-ARCH`.
- `RS-ARCH-07` remains a pure fail-closed reporting rule over routed failure facts.

The only architectural broadening is that placement now syntactically parses governed manifests too. That is still structural input integrity, not family policy.

## Information Sources
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — source of the original gap and the fix.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs` — family-local fail-closed expectations.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — top-level product-surface regression coverage.
- `apps/guardrail3/crates/app/rs/families/arch/README.md` and `.plans/todo/checks/rs/arch.md` — contract wording for required-input fail-closed behavior.
- The earlier arch audit in this session identifying malformed governed manifests as the highest-severity live bug.

## Open Questions / Future Considerations
- Governed roots can still carry dead `arch_role = "auxiliary"` metadata with no finding. That is a separate tightening opportunity.
- `RS-ARCH-06` still needs adversarial coverage for app-scoped `hexarch` overrides and nested owner resolution.
- The broader `arch` doc stack still has stale historical files; this work only updated the current README/plan wording.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — shared live-root parsing and fail-closed input failure collection.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs` — consumes placement failures into `RS-ARCH` facts.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs` — unit-level governed/auxiliary/config fail-closed coverage.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — top-level `arch` runtime regressions.
- `.worklogs/2026-03-29-220217-align-rs-arch-04-overlap-contract.md` — immediately prior `arch` contract work that fixed `RS-ARCH-04`.

## Next Steps / Continuation Plan
1. Add adversarial `RS-ARCH-06` tests for app-scoped `hexarch` overrides and nested app-owner path derivation in `rs_arch_06_owner_family_enablement_coherence_tests/`.
2. Decide how to surface dead `arch_role = "auxiliary"` metadata on governed roots and implement that either as a new `RS-ARCH-08` extension or a new rule.
3. Clean the remaining stale `arch` docs and historical handoffs so the current README/plan are the obvious source of truth.
