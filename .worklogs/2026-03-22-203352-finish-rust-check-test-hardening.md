# Finish Rust Check Test Hardening

**Date:** 2026-03-22 20:33
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/{fmt,toolchain,cargo,clippy,deny}`, family `test_support.rs` files

## Summary
Completed the post-audit cleanup pass for the Rust check families by removing the remaining family-entry test shortcuts and converting the affected tests to rule-local inputs and exact branch assertions. The result is that `fmt`, `toolchain`, `cargo`, `clippy`, and `deny` now pass the full `checks::rs` suite with tests aimed at the individual rule surfaces rather than incidental family orchestration.

## Context & Problem
After the structural one-rule/one-test normalization, an adversarial review still found medium-severity quality issues in the test layer. The main problem was that a number of rule test files were still exercising the family `check(&ProjectTree)` entrypoint or using broad assertions like “some result with this ID exists,” which allowed the wrong branch, wrong severity, or wrong file to satisfy the test. The user explicitly rejected any remaining shortcuts and required the cleanup to continue past high-severity items.

## Decisions Made

### Remove remaining family-entry leakage from rule tests
- **Chose:** Convert the remaining `fmt`, `toolchain`, `cargo`, `clippy`, and `deny` tests to build typed rule inputs directly wherever possible, or to collect family facts once and feed the specific rule input for coverage/placement rules.
- **Why:** A rule test should attack the rule surface itself. Hitting the family orchestrator in a rule test reintroduces broad incidental behavior and lets unrelated rule branches satisfy the assertion.
- **Alternatives considered:**
  - Keep using family entrypoints for “structural” rules like coverage and placement — rejected because those rules still have typed input surfaces after fact collection.
  - Leave the tests as-is because they already passed — rejected because that only confirmed smoke coverage, not branch correctness.

### Keep helper files only as input/fact builders, not as hidden rule logic
- **Chose:** Extend family `test_support.rs` only where needed to expose collected facts and typed inputs for tests, while removing dead helpers that no longer had callers.
- **Why:** Tests need lightweight fixtures and fact extraction, but they should not rely on family-wide execution helpers or stale convenience functions once rule-local inputs exist.
- **Alternatives considered:**
  - Inline all fixture construction in every single test file — rejected because it would bloat tests and obscure the exact assertion being made.
  - Keep old tree-based helpers around “just in case” — rejected because dead helpers are now compile errors and become a source of future shortcuts.

### Prefer exact branch assertions over generic ID presence
- **Chose:** Tighten the modified tests to assert the expected severity, title, message, file, and inventory state for the specific rule branch under attack.
- **Why:** The cleanup goal was to make tests break the implementation, not merely confirm that some result exists. Exact assertions materially reduce the chance of regressions slipping through on the wrong branch.
- **Alternatives considered:**
  - Only assert rule ID and severity — rejected because several earlier failures showed the wrong message/file branch could still pass.
  - Snapshot the entire family output — rejected because that would re-couple tests to family orchestration and create noisy, low-signal snapshots.

## Architectural Notes
The important architectural outcome is that the test layer now better matches the checker architecture:
- orchestrators discover and build facts
- typed inputs represent one rule-sized assertion surface
- rule tests target those inputs directly

Coverage and placement rules were the main awkward cases because they depend on family discovery. The final pattern there is:
1. build a focused fixture tree
2. collect family facts once
3. pick the exact covered/uncovered/forbidden/conflict fact for the rule
4. run the rule function directly

That keeps rule tests honest without pretending those rules can be tested in isolation from fact collection.

## Information Sources
- `AGENTS.md` — current architecture, test, and worklog rules
- `.worklogs/2026-03-22-201352-fix-rust-check-audit-backlog.md` — previous hardening pass that fixed the high-severity issues and left the medium-severity test-quality backlog
- Adversarial agent findings from the current session identifying remaining medium issues in `fmt`, `toolchain`, `cargo`, `clippy`, and `deny`
- `npx gitnexus status` — verified index freshness before the commit path
- `cargo test --lib checks::rs::{fmt,toolchain,cargo,clippy,deny} --quiet`
- `cargo test --lib checks::rs --quiet`

## Open Questions / Future Considerations
- Some rule tests still use `assert!(results.iter().any(...))`, but after this pass those are direct rule tests with tight predicates rather than family smoke tests. If the project wants an even stricter test style later, the next step would be to normalize more of those to `assert_eq!(results.len(), N)` plus index-specific assertions.
- `AGENTS.md` and `CLAUDE.md` were dirty from unrelated automation in the worktree and were intentionally left out of this commit.
- The GitNexus instructions mention `detect_changes()`, but the available local CLI surface in this repo exposed `status`; scope was checked with `git diff --stat` before commit.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/deny/test_support.rs` — deny fact/input helpers used by coverage, placement, and config-rule tests
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/test_support.rs` — clippy fact/input helpers for direct coverage and placement tests
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/test_support.rs` — cargo fact/input helpers for direct member and workspace rule tests
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/*_tests.rs` — direct rule-local fmt tests after removal of family-entry smoke coverage
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/*_tests.rs` — direct rule-local toolchain tests
- `.worklogs/2026-03-22-201352-fix-rust-check-audit-backlog.md` — previous audit fix pass and the context for why this cleanup was necessary

## Next Steps / Continuation Plan
1. Run one more adversarial audit over the now-hardened Rust families if the user wants a fresh external pass after this commit; focus on semantic mismatches rather than structural/test shortcuts.
2. If the user wants to continue implementation, move to the next unfinished Rust family after the config families, starting from the current plan docs under `.plans/todo/checks/rs`.
3. Keep excluding unrelated `AGENTS.md` / `CLAUDE.md` automation dirt unless the user explicitly asks for a docs cleanup commit.
