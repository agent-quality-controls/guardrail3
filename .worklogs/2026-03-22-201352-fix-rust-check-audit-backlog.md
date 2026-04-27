# Fix Rust Check Audit Backlog

**Date:** 2026-03-22 20:13
**Scope:** Rust check families under `apps/guardrail3/crates/app/rs/checks/rs/{cargo,clippy,deny,fmt,toolchain}`, related plan docs under `.plans/todo/checks/rs` and `.plans/by_file/rs`

## Summary
This pass fixed the concrete audit findings left after the structural normalization commit. The work tightened real `deny` and `clippy` semantics, removed a shared multi-rule threshold predicate from `clippy`, strengthened many weak rule tests, and updated the `deny` / `clippy` plans where policy had changed. The batch was driven by adversarial review rather than feature expansion.

## Context & Problem
After `0ae75db`, the file layout was clean, but the adversarial audits still found real semantic bugs and test shortcuts:
- `g3rs-deny/duplicate-entries` and `g3rs-deny/unknown-keys` were inventorying malformed or reasonless exception entries as though they were valid.
- `RS-DENY-30` ignored project-specific wrappers for non-canonical bans and treated canonical empty-wrapper additions too loosely.
- `g3rs-deny/allow-override-channel` missed unknown keys inside nested escape-hatch tables.
- `RS-DENY-26` only covered one ban-entry shape.
- `RS-CLIPPY-*` threshold rules were still thin wrappers around one shared helper predicate in `clippy_support.rs`.
- Many tests still asserted only “some result with this ID exists”, or used noisy shared fixtures, or routed through the family orchestrator rather than the specific rule surface.
- A fresh audit then found an unmodeled same-root `clippy.toml` / `.clippy.toml` conflict and a remaining deny wrapper-policy mismatch.

The user explicitly asked to fix everything, not just high-severity issues, and to avoid shortcuts or bundled “good enough” structure.

## Decisions Made

### Fix `deny` semantics before broadening test cleanup
- **Chose:** Correct the concrete `deny` rule bugs first, then harden the tests around those rules.
- **Why:** The audit had found real correctness gaps, not just style problems. Fixing test structure before repairing the semantics would have strengthened the wrong behavior.
- **Alternatives considered:**
  - Start by refactoring all remaining tests to direct rule calls — rejected because it would delay actual correctness fixes.
  - Only patch the tests to match existing `deny` behavior — rejected because the behavior itself was wrong.

### Remove the shared clippy threshold-rule predicate
- **Chose:** Delete `check_threshold_rule(...)` from `clippy_support.rs` and move the threshold-specific predicate/result logic back into each threshold rule file.
- **Why:** The current architecture forbids helpers that hide multiple rule predicates behind one API. The threshold helper violated that rule in spirit even after the file-per-rule normalization.
- **Alternatives considered:**
  - Keep the helper and merely add stronger tests — rejected because the architectural shortcut itself was the problem.
  - Replace it with a smaller shared result-builder helper — rejected for now because it still risks centralizing rule semantics again.

### Treat same-root dual clippy configs as an explicit placement error
- **Chose:** Extend clippy facts to model forbidden same-root sibling configs (`clippy.toml` vs `.clippy.toml`) using precedence, with the lower-precedence sibling becoming an `RS-CLIPPY-12` error.
- **Why:** Allowing both files to be “validated” at the same root created an ambiguous effective policy surface. This was a real semantic hole found by the second audit pass.
- **Alternatives considered:**
  - Leave both allowed and rely on filename precedence only for coverage — rejected because that still validates contradictory sibling configs instead of rejecting them.
  - Add a new rule ID — rejected because this is still a placement/shadowing concern and fits `RS-CLIPPY-12`.

### Align deny feature-ban enforcement with the generator baseline
- **Chose:** Make `g3rs-deny/skip-hygiene` enforce both `deny = ["full"]` and the canonical tokio `allow = [...]` set, and update the plan docs to match.
- **Why:** The generator already treats the tokio feature policy as a concrete baseline. Ignoring the allow-list side left a drift hole between generator and checker.
- **Alternatives considered:**
  - Keep only `full` as enforced and leave `allow` user-owned — rejected because the canonical generator already hardens the allow-list.
  - Split the allow-list into a separate new rule — rejected for now because the current plan can cleanly carry the whole tokio feature policy in `g3rs-deny/skip-hygiene`.

### Replace deny generator-coupled fixtures with a handwritten adversarial baseline
- **Chose:** Rewrite `canonical_deny_toml_service()` in deny test support as a hand-written baseline string instead of calling `build_deny_toml(...)`.
- **Why:** Generator-coupled tests allow generator and checker to drift together and still pass. The deny audit called that out directly.
- **Alternatives considered:**
  - Keep generated fixtures and add a few manual ones — rejected because the central shared fixture would remain coupled.
  - Delete the shared fixture entirely — rejected because a shared handcrafted baseline is still useful if it is independent from generation.

### Tighten rule tests toward exact branch assertions
- **Chose:** Upgrade a meaningful set of weak tests to assert exact severity/title/message/file and, where feasible, call the rule module directly rather than the family orchestrator.
- **Why:** The user’s stated standard is “tests should break the code, not confirm it.” ID-only assertions do not meet that bar.
- **Alternatives considered:**
  - Do a complete rule-by-rule isolation sweep across every family in this batch — rejected as too large for a single safe patch after already making semantic fixes.
  - Leave tests as-is once semantics were fixed — rejected because the audits specifically called out the test weakness.

## Architectural Notes
- `clippy_support.rs` is now back to normalization/data helpers rather than a multi-rule predicate engine.
- `clippy` facts now distinguish:
  - allowed configs
  - forbidden configs because they are not at allowed roots
  - forbidden configs because they are shadowed at the same root by a higher-precedence sibling
- `deny_support.rs` now has richer nested-key knowledge and a feature-entry shape that includes both `deny` and `allow`.
- `cargo/discover_tests.rs` was introduced to move a non-rule discovery/input-binding assertion out of an `RS-CARGO-10` rule test file.
- The batch intentionally improved rule-local test quality without pretending the entire remaining family-level test-orchestrator coupling is solved. That is the next cleanup line.

## Information Sources
- Prior recent worklogs:
  - `.worklogs/2026-03-22-192520-normalize-check-family-structure.md`
  - `.worklogs/2026-03-22-170103-clippy-completeness-finish.md`
  - `.worklogs/2026-03-22-164943-clippy-audit-fixes.md`
- Current plan docs:
  - `.plans/todo/checks/rs/clippy.md`
  - `.plans/todo/checks/rs/deny.md`
  - `.plans/by_file/rs/deny-toml.md`
- Current code paths:
  - `apps/guardrail3/crates/app/rs/checks/rs/clippy/**`
  - `apps/guardrail3/crates/app/rs/checks/rs/deny/**`
  - `apps/guardrail3/crates/app/rs/checks/rs/cargo/**`
- Adversarial agent passes in this session:
  - structural shortcut audit
  - deny semantic audit
  - cross-family drift audit
  - test-quality audit
- GitNexus checks:
  - `npx gitnexus status`
  - targeted `npx gitnexus impact ... --direction upstream` calls for edited symbols

## Open Questions / Future Considerations
- Many remaining rule tests across `fmt`, `toolchain`, `cargo`, `clippy`, and `deny` still call the family orchestrator instead of the rule module directly. This is medium-severity test-rigor debt, not a current semantic correctness blocker.
- `RS-CLIPPY-19` remains intentionally temporary and heuristic. That is documented, but it is still not a full canonical-key validator.
- `deny` wrapper policy may still need another policy decision if the project later distinguishes between:
  - canonical bans whose wrappers must exactly match
  - canonical bans whose wrappers are intentionally app-local

## Key Files for Context
- `AGENTS.md` — current repo instructions and Rust-only direction
- `.plans/todo/checks/rs/clippy.md` — frozen clippy policy contract including placement and threshold decisions
- `.plans/todo/checks/rs/deny.md` — current deny contract after audit reconciliation
- `.plans/by_file/rs/deny-toml.md` — by-file deny behavior and generator/validate policy split
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs` — clippy policy-root discovery, coverage, and same-root conflict handling
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/clippy_support.rs` — remaining clippy normalization helpers after threshold-helper removal
- `apps/guardrail3/crates/app/rs/checks/rs/deny/deny_support.rs` — deny baseline helpers and nested schema knowledge
- `apps/guardrail3/crates/app/rs/checks/rs/deny/rs_deny_config_18_tokio_full_ban.rs` — canonical tokio feature policy enforcement
- `apps/guardrail3/crates/app/rs/checks/rs/deny/test_support.rs` — handwritten deny baseline fixture used by tests
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover_tests.rs` — relocated non-rule discovery/input-binding test
- `.worklogs/2026-03-22-192520-normalize-check-family-structure.md` — prior structural normalization context

## Next Steps / Continuation Plan
1. Continue the remaining medium-severity test-rigor cleanup by family, starting with the current audit list:
   - `fmt` / `toolchain`: move remaining rule tests off `super::super::check` and onto rule-local inputs
   - `cargo`: isolate `g3rs-cargo/lint-levels`, `04`, `06`, `07`, `10` tests from the family orchestrator
   - `clippy`: isolate `RS-CLIPPY-04`, `05`, `06`, `07`, `08`, `13`, `14`, `20` tests from the family orchestrator
   - `deny`: add rule-local config inputs in test support and migrate the remaining config-rule tests off the family orchestrator
2. After that sweep, re-run the adversarial agents specifically against test quality and structural shortcuts, not just semantics.
3. Only once the test-isolation backlog is materially reduced, move on to the next Rust family implementation rather than carrying this debt forward.
