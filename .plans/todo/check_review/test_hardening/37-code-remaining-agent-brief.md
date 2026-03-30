# Code Remaining Agent Brief

You own the remaining `rs/code` work.

This is not a fresh family migration. The family already exists and most of the structural rewrite is done. Your job is to close the explicitly remaining rule inventory and any rule-local hardening gaps that block calling the family done.

## Read First

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/checks/rs/code.md`
4. `.plans/todo/checks/rs/code-family-stabilization.md`
5. `.plans/todo/check_review/test_hardening/02-code.md`
6. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
7. `apps/guardrail3/crates/app/rs/families/code/README.md`
8. `apps/guardrail3/crates/app/rs/families/code/FIXES.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/`
- `apps/guardrail3/crates/app/rs/families/code/crates/assertions/src/`
- `apps/guardrail3/crates/app/rs/families/code/test_support/src/`

## Family Status At Handoff

The family is implemented and already uses the family split under `families/code/`.

The remaining inventory gap is in the plan, not in the old structural rewrite:

- `RS-CODE-31` is still planned
- `RS-CODE-33` is still planned
- `RS-CODE-34` is still planned
- `RS-CODE-35` is still planned
- `RS-CODE-36` is still planned

There is also live stabilization debt tracked in:

- `.plans/todo/checks/rs/code-family-stabilization.md`

Do not spend time re-rewriting rules that are already structurally converged unless that is required to land one of the missing rules or a real semantic fix.

## Mission

Close the remaining `rs/code` backlog.

Required outcomes:

1. implement the still-planned `RS-CODE-*` rules from `code.md`
2. keep the one-rule-per-file and rule-specific `*_tests/` shape
3. add attack-vector coverage for every newly implemented rule
4. harden any adjacent rule-local semantics only where the new work exposes a real bug
5. update `.plans/todo/checks/rs/code.md`
6. update `apps/guardrail3/crates/app/rs/families/code/FIXES.md` if you close or discover family-local debt

## Constraints

- Stay inside `rs/code`
- Do not re-own `arch`, `hexarch`, `cargo`, `deps`, or `test` semantics
- Do not collapse multiple rule IDs into one production file
- Do not add grouped family tests
- Do not silently change existing rule policy to make new rules easier

## Highest-Value Targets

1. planned rule implementation
2. exact ownership boundaries around source-file policy vs architecture policy
3. fail-closed behavior for any new required inputs
4. exact hit/non-hit assertions for newly added rules

## Suggested Execution Order

1. read the planned-rule sections in `.plans/todo/checks/rs/code.md`
2. map each missing rule to the smallest fact/input shape that fits the current family architecture
3. implement facts/inputs first if any shared normalization is required
4. implement one rule at a time with its sidecar tests
5. run family-local tests after each rule or small batch
6. only after the planned rules are in, revisit any remaining stabilization items that still block a clean “done” call

## Done Means

This lane is not done until:

- `RS-CODE-31`, `33`, `34`, `35`, and `36` are implemented
- each new rule has a production file and a rule-specific `*_tests/` directory
- family tests are green
- `code.md` no longer marks those rules planned
- any still-open stabilization debt is either fixed or written down precisely
