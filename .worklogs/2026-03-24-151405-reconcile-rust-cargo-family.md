# Reconcile Rust Cargo Family

**Date:** 2026-03-24 15:14
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/cargo/**`, `.plans/todo/checks/rs/cargo.md`, `.plans/todo/check_review/test_hardening/20-cargo-agent-brief.md`

## Summary
Committed the `rs/cargo` reconciliation batch that brings the family in line with the multi-root policy-root model defined in the plan. The batch expands discovery/facts/inputs, adds the later cargo rules, migrates legacy sidecars into rule-specific test directories, and updates the cargo plan/brief to describe the implemented state accurately.

## Context & Problem
Earlier planning work established that `rs/cargo` was the main outlier among Rust families: the plan had moved to a policy-root model, but the implementation and tests were still shaped around a simpler root-only story. The dirty tree contained the code and test work to close that gap:
- owned policy-root discovery across workspaces and standalone packages
- more explicit cargo-family input failure handling
- new cargo rules around macro-ban enforcement, extra allows, member-local allows, and split metadata policy
- conversion away from old monolithic `*_tests.rs` sidecars

This needed its own commit because cargo is a foundation family with its own plan and its own handoff brief.

## Decisions Made

### Keep the whole cargo family in one commit
- **Chose:** Commit discovery, facts, inputs, rules, tests, and cargo docs together.
- **Why:** The multi-root model is only understandable when the code and the plan move together.
- **Alternatives considered:**
  - Split discovery from rules — rejected because the later rules depend on the new ownership model.
  - Commit test migration separately — rejected because the new tests are the evidence that the new ownership semantics work.

### Treat the family as implemented against the plan, not still “planned”
- **Chose:** Update the plan and brief to describe the implemented multi-root model rather than leaving “planned” placeholders in place.
- **Why:** The user has been explicit that plans should be authoritative and that later verification should compare code against a trustworthy contract.
- **Alternatives considered:**
  - Leave status text half-planned to be conservative — rejected because it would misdescribe the state after the code batch lands.

### Preserve cargo-only ownership for manifest lint policy
- **Chose:** Keep manifest-side lint enforcement (`allow` handling, inheritance, macro-ban enforcement) inside `rs/cargo`.
- **Why:** These are `Cargo.toml` policy checks, not `clippy.toml` checks. Keeping them here preserves clean family ownership.
- **Alternatives considered:**
  - Push some allow/macro policy into `rs/clippy` — rejected because that would blur the manifest-vs-clippy boundary.

## Architectural Notes
- `rs/cargo` now follows the same broad policy-root model as the stronger config families:
  - workspace roots
  - standalone package roots
  - member manifests for member-owned rules
- The batch also completes the rule split the plan called for:
  - edition policy separated from `rust-version` / MSRV policy
  - explicit input-failure rule
- Test structure moves toward the standard per-rule directory shape instead of flat sidecar files.

## Information Sources
- `.plans/todo/checks/rs/cargo.md`
- `.plans/todo/check_review/test_hardening/20-cargo-agent-brief.md`
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/**`
- prior planning/worklog chain around cargo-family under-specification

## Open Questions / Future Considerations
- The broader Rust validation runtime cutover is still separate work; this commit only reconciles the cargo family itself.
- If later runtime wiring exposes new family-selection or reporting edge cases, those should be handled in the runtime cutover work, not by weakening cargo-family ownership.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/mod.rs` — family orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs` — owned policy-root discovery
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/facts.rs` — normalized cargo-family facts
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/test_support.rs` — family test substrate
- `.plans/todo/checks/rs/cargo.md` — authoritative cargo-family contract
- `.plans/todo/check_review/test_hardening/20-cargo-agent-brief.md` — updated cargo-family handoff

## Next Steps / Continuation Plan
1. Commit the clippy/deny family packet, including canonical ban-surface changes and parity coverage.
2. Commit the code family packet with the expanded adversarial rule coverage and related fixture updates.
3. Continue through garde, deps, and hexarch/shared-discovery until the worktree is clean.
