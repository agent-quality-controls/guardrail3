# Capture RS-TEST Self-Hosting Validator Gotcha

**Date:** 2026-03-26 08:48
**Scope:** `.plans/todo/checks/rs/test.md`, `.plans/todo/validator/2026-03-26-rs-test-self-hosting-gotchas.md`, `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split.rs`, `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs`

## Summary
Documented the `RS-TEST` self-hosting gotcha in the validator plans and fixed the concrete false-negative class where root-local sidecar or external harnesses were silently skipped by `RS-TEST-03`. Added regression tests proving that ordinary unmapped harnesses now fail closed instead of disappearing from the finding set.

## Context & Problem
The user called out a real contradiction in the current validator behavior: the `rs/test` family itself did not have extracted assertions owners, but the validator was not reporting that class of issue on plain single-crate subjects. After tracing the code, the problem was not the project crawler. `ProjectTree` and family discovery could already see root-local `src/*_tests/mod.rs` and `tests/*.rs` files. The blind spot was in `RS-TEST-03` applicability: it only iterated discovered `crates/<component>/{runtime,assertions}` pairs, so ordinary single-crate roots with harnesses were silently skipped.

At the same time, the repo has a second test architecture for guardrail-family implementation roots: one production rule file plus one rule-specific sidecar test directory, with shared support in `test_support.rs`. That means the validator currently has both a false-negative class (ordinary unmapped harnesses skipped) and an unresolved self-hosting false-positive class (the checker family’s own accepted layout can be flagged as if it were ordinary app test architecture).

## Decisions Made

### Fail closed on unmapped root-local harnesses
- **Chose:** Make `RS-TEST-03` emit an explicit error when it sees root-local sidecar or external harness files that are not mapped to a discovered `runtime/assertions` component.
- **Why:** Silent skipping is the worst outcome here. If the rule sees a harness but cannot map it to a valid subject model, that is validator debt and must surface as an error.
- **Alternatives considered:**
  - Leave the silent skip in place until the full self-hosting story is designed — rejected because it preserves a known false-negative class.
  - Treat all root-local harnesses as valid — rejected because that would directly violate the `runtime/assertions` contract for ordinary application crates.

### Record the self-hosting design tension explicitly
- **Chose:** Add a validator note under `.plans/todo/validator/` and a compact gotcha note in `.plans/todo/checks/rs/test.md`.
- **Why:** The current repo has two valid test architectures, and the validator needs to distinguish them. That design tension should be recorded where future validator work will start, not left buried in code comments or chat history.
- **Alternatives considered:**
  - Only patch the code and skip the note — rejected because the next session would hit the same confusion.
  - Put the note in the family README — rejected because the user explicitly did not want the README changed, and the issue is a validator design gotcha rather than a family-rule contract change.

## Architectural Notes
- This change does not solve the full self-hosting problem for the `rs/test` family itself.
- It only closes the false-negative class for ordinary unmapped harnesses.
- The remaining work is to teach `RS-TEST-02` and `RS-TEST-03` how to recognize accepted guardrail-family implementation roots, so the validator can distinguish:
  - an ordinary crate with test harnesses that must use `runtime/assertions`
  - a checker-family implementation root with rule-specific sidecar test directories

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split.rs`
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `.plans/todo/checks/rs/test.md`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib`
- `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/test --family test --inventory --format json`
- `.worklogs/2026-03-26-081002-rs-test-family-rewrite.md`

## Open Questions / Future Considerations
- The validator still needs an explicit representation of guardrail-family implementation roots; otherwise it will continue to flag the family’s own accepted rule-test layout.
- `RS-TEST-02` currently rejects some accepted family-local sidecar wiring because its declaration matcher is too narrow for the family implementation shape.
- There is unrelated dirty repo state outside this checkpoint; it is intentionally excluded from this commit.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split.rs` — applicability and boundary logic for `RS-TEST-03`
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs` — regression tests for the unmapped-harness class
- `.plans/todo/validator/2026-03-26-rs-test-self-hosting-gotchas.md` — durable validator note for this design tension
- `.plans/todo/checks/rs/test.md` — compact gotcha reminder in the family plan
- `.worklogs/2026-03-26-081002-rs-test-family-rewrite.md` — prior family rewrite checkpoint

## Next Steps / Continuation Plan
1. Commit this validator checkpoint so the false-negative fix and gotcha note are preserved separately from the upcoming family self-hosting pass.
2. Teach `RS-TEST-02` and `RS-TEST-03` how to recognize accepted guardrail-family implementation roots and stop conflating them with ordinary application crate harnesses.
3. Re-run validation directly against `apps/guardrail3/crates/app/rs/families/test` until the family either passes or only shows findings that reflect a deliberate remaining design gap.
