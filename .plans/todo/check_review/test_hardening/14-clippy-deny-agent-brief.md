# Clippy And Deny Agent Brief

You own the `rs/clippy` and `rs/deny` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md`
6. `.plans/todo/checks/rs/clippy.md`
7. `.plans/todo/checks/rs/deny.md`

## Primary code

- `apps/guardrail3/crates/app/rs/checks/rs/clippy/`
- `apps/guardrail3/crates/app/rs/checks/rs/deny/`
- `apps/guardrail3/crates/domain/modules/clippy/`
- `apps/guardrail3/crates/domain/modules/deny.rs`

## Old adversarial sources to mine

- `apps/guardrail3/tests/adversarial_config_tests.rs`
- `apps/guardrail3/tests/fixtures/adversarial-configs/`
- `apps/guardrail3/crates/app/rs/validate/clippy_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/clippy_coverage.rs`
- old deny validator files under `apps/guardrail3/crates/app/rs/validate/`
- `apps/guardrail3/tests/unit/deny_inventory_test.rs`
- `apps/guardrail3/tests/adversarial_generate.rs`

## What you are trying to prove

These families should not drift silently from the canonical generator modules, and config-policy edge cases should not slip through.

One test = one attack vector.

That vector should be applied across all relevant policy roots / config roots / profile mixes.

## Known gaps already identified

### Clippy
- tests are still `*_tests.rs` instead of rule-specific `*_tests/` directories
- generator/checker still disagree on some global-state bans
- no direct generator-vs-checker parity test
- `RS-CLIPPY-19` is intentionally temporary and must be tested honestly, not overclaimed

### Deny
- tests are still `*_tests.rs` instead of rule-specific `*_tests/` directories
- generation still uses wrong effective-profile logic in mixed setups
- no strong generator-vs-checker parity test
- canonical deny fixture has drifted before
- `RS-DENY-19` still needs explicit policy resolution if not already settled during the pass

## Required attack classes

- policy-root placement and shadowing
- same-root precedence conflicts
- mixed profile/layer cases
- generator/checker drift
- malformed exceptions/skips/ignores/wrappers
- severity exactness on inventory vs hard errors
- temporary-heuristic behavior for `RS-CLIPPY-19`

## Structural requirement

Every rule must end with a rule-specific `*_tests/` directory.

Do not leave `*_tests.rs` rule files in place.

## Done means

- every `RS-CLIPPY-*` and `RS-DENY-*` rule has a `*_tests/` directory
- parity tests exist against the generator baseline
- drift-prone hardcoded fixtures are removed or parity-checked
- every touched rule has golden + attack-vector coverage

## Do not

- let the checker and generator keep separate silent baselines
- write tests that only look for broad family output
- hide policy decisions inside test fixtures
