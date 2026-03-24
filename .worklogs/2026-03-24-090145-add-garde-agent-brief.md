# Add Garde Agent Brief

**Date:** 2026-03-24 09:01
**Scope:** `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md`

## Summary
Added a dedicated droppable handoff file for the `rs/garde` family. The brief packages the current family contract, required reading order, known gaps, and exact hardening/done criteria so a fresh agent can take over the family without needing custom chat context.

## Context & Problem
The user wanted a "full garde handoff plan" suitable for starting a new agent. The repo already had shared hardening guidance and some per-family brief files for other families, but `garde` did not yet have its own agent brief. Without a dedicated file, a new agent would need a manually assembled packet from chat and might miss the multi-root model, conditional gating semantics, or the known live carry-forward gaps from the archived garde design work.

## Decisions Made

### Add a dedicated garde agent brief next to the other test-hardening handoffs
- **Chose:** Create `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md`.
- **Why:** The project already uses per-family brief files as droppable handoffs. Putting `garde` in the same location keeps the handoff system consistent and makes it easy to pass a single file path to a new agent.
- **Alternatives considered:**
  - Reply with an inline chat-only handoff — rejected because the user explicitly wanted it "into the file".
  - Extend the generic playbook only — rejected because `garde` has family-specific constraints that are too important to leave implicit.

### Encode the actual garde contract, not just generic hardening instructions
- **Chose:** Include root ownership, conditional garde gating, per-root `clippy.toml` coverage, fail-closed behavior through `RS-GARDE-10`, and the currently known live architectural gaps.
- **Why:** `garde` is more conditional than many other families, and a generic "rewrite tests aggressively" handoff would not be enough. The new agent needs to know what the family really governs and what is intentionally still missing.
- **Alternatives considered:**
  - Keep the brief short and point only at `garde.md` — rejected because the whole purpose was to give a fresh agent a high-quality packet without requiring guesswork.

### Make the brief action-oriented and aligned with the stronger test model
- **Chose:** The brief requires conversion from old `*_tests.rs` sidecars to rule-specific `*_tests/` directories and frames the mission around attack-vector testing, exact hit/non-hit assertions, and semantic bug finding.
- **Why:** The repo’s current hardening direction has already moved to per-rule directory sidecars and broad mutation attack tests. The handoff file needed to reflect that stronger contract explicitly.
- **Alternatives considered:**
  - Describe only architectural reading and leave testing style vague — rejected because that would recreate the earlier drift the project is trying to eliminate.

## Architectural Notes
The new brief sits on top of the shared hardening contract:
- `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
- `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`

The `garde` brief adds the family-specific details those shared docs intentionally do not own:
- multi-root ownership
- conditional gating by garde presence
- covering `clippy.toml` resolution per owned root
- `RS-GARDE-10` fail-closed behavior
- known carry-forward gaps from the legacy garde design material

## Information Sources
- `AGENTS.md`
- `.plans/todo/checks/rs/garde.md`
- `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
- `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
- `apps/guardrail3/crates/app/rs/checks/rs/garde/`
- `apps/guardrail3/tests/unit/test_garde_checks.rs`
- `.worklogs/2026-03-24-082050-reconcile-rust-plan-contracts.md`

## Open Questions / Future Considerations
- The brief records live architectural gaps, but it does not resolve them. In particular, wrapper-based validation surfaces and expanded extractor bans are still active follow-up work.
- If the family hardening pass materially changes the `garde` plan contract, this brief should be updated with the new reality rather than left as stale guidance.

## Key Files for Context
- `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md` — the new droppable handoff for `rs/garde`
- `.plans/todo/checks/rs/garde.md` — authoritative family contract and live carry-forward gaps
- `.plans/todo/check_review/test_hardening/00-shared-test-story.md` — repo-wide rule hardening model
- `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md` — required agent behavior and deliverables
- `apps/guardrail3/crates/app/rs/checks/rs/garde/` — current implementation surface the new agent will harden
- `apps/guardrail3/tests/unit/test_garde_checks.rs` — legacy garde corpus to mine for attack vectors
- `.worklogs/2026-03-24-082050-reconcile-rust-plan-contracts.md` — recent plan-contract reconciliation for `garde` and other Rust families

## Next Steps / Continuation Plan
1. Hand `.plans/todo/check_review/test_hardening/19-garde-agent-brief.md` to a fresh agent as the entrypoint for the `rs/garde` hardening pass.
2. Have that agent verify the current `garde` structure against the brief, especially the still-present `*_tests.rs` sidecars versus the required `*_tests/` directories.
3. After the `garde` pass returns, update both `garde.md` and the brief with any newly closed gaps or newly discovered semantic issues so the handoff stays current.
