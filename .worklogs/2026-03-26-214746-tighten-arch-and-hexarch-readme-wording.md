# Tighten Arch And Hexarch README Wording

**Date:** 2026-03-26 21:47
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/README.md`, `apps/guardrail3/crates/app/rs/families/hexarch/README.md`

## Summary
Made two small but important README corrections after a follow-up review: `arch` now points to the shared Rust scope plan instead of restating that shared substrate as if it were family-local source of truth, and `hexarch` now describes `assertions_common` as current shape under audit rather than settled final design.

## Context & Problem
After documenting the two families, a quick review found that the docs were close but not quite right:
- `arch/README.md` still embedded too much of the broader shared placement plan, which belongs in `apps/guardrail3/crates/app/rs/README.md`
- `hexarch/README.md` sounded too confident about `assertions_common`, even though validating that crate is one of the next audit tasks

These are small wording issues, but they matter because the next work is architecture review, and the READMEs should not overclaim stability where the code is still under scrutiny.

## Decisions Made

### Make `arch` point to the shared Rust scope source of truth
- **Chose:** replaced the subtree-shaped “target shared shape” block with a direct pointer to the shared Rust scope README plus a short family-specific summary.
- **Why:** the family README should state how `arch` consumes shared scope, not duplicate the broader platform plan.
- **Alternatives considered:**
  - leave the duplicated shared-shape block — rejected because it will drift
  - move all shared-placement discussion out of the family README — rejected because the family still needs to state that it depends on external shared scope

### Mark `assertions_common` as a current implementation shape
- **Chose:** kept `assertions_common` documented, but changed the wording to make clear it is a current compromise that still needs architectural validation.
- **Why:** the crate is real and must be acknowledged, but it should not be presented as unquestioned final design.
- **Alternatives considered:**
  - remove `assertions_common` from the README — rejected because that would hide real live structure
  - keep the stronger wording — rejected because it overstates certainty

## Architectural Notes
These edits do not change code or rule behavior. They narrow the docs so that:
- the shared Rust scope plan stays centralized in `apps/guardrail3/crates/app/rs/README.md`
- family READMEs describe family-specific boundaries and current implementation state

That separation is important before the next adversarial architecture pass on `arch` and `hexarch`.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md`
- `apps/guardrail3/crates/app/rs/families/arch/README.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md`
- `.worklogs/2026-03-26-212058-document-arch-and-hexarch-readmes.md`

## Open Questions / Future Considerations
- `hexarch` still needs the actual architecture audit around whether `assertions_common` is justified.
- `arch` may still need more doc cleanup later if the rule text drifts from implementation, but the immediate source-of-truth problem is fixed.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust scope and family-mapper source of truth.
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — current `arch` family contract.
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — current `hexarch` family contract.
- `.worklogs/2026-03-26-212058-document-arch-and-hexarch-readmes.md` — prior docs checkpoint this wording fix refines.

## Next Steps / Continuation Plan
1. Run the adversarial `arch` audit against the updated family README and the shared Rust scope plan.
2. Run the adversarial `hexarch` audit, focusing first on `assertions_common` and runtime test-only helpers.
3. If those audits expose implementation drift, fix code after the contract review rather than expanding the READMEs again first.
