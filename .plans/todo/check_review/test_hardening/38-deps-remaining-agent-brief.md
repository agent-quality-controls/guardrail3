# Deps Remaining Agent Brief

You own the remaining `rs/deps` work.

This is not a broad family rewrite. The family exists under `families/deps/` and most of the rule inventory is already implemented. Your job is to close the remaining planned rule and tighten any directly related fact/test gaps that block completion.

## Read First

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/checks/rs/deps.md`
4. `.plans/todo/check_review/test_hardening/21-deps-agent-brief.md`
5. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
6. `apps/guardrail3/crates/app/rs/families/deps/README.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/`
- `apps/guardrail3/crates/app/rs/families/deps/crates/assertions/src/`
- `apps/guardrail3/crates/app/rs/families/deps/test_support/src/`

## Family Status At Handoff

The live remaining inventory gap is:

- `RS-DEPS-12` is still planned in `.plans/todo/checks/rs/deps.md`

The plan also still calls out target-specific dependency tables as a remaining gap. Do not expand policy beyond the plan unless the implementation of `RS-DEPS-12` explicitly requires it.

## Mission

Close the remaining `rs/deps` backlog.

Required outcomes:

1. implement `RS-DEPS-12`
2. keep the rule in the existing family architecture under `families/deps/`
3. add exact owned hit/non-hit coverage for the new rule
4. add any directly necessary supporting fact normalization
5. update `.plans/todo/checks/rs/deps.md`

## Constraints

- Stay inside `rs/deps`
- Do not mix in release-family dependency policy
- Do not silently broaden target-table semantics if the plan still leaves them open
- Keep one rule per file and rule-specific sidecar tests

## Highest-Value Targets

1. exact direct-dependency counting semantics for `RS-DEPS-12`
2. ownership of renamed packages and `workspace = true` edges if they affect the count
3. false-positive control for workspace/path/internal dependencies
4. fail-closed behavior if the rule depends on manifest parsing beyond what `RS-DEPS-11` already owns

## Suggested Execution Order

1. reread the `RS-DEPS-12` contract in `.plans/todo/checks/rs/deps.md`
2. inspect existing dependency fact normalization in `facts.rs`
3. decide whether `RS-DEPS-12` can reuse existing direct-dependency inventory or needs a small extension
4. implement the rule and its assertions
5. add golden, threshold-edge, false-positive, and malformed-input coverage
6. run the family tests and update the plan doc

## Done Means

This lane is not done until:

- `RS-DEPS-12` is implemented
- the rule has a production file and a rule-specific `*_tests/` directory
- family tests are green
- `deps.md` no longer marks `RS-DEPS-12` planned
- any remaining target-table policy gap is either still explicitly documented or intentionally resolved in the plan
