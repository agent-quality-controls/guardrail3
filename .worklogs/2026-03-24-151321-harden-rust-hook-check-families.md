# Harden Rust Hook Check Families

**Date:** 2026-03-24 15:13
**Scope:** `apps/guardrail3/crates/app/rs/checks/hooks/**`, `.plans/todo/check_review/test_hardening/05-hooks.md`, `.plans/todo/check_review/test_hardening/15-hooks-agent-brief.md`, `.plans/todo/generate/rs/hooks.md`

## Summary
Committed the Rust/shared hook-family hardening pass as one batch. This includes deeper shell parsing and executable-line matching logic, broader rule-side adversarial coverage across `HOOK-SHARED` and `HOOK-RS`, and updated hook hardening docs/briefs that reflect the current state of the lane.

## Context & Problem
The dirty tree contained a very large hook-family packet touching:
- the shared shell parsing substrate
- almost every `HOOK-RS-*` rule
- several `HOOK-SHARED-*` rules
- test suites that were expanded to catch echoed-command false passes, dispatcher lookalikes, and compact shell forms
- handoff and generator docs for the hook lane

These files form one semantic unit. Splitting them apart would make it harder to understand why the shell parser changed and which hook false positives or false negatives the rule/test changes were closing.

## Decisions Made

### Keep shared and Rust hook families together in one code commit
- **Chose:** Commit `hooks/shared`, `hooks/rs`, and the shared `shell.rs` substrate together.
- **Why:** The Rust hook family depends on the shared shell parsing/executable-command semantics, and several fixes span both levels.
- **Alternatives considered:**
  - Split `HOOK-SHARED` and `HOOK-RS` into separate commits — rejected because the shell/parser changes are shared and the resulting history would be misleading.
  - Put shell parser changes in a standalone substrate commit — rejected because they would be contextless without the rule/test expansions that motivated them.

### Treat the batch as semantic hardening, not just test churn
- **Chose:** Describe the commit around the semantic false-pass closures, not only “more tests”.
- **Why:** Several rules changed behavior materially:
  - echoed tool strings no longer satisfy executable checks
  - dispatcher lookalikes no longer satisfy hook structure checks
  - compact shell forms and nested branching are handled more accurately
- **Alternatives considered:**
  - Frame it as only a sidecar-test migration — rejected because that would understate the actual rule behavior changes.

### Keep the generator hook spec with the hook batch
- **Chose:** Include `.plans/todo/generate/rs/hooks.md`.
- **Why:** The updated generator contract now spells out which modular hook scripts are always generated and which are conditional. That is directly relevant to the hook rule semantics.
- **Alternatives considered:**
  - Leave the generator note for a later docs commit — rejected because it now documents behavior this batch is implicitly testing against.

## Architectural Notes
- `hooks/shell.rs` is acting as a small parsing/normalization substrate for both hook families.
- The hardening trend in this batch is to move away from raw substring detection and toward:
  - executable-line context
  - shell-structure awareness
  - branch-local trigger/validation matching
- The docs now treat the lane as largely hardened at the rule/test level, with routing/generator parity work left as the next step.

## Information Sources
- `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs`
- `apps/guardrail3/crates/app/rs/checks/hooks/shared/**`
- `apps/guardrail3/crates/app/rs/checks/hooks/rs/**`
- `.plans/todo/check_review/test_hardening/05-hooks.md`
- `.plans/todo/check_review/test_hardening/15-hooks-agent-brief.md`
- `.plans/todo/generate/rs/hooks.md`

## Open Questions / Future Considerations
- The hook families still have follow-on routing/generator parity work outside this batch.
- The current tree still contains a combined hooks brief plus split hook briefs; that is acceptable for now but may be worth pruning later if the combined brief stops being useful.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs` — shared shell/executable-line semantics
- `apps/guardrail3/crates/app/rs/checks/hooks/shared/` — shared hook-structure rules
- `apps/guardrail3/crates/app/rs/checks/hooks/rs/` — Rust hook semantic rules
- `.plans/todo/check_review/test_hardening/05-hooks.md` — current lane status and remaining work
- `.plans/todo/check_review/test_hardening/15-hooks-agent-brief.md` — updated droppable hook handoff
- `.plans/todo/generate/rs/hooks.md` — hook generator contract

## Next Steps / Continuation Plan
1. Commit the cargo family reconciliation as its own batch, including the multi-root policy-root discovery changes and the updated cargo brief/plan.
2. Commit the clippy/deny family packet with the canonical ban expansion and parity coverage.
3. Continue through the remaining family packets until the worktree is clean.
