# Add Toolchain And Fmt Stabilization Handoffs

**Date:** 2026-03-27 19:25
**Scope:** `.plans/todo/family-stabilization-handoffs/toolchain.md`, `.plans/todo/family-stabilization-handoffs/fmt.md`

## Summary
Added two cold-start handoff briefs for `RS-TOOLCHAIN` and `RS-FMT`. Each brief explains the current Rust family architecture, the self-hosting stabilization goal, the attack-review mindset, and the exact verification commands and end-state expected from a worker.

## Context & Problem
The next wave of family work is better delegated on the smaller and structurally simpler families first. `toolchain` and `fmt` are both small, already have passing unit suites, and have relatively low `RS-TEST` debt, which makes them safe worker tasks. What was missing was precise handoff context: a worker without the recent `arch`/`test`/`cargo`/`hexarch`/`code` history would not know that the goal is to fix the family and its detector, not to reduce unrelated repo findings.

## Decisions Made

### Use dedicated stabilization handoff files
- **Chose:** Write one standalone handoff file per easy family under `.plans/todo/family-stabilization-handoffs/`.
- **Why:** That keeps the worker ask explicit, linkable, and separate from the rule-inventory plans.
- **Alternatives considered:**
  - Put the notes into a single shared meta-plan — rejected because workers need one direct file per assignment.
  - Reuse old `rs-test-compliance-handoffs` format without new context — rejected because these new handoffs need more architecture and attack-review guidance.

### Emphasize “fix rules, not repo debt”
- **Chose:** Make the handoffs explicit that the task is family stabilization plus adversarial rule review, not repo cleanup.
- **Why:** Recent `RS-CODE` work showed the difference matters; lower finding counts are not evidence of correctness.
- **Alternatives considered:**
  - Phrase the work as “make the family pass” only — rejected because that invites cargo-cult code movement without rule validation.

### Include shared Rust architecture in the handoffs
- **Chose:** Summarize `placement -> family_selection -> FamilyMapper -> family runtime -> rule inputs` in both files.
- **Why:** A cold-start worker could otherwise reintroduce family-local root discovery or bypass the routed-family architecture.
- **Alternatives considered:**
  - Just link `apps/guardrail3/crates/app/rs/README.md` — rejected because the worker needs the critical constraint visible inside the handoff itself.

## Architectural Notes
These handoffs assume the stabilized-family pattern already established by `test`, `arch`, `cargo`, `hexarch`, and `code`:

- family workspace root
- `crates/runtime`
- `crates/assertions`
- `test_support`
- external routing via shared Rust placement and `FamilyMapper`

The handoffs intentionally push the worker to preserve that boundary and to treat the rule family as the thing under repair.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `.plans/todo/checks/rs/toolchain.md`
- `.plans/todo/checks/rs/fmt.md`
- `.plans/todo/rs-test-compliance-handoffs/arch.md`
- live validation snapshots gathered from `rs validate ... --family arch/test/self`

## Open Questions / Future Considerations
- `deps` and `garde` are also likely handoff-safe, but they are larger and were not documented in this commit.
- `clippy`, `deny`, and especially `release` remain poorer cold-start worker tasks until they are framed more tightly.

## Key Files for Context
- `.plans/todo/family-stabilization-handoffs/toolchain.md` — worker brief for `RS-TOOLCHAIN`
- `.plans/todo/family-stabilization-handoffs/fmt.md` — worker brief for `RS-FMT`
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust architecture contract
- `apps/guardrail3/crates/app/rs/families/test/README.md` — stabilized self-hosted family specimen
- `.worklogs/2026-03-27-183815-merge-rs-code-28-into-27.md` — nearby context on current rule-hardening lane

## Next Steps / Continuation Plan
1. Commit these two handoff files and keep the repo clean except for untracked build output.
2. Continue the `RS-CODE` adversarial pass locally, focusing on untouched rule surfaces and concrete false positives/false negatives rather than repo cleanup.
3. Once `RS-CODE` looks stable enough, start `RS-CLIPPY` locally with the same sequence: README, structural stabilization, self-hosting, then attack-review.
