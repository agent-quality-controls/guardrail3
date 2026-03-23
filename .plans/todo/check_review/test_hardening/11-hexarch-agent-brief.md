# Hexarch Agent Brief

You own the `rs/hexarch` hardening pass.

## Read first

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/check_review/test_hardening/01-hexarch.md`
6. `.plans/todo/checks/rs/hexarch.md`

## Primary code

- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/`

## Old adversarial sources to mine

- `apps/guardrail3/tests/unit/test_hex_arch_checks.rs`
- `apps/guardrail3/tests/unit/rs_arch_01/`
- `apps/guardrail3/tests/fixtures/r_arch_01/`

## What you are trying to prove

The family should survive broad structural attacks against:
- all Rust app hex roots
- nested hex roots
- workspace-member coverage
- dependency-direction rules
- boundary config bypasses

One test = one attack vector.

That test should mutate the golden fixture everywhere the vector should matter:
- all matching top-level hex roots
- all matching nested hex roots
- all matching workspace members or dependency edges

## Known gaps already identified

- tests are still `*_tests.rs` instead of rule-specific `*_tests/` directories
- migrated test depth is weaker than the old deliberate corpus
- `rs/hexarch` still fails open on unreadable/unparsable Rust source for `RS-HEXARCH-22/23`
- `rs/hexarch` still fails open on malformed `guardrail3.toml` for boundary-config checks
- need direct proof that nested and top-level roots are attacked together
- `crates/macros/` is optional and must be allowed without weakening the rest of the structure

## Required attack classes

### Structural roots
- golden
- missing required dirs across all owned hex roots
- illegal extra sibling across all owned hex roots
- nested root parity
- optional `macros/`
- false positives against non-owned or non-Rust roots

### Workspace coverage
- missing members everywhere
- extra members everywhere
- out-of-boundary members
- malformed Cargo.toml fail-closed

### Dependency / boundary
- illegal direction edges across all matching members
- renamed dependency bypass
- inherited workspace dependency bypass
- target/dev edge variants
- cross-app leaks
- malformed boundary config fail-closed

## Structural requirement

Every rule must end with a rule-specific `*_tests/` directory.

Do not leave `*_tests.rs` rule files in place.

## Done means

- every `RS-HEXARCH-*` rule has a `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact owned hit/non-hit assertions are used
- semantic bugs found during hardening are fixed or written into the lane file

## Do not

- port old tests mechanically
- keep grouped or loose “some result exists” assertions
- change rule policy silently just to satisfy a test
