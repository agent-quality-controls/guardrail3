# Harden Rust Garde Family

**Date:** 2026-03-24 15:16
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/garde/**`, `apps/guardrail3/crates/app/rs/validate/{allow_checks.rs,ast_visitors.rs}`, `.plans/todo/checks/rs/garde.md`, `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md`, `.plans/2026-03-24-085025-harden-rs-garde-family.md`

## Summary
Committed the `rs/garde` hardening batch. The family now includes the later field-level, nested-dive, and context-surface rules, has the new rule-directory test shape across the packet, and carries forward a small legacy-validator fix so garde-skip semantics stay coherent until the Rust validation cutover removes the old runtime path.

## Context & Problem
The garde packet in the dirty tree covered several related concerns:
- family-side facts/parse/test support expansion
- new rules `RS-GARDE-AST-05`, `12`, and `13`
- conversion away from old flat `*_tests.rs` sidecars
- plan and handoff updates reflecting the implemented 13-rule family
- a scratch planning note for the garde hardening pass
- a small old-runtime AST/allow-check fix around type-level `#[garde(skip)]`

The last point is important: although the long-term product direction is to cut over to the new family runtime, the old Rust validator path still exists today. Leaving the old garde-skip messaging/AST shape inconsistent while committing the new garde family would preserve an avoidable semantic mismatch.

## Decisions Made

### Keep the new garde family and the legacy garde-skip fix together
- **Chose:** Commit the new family packet together with the small `app/rs/validate` garde-skip adjustments.
- **Why:** Both changes describe the same semantic boundary: what counts as a garde-skip bypass on fields versus types.
- **Alternatives considered:**
  - Leave the old-runtime fix for a later cleanup commit — rejected because it would knowingly preserve inconsistent garde semantics.
  - Put the old-runtime fix in a generic legacy batch — rejected because it is clearly garde-related.

### Treat the family as a 13-rule implemented contract
- **Chose:** Update the garde plan and handoff to speak about the implemented 13-rule family, not the earlier 10-rule shape.
- **Why:** The user has been explicit that plans should describe the target/current contract accurately, not lag behind code once the code exists.
- **Alternatives considered:**
  - Keep the docs conservative and mention the new rules as future additions — rejected because that would immediately make plan-vs-code verification noisy.

### Preserve the scratch garde hardening note
- **Chose:** Commit `.plans/2026-03-24-085025-harden-rs-garde-family.md` with the garde batch.
- **Why:** It is directly about this family and now becomes part of the repo’s planning record instead of floating untracked.
- **Alternatives considered:**
  - Drop it — rejected because the user asked to sort and commit the whole worktree.

## Architectural Notes
- The garde family now owns three more explicit AST-level concerns:
  - field-level validator adequacy
  - nested `#[garde(dive)]`
  - explicit `ctx` wiring
- The family’s source-root behavior is now documented more clearly:
  - workspace roots and standalone package roots are the owned roots
  - workspace members are not independent garde roots
- The old runtime fix is intentionally narrow and should disappear once the Rust validation cutover removes `app/rs/validate/**` from runtime use.

## Information Sources
- `apps/guardrail3/crates/app/rs/checks/rs/garde/**`
- `apps/guardrail3/crates/app/rs/validate/allow_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/ast_visitors.rs`
- `.plans/todo/checks/rs/garde.md`
- `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md`
- `.plans/2026-03-24-085025-harden-rs-garde-family.md`

## Open Questions / Future Considerations
- The runtime cutover still needs to make the new garde family the only public path and retire the old validator path entirely.
- The family is now much stronger, but broad lib-test linking cost remains a practical verification problem until the crate/test topology is split.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/garde/mod.rs` — garde family orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/garde/facts.rs` — owned-root and rule-input normalization
- `apps/guardrail3/crates/app/rs/checks/rs/garde/parse.rs` — source parsing substrate used by the family
- `.plans/todo/checks/rs/garde.md` — authoritative garde-family contract
- `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md` — updated garde handoff
- `apps/guardrail3/crates/app/rs/validate/allow_checks.rs` — temporary old-runtime garde-skip semantics still present before cutover

## Next Steps / Continuation Plan
1. Commit the deps family batch.
2. Commit the hexarch/shared-discovery batch, including `ProjectTree` walker changes and hexarch docs.
3. Finish by checking that the worktree is clean and that any leftovers are intentionally grouped.
