# Libarch Implementation Agent Brief

You own the `rs/libarch` lane.

Unlike the other remaining families, this one does not exist yet as a live family folder. This is a new family implementation task, not just hardening.

## Read First

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/checks/rs/libarch.md`
4. `.plans/todo/checks/rs/arch.md`
5. `.plans/todo/checks/rs/hexarch.md`
6. `.plans/todo/checks/rs/deps.md`
7. `.plans/todo/checks/rs/code.md`
8. `.plans/todo/check_review/test_hardening/27-libarch-agent-brief.md`
9. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`

## Primary Code

There is no family yet.

Create the live family under:

- `apps/guardrail3/crates/app/rs/families/libarch/`

Target shape:

- `README.md`
- `crates/runtime/`
- `crates/assertions/`
- optional `test_support/` if genuinely needed

## Family Status At Handoff

Current local reality:

- `.plans/todo/checks/rs/libarch.md` defines the planned rule inventory
- there is no corresponding family folder under `apps/guardrail3/crates/app/rs/families/`

This is the largest remaining Rust-family gap.

## Mission

Implement `rs/libarch` in the current family architecture.

Required outcomes:

1. create the family package group under `families/libarch/`
2. implement the planned `RS-LIBARCH-*` rules from `libarch.md`
3. keep one rule per file and one rule-specific `*_tests/` directory per rule
4. add family README and any required family-local assertions/test-support split
5. wire the family into the Rust runtime and selection/reporting pipeline if the enum and routing do not already cover it
6. update `.plans/todo/checks/rs/libarch.md`

## Constraints

- Do not collapse `libarch` into `arch`, `hexarch`, `deps`, or `code`
- Reuse shared placement and routing patterns instead of inventing a one-off discovery path
- Keep the family focused on layered-library escalation, workspace shape, crate-direction, and facade/export semantics
- Do not silently widen the policy beyond `libarch.md`

## Highest-Value Targets

1. correct fact/input design before writing rules
2. threshold/escalation ownership
3. exact layered workspace shape enforcement
4. dependency-direction rules between `api`, `core`, and `infra`
5. root facade export semantics
6. fail-closed behavior for malformed Cargo and source inputs

## Suggested Execution Order

1. read `libarch.md` completely and freeze the exact rule contracts
2. inspect `arch`, `hexarch`, `deps`, and `code` for reusable patterns only
3. create the family scaffolding and README
4. implement `facts.rs` and `inputs.rs`
5. land the shape/escalation rules first
6. land the dependency/facade rules second
7. wire the family into runtime/selection/reporting if needed
8. run family tests and any targeted validator coverage

## Done Means

This lane is not done until:

- `families/libarch/` exists and builds
- the planned `RS-LIBARCH-*` rules are implemented
- each rule has a production file and a rule-specific `*_tests/` directory
- the family is wired into the live Rust validation flow if required
- `libarch.md` no longer describes the family as planned-only
