# Code Agent Brief

You own the `rs/code` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/02-code.md`
6. `.plans/todo/checks/rs/code.md`
7. `.plans/todo/check_review/test_hardening/12-code-execution-plan.md`

## Primary code

- `apps/guardrail3/crates/app/rs/checks/rs/code/`

## Old adversarial sources to mine

- `apps/guardrail3/tests/unit/test_source_scan.rs`
- `apps/guardrail3/crates/app/rs/validate/source_scan.rs`
- `apps/guardrail3/crates/app/rs/validate/ast_helpers.rs`
- `apps/guardrail3/crates/app/rs/validate/ast_visitors.rs`

## What you are trying to prove

Source-level bypasses should not slip through via:
- attribute placement
- aliasing
- grouped imports
- nested modules
- prod/test path confusion
- parse/read failure
- public API edge cases

One test = one attack vector.

That vector should be applied across all relevant source files in the golden tree, not just one toy file.

## Current state

The family is no longer in the initial migration phase.

The structural conversion to rule-specific `*_tests/` directories is complete across `RS-CODE-01..30`.

The main work pattern is now repeated adversarial convergence per rule:

1. inspect the current rule/tests against the active plan
2. run a targeted compile/test pass when unrelated repo blockers permit
3. attack the rule from four angles:
   - completeness
   - missing scenarios
   - pattern parity
   - false positives / exactness
4. fix real semantic bugs first, not just tests
5. rerun attacks until the remaining findings are only combinatorial noise or policy-ambiguous expansion

The populated shared golden fixture at `apps/guardrail3/tests/fixtures/r_arch_01/golden/` is the normal mutation target now.

Do not fall back to tiny snippet-only tests unless the rule is inherently local and the direct branch is the point being tested.

## Skill / workflow note

Use the `test-attack` skill for this lane.

If parallel attack work is explicitly requested, split the attack into focused passes:
- completeness
- missing scenarios
- pattern parity
- false positives / exactness

Do not treat every subagent suggestion as mandatory.

A finding is worth patching only if it is one of:
- a real false negative
- a real false positive
- a rule-boundary bug
- a fail-open hole
- a material exactness gap that could let the rule regress silently

Stop expanding when the remaining findings are just combinatorial exhaustiveness with no new bug class.

## Verified family status

- `RS-CODE-01..19` have already gone through repeated attack-loop hardening in this lane.
- `RS-CODE-20` has now also been pushed through convergence.
- `RS-CODE-21..30` are in an active exactness/convergence batch; do not treat them as untouched.

`RS-CODE-20` current decisions:
- `RS-CODE-20` is the owner for foreign-mod allow suppression.
- `RS-CODE-03/04` no longer claim `ForeignMod` item-level allows.
- `RS-CODE-20` now covers direct `#[allow(...)]` and `#[cfg_attr(..., allow(...))]` on foreign mods.
- `RS-CODE-18` no longer claims always-true `cfg_attr(..., allow(...))` on foreign mods; that overlap was intentionally removed so foreign-mod suppression has one owner.

Verification state at handoff:
- `cargo check -p guardrail3 --lib` is green for the current `rs/code` work.
- `cargo check -p guardrail3 --tests` is still blocked by unrelated code outside this family in `apps/guardrail3/crates/app/rs/checks/rs/release/rs_bin_01_binary_release_workflow_tests/bypasses.rs`.

Do not “fix the world” outside `rs/code` unless the user explicitly expands scope.

## Known gaps already identified

- `rs/code` still depends on legacy `ast_helpers` in parts of the family
- some rules still need deeper grouped / aliased / precedence attacks even after structural migration
- `RS-CODE-21..30` have not all gone through the same repeated convergence loop yet
- `RS-CODE-30` has fail-closed coverage, but that branch should still be revisited with the same adversarial protocol used on the other rules

## Required attack classes

- crate-level and item-level allow bypasses
- `cfg_attr` bypasses
- `include!` / path-attr bypasses
- grouped `use std::{fs::*, ...}` and aliasing variants
- `unsafe`, panic, todo, unwrap/expect escapes
- facade/lib organization and public API leakage
- parse/read failures and malformed source
- false positives for similar legal syntax

## Structural requirement

Every rule must end with a rule-specific `*_tests/` directory.

Do not leave `*_tests.rs` rule files in place.

## Done means

- every `RS-CODE-*` rule has a `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact file hit sets are asserted where practical
- legacy helper dependence is reduced or explicitly documented
- the shared golden tree is populated enough that source-rule mutations happen against realistic Rust and TypeScript code rather than comment-only placeholders
- repeated adversarial passes no longer surface material rule-local bugs

## Do not

- add helper abstractions that hide source-rule semantics
- write small local-only mutations when the attack should apply across multiple files
- silently narrow the rule contract

## Resume point

Resume in the middle of the `RS-CODE-21..30` verification batch.

Before moving on:
- trust the current `RS-CODE-20` ownership decisions unless a plan-level contradiction is found
- do not reopen `RS-CODE-20` for optional combinatorics
- keep documenting every finished rule in `02-code.md`
- `RS-CODE-21..30` have had a broad exactness-tightening pass; the next step is to finish targeted cargo verification on `target/code` and only then decide which rules need another material attack pass

Suggested next sequence:
1. finish targeted verification for the current `RS-CODE-21..30` batch on `target/code`
2. fix the first real compile/test failures that surface from that run
3. rerun focused rule tests on the affected rules until the batch is green
4. only then decide whether any rule still needs another material attack pass
