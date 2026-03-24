# Repair Rust Agent Briefs

**Date:** 2026-03-24 12:12
**Scope:** `AGENTS.md`, `.plans/todo/check_review/test_hardening/README.md`, `.plans/todo/check_review/test_hardening/20-cargo-agent-brief.md`, `.plans/todo/check_review/test_hardening/21-deps-agent-brief.md`, `.plans/todo/check_review/test_hardening/22-hooks-shared-agent-brief.md`, `.plans/todo/check_review/test_hardening/23-hooks-rs-agent-brief.md`, `.plans/todo/check_review/test_hardening/24-fmt-agent-brief.md`, `.plans/todo/check_review/test_hardening/25-toolchain-agent-brief.md`, `.plans/todo/check_review/test_hardening/26-test-agent-brief.md`, `.plans/todo/check_review/test_hardening/27-libarch-agent-brief.md`

## Summary
Repaired the Rust-family agent handoff set after adversarial review found stale, contradictory, and incomplete briefs. Aligned the repo-level test-pattern contract with the new rule-specific `*_tests/` directory standard and fixed family briefs that were missing cross-family dependencies or lane-level context.

## Context & Problem
The project now relies on droppable per-family handoff briefs so multiple terminals or subagents can harden families independently without bespoke explanation. After generating the missing briefs, adversarial reviewers found several real problems:
- the top-level `AGENTS.md` still instructed fresh agents to use `*_tests.rs`
- split hook briefs had dropped critical lane context that still lived only in the combined hook brief
- the cargo brief had drifted behind the current `cargo.md` contract
- the deps brief still told agents to do a migration that had already happened
- some new briefs omitted required cross-family plan docs
- the new `libarch` brief overconstrained the root shape relative to the plan

If left unfixed, a “droppable” agent brief would not actually be trustworthy as a standalone handoff.

## Decisions Made

### Align repo-level instructions with the new test-module standard
- **Chose:** update `AGENTS.md` so it now describes one rule-specific `*_tests/` directory per rule, with `mod.rs` path wiring.
- **Why:** every brief tells the agent to read `AGENTS.md` first. Leaving `AGENTS.md` on the old `*_tests.rs` contract made every new brief internally contradictory.
- **Alternatives considered:**
  - Leave `AGENTS.md` alone and let the briefs override it — rejected because that makes the primary cold-start doc untrustworthy.
  - Revert the briefs back to `*_tests.rs` — rejected because the current architecture/test-hardening contract has already moved to per-rule directories.

### Keep combined and split hook briefs, but make the distinction explicit
- **Chose:** keep `15-hooks-agent-brief.md` as the combined migration lane and add/repair `22-hooks-shared-agent-brief.md` and `23-hooks-rs-agent-brief.md` as split family briefs, with the README explicitly differentiating them.
- **Why:** the combined hook lane still carries routing/parity debt that is broader than either family, but we also need droppable family-specific hook handoffs.
- **Alternatives considered:**
  - Delete the combined hook brief — rejected because it still contains legitimate cross-family lane context.
  - Keep the split briefs purely family-local — rejected because reviewers showed they became unsafe when they dropped execution-plan and parity context.

### Repair stale family briefs instead of treating them as “good enough”
- **Chose:** patch `20-cargo-agent-brief.md` and `21-deps-agent-brief.md` immediately.
- **Why:** stale briefs in the active handoff directory are latent traps. Even if `cargo` is not the next lane to run, leaving a known-wrong handoff file behind reintroduces drift later.
- **Alternatives considered:**
  - Ignore stale briefs until those lanes are active — rejected because the handoff folder is supposed to be current and droppable.
  - Delete the stale briefs — rejected because the underlying family contracts still need corresponding handoff files.

### Make cross-family plan dependencies explicit in the briefs
- **Chose:** add `cargo.md` to `toolchain` and `fmt`, and add hook plans to `test`.
- **Why:** some families depend on adjacent plan contracts for correct interpretation of severity and ownership. A brief that omits those plans is incomplete even if it names the dependency in prose.
- **Alternatives considered:**
  - Mention dependencies without adding them to “Read First” — rejected because agents often stop at the listed reading set.
  - Copy dependency semantics into each brief — rejected because it duplicates policy and drifts faster.

## Architectural Notes
The handoff system now has two layers:
- global contract: `AGENTS.md`, checker architecture, shared test story, family playbook
- family packet: one brief per family or combined lane, pointing at the correct plan docs, code dirs, seed material, known gaps, and done criteria

The important constraint is that these briefs are not merely summaries. They are operational packets intended to be dropped into a fresh agent. That means contradictions against repo-level docs, stale rule inventories, or omitted cross-family contracts are architectural bugs in the planning layer, not minor doc polish issues.

## Information Sources
- `AGENTS.md`
- `.plans/todo/check_review/test_hardening/README.md`
- `.plans/todo/check_review/test_hardening/15-hooks-agent-brief.md`
- `.plans/todo/check_review/test_hardening/20-cargo-agent-brief.md`
- `.plans/todo/check_review/test_hardening/21-deps-agent-brief.md`
- `.plans/todo/check_review/test_hardening/22-hooks-shared-agent-brief.md`
- `.plans/todo/check_review/test_hardening/23-hooks-rs-agent-brief.md`
- `.plans/todo/check_review/test_hardening/24-fmt-agent-brief.md`
- `.plans/todo/check_review/test_hardening/25-toolchain-agent-brief.md`
- `.plans/todo/check_review/test_hardening/26-test-agent-brief.md`
- `.plans/todo/check_review/test_hardening/27-libarch-agent-brief.md`
- `.plans/todo/checks/rs/cargo.md`
- `.plans/todo/checks/rs/deps.md`
- `.plans/todo/checks/rs/fmt.md`
- `.plans/todo/checks/rs/toolchain.md`
- `.plans/todo/checks/rs/test.md`
- `.plans/todo/checks/rs/libarch.md`
- `.plans/todo/checks/hooks/shared.md`
- `.plans/todo/checks/hooks/rs.md`
- adversarial subagent reviews in this session covering handoff completeness and split-hook brief safety

## Open Questions / Future Considerations
- `cargo` still has a handoff brief even though that family implementation is under-reconciled; the brief is now current, but the lane should still be treated as architecture repair first.
- The hook lane still spans both split family work and broader routing/parity cleanup. The combined brief remains necessary until that debt is gone.
- The broader `test_hardening/` folder still contains many modified execution-plan and matrix docs from other lanes that were intentionally left out of this commit.

## Key Files for Context

- `AGENTS.md` — repo-level cold-start contract; now aligned to rule-specific `*_tests/` directories.
- `.plans/todo/check_review/test_hardening/README.md` — index of the handoff packet set and the combined-vs-split hook brief distinction.
- `.plans/todo/check_review/test_hardening/15-hooks-agent-brief.md` — combined hook lane context that still matters for routing/parity debt.
- `.plans/todo/check_review/test_hardening/22-hooks-shared-agent-brief.md` — split shared-hook handoff.
- `.plans/todo/check_review/test_hardening/23-hooks-rs-agent-brief.md` — split Rust-hook handoff.
- `.plans/todo/check_review/test_hardening/24-fmt-agent-brief.md` — fmt handoff with explicit cross-family plan reads.
- `.plans/todo/check_review/test_hardening/25-toolchain-agent-brief.md` — toolchain handoff with cargo linkage.
- `.plans/todo/check_review/test_hardening/26-test-agent-brief.md` — test handoff with hook-boundary docs included.
- `.plans/todo/check_review/test_hardening/27-libarch-agent-brief.md` — libarch implementation/hardening handoff.
- `.plans/todo/checks/rs/cargo.md` — current cargo contract used to repair the stale cargo brief.
- `.plans/todo/checks/rs/libarch.md` — source of truth for the libarch root-shape contract.
- `.worklogs/2026-03-24-114556-tighten-rust-plan-contracts.md` — prior planning pass that tightened Rust family contracts before handoff verification.

## Next Steps / Continuation Plan

1. Commit the repaired handoff packet set (`AGENTS.md`, handoff README, and family briefs) without pulling in unrelated dirty worktree files.
2. Run one more adversarial verification pass against the updated handoff set, specifically checking for any remaining contradiction between repo-level docs and family briefs.
3. If that pass comes back clean, either:
   - commit any final doc repairs from the verification pass, or
   - stop planning work here and return to family execution/hardening lanes.
