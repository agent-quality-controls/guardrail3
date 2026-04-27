# Freeze Additional Rust Rule Candidates

**Date:** 2026-03-23 20:10
**Scope:** `.plans/todo/check_review/06-new-rust-rule-candidates.md`, `.plans/todo/checks/rs/code.md`, `.plans/todo/checks/rs/deps.md`

## Summary
This pass converted two more Rust rule candidates from vague ideas into concrete planned hard rules and tightened the candidate classification around them. The result is a cleaner split between crisp universal additions and the still-ambiguous policy-heavy ideas.

## Context & Problem
After freezing `RS-CODE-31` and the narrow public-error rule, there were still open candidate discussions around generic parameter count and dependency pressure. The user clarified the product philosophy: internal code is not exempt from quality bars, and thresholds should be set high enough that crossing them is almost certainly bad code rather than a style disagreement. That made it possible to define these as hard guardrails instead of leaving them as fuzzy “maybe later” heuristics.

## Decisions Made

### Turn generic parameter count into a hard universal rule
- **Chose:** Plan `RS-CODE-34` as an `Error` when a `struct`, `enum`, `trait`, or `fn` has more than 6 type/const generic parameters.
- **Why:** At that threshold, the pattern is much more likely to be agentic abstraction sludge than legitimate everyday Rust design.
- **Alternatives considered:**
  - Count lifetimes too — rejected because lifetimes inflate noise and would push the rule toward false positives.
  - Apply to `impl` blocks directly — rejected because the generic burden is already more meaningfully represented on the item definitions themselves.

### Turn direct dependency sprawl into a hard universal rule
- **Chose:** Plan `g3rs-deps/direct-dependency-cap` as an `Error` when one crate has more than 25 unique direct dependency names across normal/build/dev/target direct dependency tables.
- **Why:** A high direct-dependency cap catches real sprawl without needing crate-role exceptions, and it aligns with the “shitty code is shitty code whether publishable or internal” principle.
- **Alternatives considered:**
  - Count only `[dependencies]` — rejected because build/dev/target direct dependencies still contribute to complexity and abuse.
  - Include transitive-depth pressure now — rejected because that needs more policy and graph interpretation than the current hard universal rule should own.

### Remove the `#[must_use]` function rule from the immediate frozen set
- **Chose:** Keep the previously discussed `#[must_use]` rule out of the immediate concrete planned set.
- **Why:** With no bypass mechanism, the bar for direct source rules is very high. The other rules were cleaner and more defensible under the product philosophy.
- **Alternatives considered:**
  - Keep it as a warning-level planned rule — rejected because warning-only lint noise is not the desired model.

## Architectural Notes
These new planned rules continue the current split:
- source-shape hard rules belong in `rs/code`
- crate dependency-surface hard rules belong in `rs/deps`

They also reinforce the working design principle for new direct rules:
- no special “internal crate” exemptions
- high thresholds
- only adopt rules that are crisp enough to enforce without bypasses

## Information Sources
- `.plans/todo/check_review/06-new-rust-rule-candidates.md` — candidate inventory and classification
- `.plans/todo/checks/rs/code.md` — live `rs/code` family contract
- `.plans/todo/checks/rs/deps.md` — live `rs/deps` family contract
- user discussion in this session clarifying:
  - no publishable/internal quality split
  - thresholds should catch obviously bad code, not style differences
  - warnings are not the preferred model for actionable rules

## Open Questions / Future Considerations
- `string-based dispatch warning`, `pub(crate)` discipline, dependency-type leakage, and structural-organization ideas remain unresolved and should stay in the candidate bucket until they can be made equally crisp.
- `RS-CODE-33` still needs implementation design: whether it replaces `RS-CODE-25` outright or subsumes it during migration.
- The repo has a large unrelated hardening worktree in progress; this commit intentionally excludes those changes.

## Key Files for Context
- `.plans/todo/check_review/06-new-rust-rule-candidates.md` — current candidate classification and ownership
- `.plans/todo/checks/rs/code.md` — planned `RS-CODE-31`, `33`, and `34`
- `.plans/todo/checks/rs/deps.md` — planned `g3rs-deps/direct-dependency-cap`
- `.worklogs/2026-03-23-160918-add-family-hardening-agent-briefs.md` — prior planning work that set up the parallel hardening structure

## Next Steps / Continuation Plan
1. Continue the candidate review only for items that can be made crisp enough for no-bypass hard enforcement.
2. When implementation time comes, treat `RS-CODE-33`, `RS-CODE-34`, and `g3rs-deps/direct-dependency-cap` as independent rule additions with full family-local tests.
3. Keep unrelated hardening-lane work out of planning-only commits unless explicitly integrating those lanes.
