# Family Agent Playbook

This file tells one agent exactly what to do when assigned one Rust check family for hardening.

Use it as the per-terminal contract.

## Mission

Take one already-implemented Rust family and harden its tests so the rules are hard to bypass.

Do not expand scope to other families unless the work uncovers a shared architectural blocker.

## Inputs

Before changing anything, read:

1. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
2. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
3. the family-specific hardening file for your lane
4. the active family plan under `.plans/todo/checks/rs/`
5. the family code under `apps/guardrail3/crates/app/rs/checks/rs/<family>/`
6. any old test corpus that exists for the family

## Required outcomes

For your family:

1. verify structure
- one rule per production file
- one rule-specific test module directory per rule
- no grouped family test files
- no grouped rule files

2. build a coverage matrix
- for every rule, list attack classes currently covered
- list attack classes still missing

3. rewrite tests to the attack model
- one test = one attack vector
- each test mutates the golden fixture everywhere that vector should matter
- assert exact owned hits
- assert exact owned non-hits
- assert exact rule ID and severity

4. port old test ideas correctly
- do not port old tests mechanically
- map each old test to:
  - current rule ID
  - attack vector
  - whether it is still valid in the new architecture

5. find real semantic bugs
- false negatives
- false positives
- fail-open behavior
- scope leaks
- orchestrator/rule boundary violations
- plan/code mismatches

## Hard rules

Do not:
- add grouped family test files
- leave `*_tests.rs` files in place for rule tests
- write happy-path-only tests
- assert only “some result exists”
- hide rule semantics in test helpers
- change rule policy silently just to make tests pass

Do:
- use rule-specific `*_tests/` directories
- split test files by attack class
- mutate the golden fixture broadly for each attack vector
- prefer exact set assertions over loose contains checks
- document every found mismatch in the family hardening file

## Preferred test module layout

For each rule:

- `rs_xx_yy_rule_tests/`
- `mod.rs`
- `golden.rs`
- `attack_vector_a.rs`
- `attack_vector_b.rs`
- `false_positives.rs`
- `fail_closed.rs`
- `severity_exactness.rs`

Use only the files that make sense for the rule, but keep the layout semantic.

## Deliverables

At the end of the family pass, the agent must leave:

1. hardened test modules in code
2. any necessary bug fixes in the family implementation
3. the family hardening plan updated with:
  - closed gaps
  - remaining gaps
  - policy questions, if any

## Stop conditions

The family pass is not done until:

- every rule has a rule-specific test module directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact-result assertions replace loose “rule exists” checks in the touched modules
- newly found semantic bugs are either fixed or explicitly written down

## Escalate instead of guessing when

- the rule contract in the active plan is ambiguous
- generator and checker disagree and the correct policy is unclear
- the family needs a cross-family architectural change
- an old test idea conflicts with the new product direction
