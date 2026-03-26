# Cargo Test-Support Boundary Cleanup

**Date:** 2026-03-26 18:45
**Scope:** `apps/guardrail3/crates/app/rs/families/cargo/**`

## Summary
Removed the semantic lint fixture API from cargo family `test_support`, inlined the TOML fixture bodies into the rule test modules themselves, and deleted the now-dead rule-module fixture helpers. The cargo family now keeps `test_support` generic while preserving test coverage and passing the stricter `RS-TEST-18` boundary.

## Context & Problem
`RS-TEST-18` was meant to force `test_support` to stay generic, but the cargo family still exposed semantic TOML bodies via `LintFixture`/`lint_fixture(...)` and later via helper constants/functions. That made `test_support` part of the rule semantics instead of a shared generic fixture layer. The goal was to move those semantic fixtures down into the rule-local test modules without breaking the existing cargo checks.

## Decisions Made

### Keep production runtime helpers, remove fixture bodies
- **Chose:** Preserve `crates/runtime/src/lint_support.rs` for the real production lint helper functions and expectation tables, but remove the semantic TOML fixture bodies from the cargo test-support surface.
- **Why:** The helper functions are used by runtime rule logic; the TOML bodies are only test fixtures and belong next to the tests that consume them.
- **Alternatives considered:**
  - Delete `lint_support.rs` entirely — rejected because production rules still import its helper functions.
  - Keep fixture TOML in a sibling runtime module — rejected because it reintroduced the same semantic boundary leak that `RS-TEST-18` was supposed to close.

### Inline fixture bodies in rule-local test modules
- **Chose:** Put the TOML bodies as local constants in each `rs_cargo_*_tests/cases.rs`.
- **Why:** This keeps the semantic fixture data adjacent to the rule scenarios that need it and avoids a shared semantic fixture API.
- **Alternatives considered:**
  - Centralize fixtures in a separate helper module under runtime — rejected because that still made the runtime layer a semantic fixture host.
  - Keep the TOML bodies in `test_support` — rejected because it violated the intended boundary.

### Remove dead rule-module fixture helpers
- **Chose:** Delete the old fixture constant/function block from each rule test `mod.rs` once the cases owned their local constants.
- **Why:** The helper block became dead code and would otherwise keep a second semantic fixture surface alive.
- **Alternatives considered:**
  - Leave the helpers and silence dead-code warnings — rejected because that would preserve the extra API surface the boundary fix was trying to eliminate.

## Architectural Notes
This slice makes the cargo family’s test architecture clearer:
- `test_support` now holds only generic tree/entry helpers.
- Rule fixtures live locally in the rule test cases.
- Production runtime helpers remain in runtime, but they no longer carry semantic fixture TOML.

That separation better matches the stricter `RS-TEST-18` expectation and keeps the family aligned with the broader “generic support vs. semantic assertions” split used elsewhere.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/cargo/test_support/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lint_support.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_*_tests/cases.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_*_tests/mod.rs`
- Previous RS family worklogs in `.worklogs/` describing the `RS-TEST-18` boundary tightening

## Open Questions / Future Considerations
- The cargo family still carries a lot of repetitive fixture text across rule tests. If that becomes painful, the next step should be a rule-local fixture builder API that remains private to each test module, not a shared semantic `test_support` layer.
- Similar cleanup may be needed for other families once they are brought under the stricter `RS-TEST-18` interpretation.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/cargo/test_support/src/lib.rs` — generic-only tree and entry helpers.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lib.rs` — runtime module wiring for the cargo family.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lint_support.rs` — production lint helper functions and expectation tables.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_01_workspace_lints_tests/cases.rs` — example of inlined local fixture bodies.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_01_workspace_lints_tests/mod.rs` — example of the removed dead helper surface.

## Next Steps / Continuation Plan
1. Stage only the cargo-family files touched in this slice.
2. Commit the worklog and code together as one checkpoint.
3. If the cargo family later needs another boundary pass, keep any new semantic fixtures local to the relevant rule test module and avoid reintroducing shared fixture helpers.
