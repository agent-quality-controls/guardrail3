# Document Arch And Hexarch READMEs

**Date:** 2026-03-26 21:20
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/README.md`, `apps/guardrail3/crates/app/rs/families/hexarch/README.md`

## Summary
Updated the `arch` family README so its implementation-shape section matches the current self-hosted workspace layout, and added the missing `hexarch` family README describing the routed architecture, workspace structure, and ownership boundaries actually in use.

## Context & Problem
After finishing the `RS-TEST` hardening pass, the next work area is `arch` and `hexarch`. Both families were green under `RS-ARCH` and `RS-TEST`, but their family-local documentation was not in a usable state:
- `arch/README.md` still described the old single-crate `src/` layout
- `hexarch` had no family README at all

That made the next audit step weak, because the implementation and the stated contract were not aligned.

## Decisions Made

### Rewrite only the stale `arch` implementation-shape section
- **Chose:** updated the `Family Implementation Shape` section in `arch/README.md` instead of rewriting the full rule inventory.
- **Why:** the rule descriptions were still broadly valid; the drift was in the self-hosted family shape and shared routing/placement boundary.
- **Alternatives considered:**
  - rewrite the whole README — rejected because it would add churn without improving the immediate contract mismatch
  - leave the stale section until a later audit — rejected because the current layout is already stable enough to document now

### Add a focused `hexarch` family README
- **Chose:** added a concise family README centered on ownership, routing, workspace shape, and self-hosting expectations.
- **Why:** `hexarch` needs a source of truth before the next architecture-tightening pass, especially around `placement`, `FamilyMapper`, `assertions`, `assertions_common`, and `test_support`.
- **Alternatives considered:**
  - copy the full rule inventory into the README — rejected because the immediate need is architectural clarity, not duplicating every rule plan
  - wait until after the next `hexarch` audit — rejected because that audit needs a documented target

### Explicitly document `assertions_common`
- **Chose:** called out `crates/assertions_common` as an assertions-only helper crate in the new `hexarch` README.
- **Why:** it exists in the live workspace and is one of the likely pressure points for the next audit; pretending it is not part of the design would keep the doc inaccurate.
- **Alternatives considered:**
  - omit it and describe only runtime/assertions/test_support — rejected because that would hide a real part of the current architecture

## Architectural Notes
The docs now reflect the current family boundary:
- `arch` is a routed self-hosted workspace with `runtime`, `assertions`, and `test_support`
- `hexarch` is also a routed self-hosted workspace, but with an additional `assertions_common` helper crate that is intentionally narrower than `test_support`

The next audit on these families should now be able to compare implementation to documented structure without tripping over stale README text first.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/arch/README.md`
- `apps/guardrail3/crates/app/rs/families/arch/Cargo.toml`
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/arch/test_support/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/Cargo.toml`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions_common/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/test_support/src/lib.rs`

## Open Questions / Future Considerations
- `hexarch` still needs a deeper architecture audit around whether `assertions_common` and runtime test helpers are the right long-term split.
- `arch` may still need broader README cleanup later if the rule text drifts from implementation, but the highest-value mismatch is fixed now.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — current family contract for `RS-ARCH`.
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — current family contract for `RS-HEXARCH`.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs` — current routed `arch` family entrypoint.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — current routed `hexarch` family entrypoint.
- `.worklogs/2026-03-26-205357-tighten-rs-test-validator-holes.md` — immediate prior checkpoint that closed the `RS-TEST` hardening pass before this docs pass.

## Next Steps / Continuation Plan
1. Run an adversarial audit of `arch` against its updated README and the shared `placement` / `FamilyMapper` plan.
2. Run an adversarial audit of `hexarch`, focusing on family-local discovery boundaries and the legitimacy of `assertions_common`.
3. If those audits expose code drift, fix implementation after the contract is pinned rather than letting the READMEs drift again.
