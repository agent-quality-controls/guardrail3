# Finalize Libarch Root Shape

**Date:** 2026-03-24 12:17
**Scope:** `.plans/todo/checks/rs/libarch.md`, `.plans/todo/check_review/test_hardening/20-cargo-agent-brief.md`, `.plans/todo/check_review/test_hardening/21-deps-agent-brief.md`, `.plans/todo/check_review/test_hardening/22-hooks-shared-agent-brief.md`, `.plans/todo/check_review/test_hardening/23-hooks-rs-agent-brief.md`, `.plans/todo/check_review/test_hardening/27-libarch-agent-brief.md`

## Summary
Closed the remaining second-pass handoff gaps found by adversarial review. The main architectural decision was to freeze `libarch` layered mode as `workspace root + root facade package`, removing the remaining root-shape ambiguity from the plan and the implementation brief.

## Context & Problem
After committing the first handoff-repair batch, a second adversarial pass still found concrete issues:
- `cargo` brief misclassified `g3rs-cargo/disallowed-macros-deny` and `RS-CARGO-14`
- `deps` brief dropped the mixed-scope ownership split and failed to warn that target-specific tables remain out of scope
- split hook briefs still lacked generator/template sources and enough routing/parity context
- `libarch` still had a real contract contradiction:
  - main rule intent required the layered root to be both workspace root and facade package
  - open questions still allowed a pure workspace root

That last ambiguity was especially dangerous because a fresh implementation agent could legitimately choose either shape and still think it was following the plan.

## Decisions Made

### Freeze `libarch` layered mode to workspace root + facade package
- **Chose:** require the layered library root to remain both the workspace root and the root facade package.
- **Why:** this is the more enforceable architecture. It gives `RS-LIBARCH-11` a concrete root export surface and avoids inventing two incompatible layered shapes for one family.
- **Alternatives considered:**
  - Allow either facade-package root or pure workspace root — rejected because the family would then need two separate root-shape semantics and the current rule inventory is not written for that.
  - Make the root a pure workspace only — rejected because it weakens the package-level facade contract and makes root export ownership less direct.

### Keep handoff briefs aligned to current plan scope, not future semantics
- **Chose:** patch `cargo`, `deps`, and hook split briefs to reflect the current family contracts exactly, including explicit out-of-scope areas.
- **Why:** the handoff packet is supposed to be droppable into a fresh agent. Overstating current scope or leaving active limitations implicit causes agents to invent semantics.
- **Alternatives considered:**
  - Leave “close enough” brief wording in place — rejected because the second-pass reviewers found real concrete ways it would misdirect implementation.

### Preserve lane-level hook context inside split briefs
- **Chose:** add generator/template sources and current routing/generation state to the split hook briefs instead of forcing agents to rediscover it from the combined lane brief.
- **Why:** the split briefs are meant to be standalone. If generator parity and TS/non-Rust cleanup remain live lane debt, the split briefs must say what is already fixed and where the remaining sources of truth live.
- **Alternatives considered:**
  - Keep the split briefs strictly family-local and rely on `15-hooks-agent-brief.md` for lane context — rejected because that makes the split briefs not truly droppable.

## Architectural Notes
The important planning rule reinforced here is:
- if a family contract has an unresolved shape decision, it is not enough to “mention the tension” in the brief
- either freeze the contract or explicitly block implementation on that decision

For `libarch`, the right move was to freeze it. The family is about enforceable crate boundaries, and allowing two root shapes would weaken that right where the architecture is supposed to be strongest.

## Information Sources
- `.plans/todo/checks/rs/libarch.md`
- `.plans/todo/check_review/test_hardening/27-libarch-agent-brief.md`
- `.plans/todo/check_review/test_hardening/20-cargo-agent-brief.md`
- `.plans/todo/check_review/test_hardening/21-deps-agent-brief.md`
- `.plans/todo/check_review/test_hardening/22-hooks-shared-agent-brief.md`
- `.plans/todo/check_review/test_hardening/23-hooks-rs-agent-brief.md`
- adversarial second-pass reviewer outputs from this session on cargo/deps/hooks/libarch brief correctness
- `.worklogs/2026-03-24-121223-repair-rust-agent-briefs.md`

## Open Questions / Future Considerations
- `libarch` still has other implementation questions, but the root shape is no longer one of them.
- The hook lane still spans both split family work and broader routing/parity cleanup, so the combined hook brief remains useful even after the split briefs are repaired.
- `cargo` still needs implementation reconciliation, but its brief is now aligned to the current plan contract again.

## Key Files for Context

- `.plans/todo/checks/rs/libarch.md` — source of truth for the frozen layered-library contract.
- `.plans/todo/check_review/test_hardening/27-libarch-agent-brief.md` — droppable implementation/hardening packet for `libarch`.
- `.plans/todo/check_review/test_hardening/20-cargo-agent-brief.md` — corrected cargo handoff, especially rule applicability and input-failure ownership.
- `.plans/todo/check_review/test_hardening/21-deps-agent-brief.md` — corrected deps handoff with explicit ownership split and target-table limitation.
- `.plans/todo/check_review/test_hardening/22-hooks-shared-agent-brief.md` — split shared-hook packet with generator/routing context restored.
- `.plans/todo/check_review/test_hardening/23-hooks-rs-agent-brief.md` — split Rust-hook packet with legacy tool-check sources and grouped-test debt called out.
- `.worklogs/2026-03-24-121223-repair-rust-agent-briefs.md` — prior batch that repaired the broader handoff set.

## Next Steps / Continuation Plan

1. Commit this second-pass repair batch and the `libarch` plan finalization without pulling in unrelated worktree changes.
2. If more adversarial review is desired, run a short third pass only against the still-edited handoff briefs and `libarch.md` to confirm there are no remaining silent contradictions.
3. After that, stop planning work here and return to execution lanes unless a new family-level contract gap is discovered.
