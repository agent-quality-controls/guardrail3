# Harden Rust Code Family

**Date:** 2026-03-24 15:15
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/code/**`, `.plans/todo/check_review/06-new-rust-rule-candidates.md`, `.plans/todo/check_review/test_hardening/02-code.md`, `.plans/todo/check_review/test_hardening/12-code-agent-brief.md`, `.plans/todo/check_review/test_hardening/12-code-execution-plan.md`, `apps/guardrail3/crates/lib.rs`, `apps/guardrail3/tests/fixtures/r_arch_01/golden/**`

## Summary
Committed the large `rs/code` hardening batch. This expands attack-vector coverage across the code family, adds stronger direct/inventory/false-positive cases for later rules, updates the code-lane execution docs, and refreshes the shared golden fixture sources that these rule suites mutate.

## Context & Problem
The dirty tree contained one very large code-family packet:
- parser/facts updates in `rs/code`
- widened adversarial coverage for a long run of `RS-CODE-*` rules
- planning/docs updates for the code lane and the newly accepted string-dispatch candidate
- shared fixture source changes under `tests/fixtures/r_arch_01/golden/**`
- a crate-level recursion limit bump in `crates/lib.rs` to tolerate the larger compile-time shape of the expanded family test modules

These pieces belong together because the code family is heavily fixture-driven. The fixture edits are part of the rule-hardening packet, not generic repository churn.

## Decisions Made

### Keep fixture source edits with the code-family commit
- **Chose:** Include the modified golden fixture Rust sources in the `rs/code` batch.
- **Why:** The updated tests exercise facade-only-lib, public API, unwrap/expect, and related code-family semantics against those fixture sources.
- **Alternatives considered:**
  - Save the fixture edits for a later fixture-only commit — rejected because the fixture changes would become impossible to interpret without the code-family tests that motivated them.

### Treat the batch as a semantic hardening pass, not a test-only pass
- **Chose:** Describe the batch around parser/rule/test strengthening rather than just “more tests”.
- **Why:** `parse.rs`, `facts.rs`, and some production rule files changed, not only sidecars.
- **Alternatives considered:**
  - Frame it as documentation/test cleanup — rejected because the code family behavior changed materially.

### Record the new accepted string-dispatch rule in the candidate inventory
- **Chose:** Include the `06-new-rust-rule-candidates.md` update in this commit.
- **Why:** The code family now owns that accepted candidate, and the inventory change is directly tied to the family’s rule surface.
- **Alternatives considered:**
  - Leave the candidate inventory for a later docs-only commit — rejected because it would separate the planning record from the owning family’s changes.

## Architectural Notes
- The `rs/code` family continues to be the main source-analysis family for Rust-only structural/syntax guardrails.
- The family’s hardening pattern is now strongly attack-vector-based:
  - direct mutation cases
  - exact inventory assertions
  - explicit false-positive controls
  - fail-closed cases where appropriate
- The crate-level recursion-limit bump exists to support the larger generated/module-expanded test surface for this family packet.

## Information Sources
- `apps/guardrail3/crates/app/rs/checks/rs/code/**`
- `.plans/todo/check_review/06-new-rust-rule-candidates.md`
- `.plans/todo/check_review/test_hardening/02-code.md`
- `.plans/todo/check_review/test_hardening/12-code-agent-brief.md`
- `.plans/todo/check_review/test_hardening/12-code-execution-plan.md`
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/**`

## Open Questions / Future Considerations
- The code family is now much deeper, but the broader Rust runtime cutover still needs to make the new family architecture the actual public validator path.
- If the monolithic lib-test binary remains too expensive, the code family is one of the strongest candidates for future crate/test-target splitting.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/code/parse.rs` — source parsing and extraction logic used by the family
- `apps/guardrail3/crates/app/rs/checks/rs/code/facts.rs` — normalized code-family facts
- `apps/guardrail3/crates/app/rs/checks/rs/code/` — the full family rule/test packet
- `.plans/todo/check_review/test_hardening/02-code.md` — lane status and convergence notes
- `.plans/todo/check_review/test_hardening/12-code-agent-brief.md` — code-family handoff
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/` — shared golden Rust fixture mutated by many code-family tests

## Next Steps / Continuation Plan
1. Commit the garde family batch, including the new field/nesting/context rules and the updated family brief/plan.
2. Commit the deps family batch.
3. Finish with the hexarch/shared-discovery batch so the remaining family work is fully sorted.
