# Add Family Hardening Agent Briefs

**Date:** 2026-03-23 16:09
**Scope:** `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`, `.plans/todo/check_review/test_hardening/`

## Summary
This change turned the test-hardening idea into an actionable handoff system for parallel family work. It tightened the architecture contract around rule-specific test module directories and added shared hardening docs plus droppable per-family agent briefs for `hexarch`, `code`, `release`, `clippy+deny`, and `hooks`.

## Context & Problem
We had reached a point where the main blocker was no longer family implementation breadth, but the lack of a clean way to parallelize hardening across multiple terminals. A generic “playbook” was not sufficient: a fresh agent still needed the exact docs, code roots, old test corpus, known gaps, required attack classes, and done criteria for one family. The user explicitly wanted something droppable into a new agent so it could do a strong family-local hardening pass without additional coaching.

## Decisions Made

### Make the test architecture uniform across all rules
- **Chose:** Update the architecture doc so every rule uses a rule-specific `*_tests/` module directory, with tests split by attack class.
- **Why:** Leaving a “small rules can keep single sidecar files” exception would create another loophole and inconsistent quality bar.
- **Alternatives considered:**
  - Keep `*_tests.rs` for small rules — rejected because the user explicitly wanted the same treatment for every rule.
  - Leave the structure unspecified and only define behavior — rejected because parallel agents need a concrete file-layout contract.

### Define a shared attack-model contract
- **Chose:** Add a shared test-story file that states one test equals one attack vector, and each attack vector must be applied everywhere it should matter in the golden fixture.
- **Why:** The previous wording still left room for narrow “one mutation, one assertion” tests. The new wording captures the intended broad mutation strategy.
- **Alternatives considered:**
  - Keep the guidance embedded only in the architecture doc — rejected because the hardening campaign needs its own explicit execution contract.

### Produce per-family agent briefs instead of one generic handoff
- **Chose:** Create five family-specific brief files with the exact reading list, code roots, old corpus paths, known gaps, required attack classes, and done criteria.
- **Why:** A new agent cannot do a great job from a generic playbook alone; it also needs family-local context and targets.
- **Alternatives considered:**
  - Rely on the generic playbook plus ad hoc instructions per session — rejected because it does not scale to parallel terminals and is easy to underspecify.
  - Put everything in one giant universal brief — rejected because family-specific context would get diluted and harder to hand off cleanly.

## Architectural Notes
The updated structure for the hardening phase is:
- global architecture contract in the checker architecture doc
- universal attack philosophy in `00-shared-test-story.md`
- universal agent workflow in `99-family-agent-playbook.md`
- per-family droppable briefs in `11..15-*-agent-brief.md`

This creates a clean handoff hierarchy:
1. universal constraints
2. family lane strategy
3. family-local execution brief

## Information Sources
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — active structural source of truth
- `.plans/todo/check_review/README.md` — grouped Rust hardening backlog
- `.plans/todo/check_review/test_hardening/01-hexarch.md`
- `.plans/todo/check_review/test_hardening/02-code.md`
- `.plans/todo/check_review/test_hardening/03-release.md`
- `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md`
- `.plans/todo/check_review/test_hardening/05-hooks.md`
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` — prior extracted gap inventory
- `.worklogs/2026-03-23-153943-archive-check-review-and-hexarch-macros.md` — prior backlog reorganization context

## Open Questions / Future Considerations
- The repo code still uses `*_tests.rs` today; these docs define the target shape for the hardening pass rather than reflecting already-completed restructuring.
- If additional families are split later, they should get their own agent briefs instead of overloading the existing ones.
- The untracked frontend-plan files in `.plans/todo/checks/rs/` remain unrelated and were intentionally left out of this commit.

## Key Files for Context
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — updated structural contract for rule tests
- `.plans/todo/check_review/test_hardening/README.md` — index for the hardening packet set
- `.plans/todo/check_review/test_hardening/00-shared-test-story.md` — universal test attack model
- `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md` — agent workflow contract
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md` — family packet for hexarch
- `.plans/todo/check_review/test_hardening/12-code-agent-brief.md` — family packet for code
- `.plans/todo/check_review/test_hardening/13-release-agent-brief.md` — family packet for release
- `.plans/todo/check_review/test_hardening/14-clippy-deny-agent-brief.md` — family packet for clippy/deny
- `.plans/todo/check_review/test_hardening/15-hooks-agent-brief.md` — family packet for hooks

## Next Steps / Continuation Plan
1. Start parallel hardening using the new briefs, beginning with `11-hexarch-agent-brief.md` and `12-code-agent-brief.md`.
2. As each family agent completes, update the corresponding lane file (`01..05`) with closed gaps and remaining semantic risks.
3. Convert the actual codebase family tests from `*_tests.rs` to `*_tests/` directories as the hardening passes proceed, not in a separate blind structural sweep.
