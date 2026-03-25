# Move Help Tests To CLI Crate

**Date:** 2026-03-25 10:43
**Scope:** `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs`, `apps/guardrail3/crates/adapters/inbound/cli/help_gen_tests.rs`, `apps/guardrail3/tests/unit.rs`, `apps/guardrail3/tests/unit/help_gen_test.rs`

## Summary
Moved the CLI help tests out of the root `tests/unit.rs` harness and into the promoted inbound CLI crate. While doing that, corrected one stale assertion that expected `PROFILES` in top-level help even though that text actually belongs to `rs init`.

## Context & Problem
After promoting `adapters/inbound/cli` into a real crate, the root test harness still contained a CLI-owned unit test: `tests/unit/help_gen_test.rs`. That kept one more facade-oriented test on the root path even though the promoted CLI crate already owned the code under test. The point of the workspace split is not just code ownership, but test ownership too.

## Decisions Made

### Move `help_gen_test` into the CLI crate
- **Chose:** Add `help_gen_tests.rs` next to `help_gen.rs` and include it from the crate-local module via `#[cfg(test)]`.
- **Why:** This keeps the test attached to the real owner and removes one more root-harness dependency on the CLI facade.
- **Alternatives considered:**
  - Leave the test in `tests/unit.rs` for convenience — rejected because it preserves root coupling for no real benefit.
  - Convert it into a binary/integration CLI test — rejected because this is still a pure help-injection unit test, not a product-entry shell test.

### Fix the stale `PROFILES` assertion instead of preserving it
- **Chose:** Move the `PROFILES` assertion onto a dedicated `rs init` help test.
- **Why:** Once the test ran inside the CLI crate, it exposed that `PROFILES` is attached to `rs init` help, not top-level after-help. The old assertion was simply testing the wrong command surface.
- **Alternatives considered:**
  - Keep the old assertion and treat the failure as a regression — rejected because the actual help wiring already clearly places `PROFILES` under `rs init`.
  - Drop the assertion entirely — rejected because the `rs init` help text is still worth testing; it just belongs on the correct subcommand.

## Architectural Notes
This is a small but important test-topology change:
- the CLI crate now owns its own help tests
- the root `unit.rs` harness is slightly smaller
- the crate split starts to pay off in test placement, not just manifests and imports

This also validates the promoted CLI crate as a real testing surface rather than only a compile-time wrapper.

## Information Sources
- `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs` — owner of the tested help injection logic
- `apps/guardrail3/tests/unit/help_gen_test.rs` — previous root-harness location
- `.worklogs/2026-03-25-104020-inbound-cli-crate-promotion.md` — prior CLI crate promotion

## Open Questions / Future Considerations
- The root harness still includes many non-CLI tests that belong on narrower crates.
- More CLI-owned tests can likely move out of `tests/unit.rs` once the next small slices are identified.

## Key Files for Context
- `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs` — production help-injection code with crate-local test inclusion
- `apps/guardrail3/crates/adapters/inbound/cli/help_gen_tests.rs` — new crate-local help tests
- `apps/guardrail3/tests/unit.rs` — root harness after removing the CLI help test
- `.worklogs/2026-03-25-104020-inbound-cli-crate-promotion.md` — prerequisite CLI crate promotion

## Next Steps / Continuation Plan
1. Keep scanning `tests/unit.rs` for other tests that now clearly belong to promoted crates.
2. Prefer moving small, self-contained root tests like this first so the harness shrinks without broad refactor churn.
