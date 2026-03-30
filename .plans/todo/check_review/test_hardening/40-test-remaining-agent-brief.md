# Test Remaining Agent Brief

You own the remaining `rs/test` work.

The family is implemented, but the lane is not fully closed. The remaining work is hardening and closure work, not fresh rule-inventory expansion.

## Read First

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/checks/rs/test.md`
4. `.plans/todo/check_review/test_hardening/26-test-agent-brief.md`
5. `.plans/todo/check_review/test_hardening/34-test-family-rewrite-agent-brief.md`
6. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
7. `apps/guardrail3/crates/app/rs/families/test/README.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/`
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/`
- `apps/guardrail3/crates/app/rs/families/test/test_support/src/`

## Family Status At Handoff

All `RS-TEST-*` rules are marked implemented in `.plans/todo/checks/rs/test.md`.

However, the lane still has open family-level closure work:

- the plan still references remaining migration/hardening work
- this lane should not yet be treated as fully closed

This is a “finish and prove it” lane, not a “write new planned rules” lane.

## Mission

Close the remaining `rs/test` hardening and family-closure debt.

Required outcomes:

1. verify that every implemented `RS-TEST-*` rule matches the current family README contract
2. close any remaining structural test debt
3. add or tighten attack-vector coverage where the family is still weak
4. fix any real semantic bugs found during that pass
5. update `.plans/todo/checks/rs/test.md` so it no longer carries stale “still in migration” language if the work is actually complete

## Constraints

- Stay inside `rs/test`
- Do not re-own generic hook structure in `RS-TEST-08`
- Do not invent new rule IDs unless the plan is explicitly expanded
- Keep one rule per file and rule-specific sidecar tests

## Highest-Value Targets

1. exact multi-root ownership behavior
2. fail-closed handling through the family input-failure rule
3. hook-surface semantics for `RS-TEST-08`
4. heuristic false-positive control in assertion-quality rules
5. plan/code/README convergence

## Suggested Execution Order

1. compare `test.md` against the family README and the live file layout
2. identify which “remaining migration/hardening” statements are still true vs stale
3. attack the weakest implemented rules first:
   - `RS-TEST-08`
   - `RS-TEST-13`
   - `RS-TEST-15`
   - `RS-TEST-17`
   - `RS-TEST-19`
4. fix real bugs first, then docs
5. finish by making the plan language accurately reflect the post-pass state

## Done Means

This lane is not done until:

- the family has no material remaining hardening gap that keeps it out of the “done” bucket
- family tests are green
- any real semantic bugs found are fixed or explicitly documented
- `test.md` accurately reflects the live status instead of carrying stale migration language
