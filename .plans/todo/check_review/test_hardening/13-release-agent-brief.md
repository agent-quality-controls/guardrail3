# Release Agent Brief

You own the `rs/release` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/03-release.md`
6. `.plans/todo/checks/rs/release.md`

## Primary code

- `apps/guardrail3/crates/app/rs/checks/rs/release/`

## Old adversarial sources to mine

- `apps/guardrail3/tests/unit/test_release_checks.rs`
- `apps/guardrail3/tests/unit/test_release_repo_checks.rs`
- `apps/guardrail3/tests/unit/test_release_crate_checks.rs`
- `apps/guardrail3/tests/unit/test_release_crate_deps.rs`
- `apps/guardrail3/tests/unit/test_release_bin_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/release_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/release_repo_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/release_crate_checks.rs`
- `apps/guardrail3/crates/app/rs/validate/release_crate_deps.rs`
- `apps/guardrail3/crates/app/rs/validate/release_bin_checks.rs`

## What you are trying to prove

The family should detect real release-wiring failures, not comments/prose that accidentally contain the right strings.

One test = one attack vector.

That vector should be applied across all relevant release surfaces:
- repo configs
- publishable crates
- dependent publishable crates
- binary workflow targets

## Known gaps already identified

- tests are still `*_tests.rs` instead of rule-specific `*_tests/` directories
- workflow rules still overclaim semantic strength while using broad string matching
- `readme = false` is still buggy
- `RS-PUB-10/11` still miss `workspace = true` inherited local path edges
- `RS-RELEASE-12` is only partially fail-closed
- some rule inputs are still too aggregate-heavy
- semantic `release-plz.toml` / `cliff.toml` baseline is still only partly real

## Required attack classes

- fake workflow hits via comments or prose
- missing real executable release step
- inherited path-edge attacks
- publishability inference bugs
- `readme = false`
- malformed release config / partial facts
- false positives for non-publishable crates

## Structural requirement

Every rule must end with a rule-specific `*_tests/` directory.

Do not leave `*_tests.rs` rule files in place.

## Done means

- every `RS-RELEASE-*`, `RS-PUB-*`, and `RS-BIN-*` rule has a `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- prose/comments cannot satisfy workflow rules in the hardened suite
- inherited path-edge cases are attacked directly

## Do not

- preserve old substring heuristics just because they existed before
- write tests that only prove “some release error exists”
- silently narrow publishability policy
