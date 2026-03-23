# Code Agent Brief

You own the `rs/code` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/02-code.md`
6. `.plans/todo/checks/rs/code.md`

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

## Known gaps already identified

- a first migration batch has started, but many rules are still `*_tests.rs` instead of rule-specific `*_tests/` directories
- `rs/code` still depends on legacy `ast_helpers` in parts of the family
- whole-type `#[garde(skip)]` ownership is still missing explicitly
- grouped/aliased attribute edge cases still need deeper attacks
- `RS-CODE-30` added fail-closed input handling, but the adversarial depth is still shallow
- the existing reusable golden tree under `apps/guardrail3/tests/fixtures/r_arch_01/golden/` is now populated enough to act as the shared mutation target; the remaining gap is converting rule tests to use it broadly

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

## Do not

- add helper abstractions that hide source-rule semantics
- write small local-only mutations when the attack should apply across multiple files
- silently narrow the rule contract
