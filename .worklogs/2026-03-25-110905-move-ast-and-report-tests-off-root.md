# Move AST And Report Tests Off Root

**Date:** 2026-03-25 11:09
**Scope:** `apps/guardrail3/crates/app/rs/ast/src/*`, `apps/guardrail3/crates/domain/report/*`, `apps/guardrail3/tests/unit.rs`

## Summary
Moved the remaining pure AST helper tests and the report severity-count tests out of the root `tests/unit.rs` harness and into their actual owner crates. This continues Phase 2/step-15 of the workspace split plan by shrinking the root test aggregator instead of leaving narrow crate tests behind the monolithic root package.

## Context & Problem
The current workspace split plan explicitly calls out root-test decomposition as a parallel requirement, not optional cleanup after crate promotion. Even after promoting many real crates, `apps/guardrail3/tests/unit.rs` was still keeping simple owner-aligned tests on the root package hot path, which undermines the point of the split.

Three tests were especially clean candidates:
- `ast_helpers_test.rs`
- `ast_visitors_test.rs`
- `report_test.rs`

Those tests were already exercising code whose real owners had been promoted:
- `guardrail3-app-rs-ast`
- `guardrail3-domain-report`

Leaving them in the root harness provided no architectural value and kept the root package wider than necessary.

## Decisions Made

### Move AST helper coverage into the AST crate
- **Chose:** Add crate-local sidecar test files under `apps/guardrail3/crates/app/rs/ast/src/` and wire them from the owning production modules.
- **Why:** The AST helpers and extra visitors already live in the dedicated AST crate. Their tests should compile there directly, not through `guardrail3::app::rs::validate::*` re-exports from the root package.
- **Alternatives considered:**
  - Keep the tests at root but switch imports to direct crates — rejected because that still leaves `tests/unit.rs` carrying owner-local tests.
  - Create integration tests under the AST crate — rejected because these are white-box helper tests that fit better as close sidecars on the owned modules.

### Move report counting tests into the report crate
- **Chose:** Add `report_tests.rs` under `apps/guardrail3/crates/domain/report/` and wire it from `mod.rs`.
- **Why:** The report counting logic is fully owned by `guardrail3-domain-report`. Root-level placement was pure historical residue.
- **Alternatives considered:**
  - Leave the root test in place — rejected because it contradicts the plan’s requirement to reduce the root aggregator before claiming crate-split wins.
  - Convert it to a root integration test — rejected because there is no product-surface behavior here, just domain logic.

### Reduce the root harness immediately instead of batching deletions later
- **Chose:** Remove the corresponding entries from `apps/guardrail3/tests/unit.rs` and delete the old root test files in the same batch.
- **Why:** This keeps the architectural move honest. New owner-crate tests are only useful if the root harness actually gets smaller.
- **Alternatives considered:**
  - Temporarily duplicate tests in both places — rejected because it preserves the bottleneck and complicates future cleanup.

## Architectural Notes
This batch is small, but it follows the split plan correctly:
- owner-local unit tests now live with their real owner crate
- the root `tests/unit.rs` aggregator is smaller than before
- the root facade is slightly less attractive as a default dependency path

This is the same pattern already used for CLI help tests and `app-core` tests earlier in the split. The remaining root harness is now more concentrated in true legacy-validator/test-debt areas rather than obvious owner-local helpers.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — especially Phase 2 and step 15 emphasis on dismantling `tests/unit.rs`
- `apps/guardrail3/tests/unit.rs` — current root harness inventory
- `apps/guardrail3/tests/unit/ast_helpers_test.rs` — old root AST helper tests
- `apps/guardrail3/tests/unit/ast_visitors_test.rs` — old root extra visitor test
- `apps/guardrail3/tests/unit/report_test.rs` — old root report-count test
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs` — actual AST helper owner
- `apps/guardrail3/crates/app/rs/ast/src/extra_visitors.rs` — actual extra visitor owner
- `apps/guardrail3/crates/domain/report/mod.rs` — actual report model owner
- `.worklogs/2026-03-25-104338-move-help-tests-to-cli-crate.md`
- `.worklogs/2026-03-25-104900-move-app-core-tests-off-root.md`

## Open Questions / Future Considerations
- The remaining root harness entries are now mostly legacy Rust validator tests, TS legacy tests, and arch fixture suites. Those need cluster-level decisions rather than the simple owner moves used here.
- The next meaningful sweep should probably attack the release/hexarch/test legacy clusters or convert remaining root imports from `guardrail3::` to direct crate paths where owner crates already exist.
- `tests/unit/test_support/assertions.rs` still depends on the root facade for report types; that should eventually move to direct crate imports even before the harness is fully dismantled.

## Key Files for Context
- `apps/guardrail3/tests/unit.rs` — current root unit-test aggregator and remaining debt inventory
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs` — AST helper owner with new local test wiring
- `apps/guardrail3/crates/app/rs/ast/src/extra_visitors.rs` — extra visitor owner with new local test wiring
- `apps/guardrail3/crates/domain/report/mod.rs` — report owner with new local test wiring
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — authoritative split plan
- `.worklogs/2026-03-25-104338-move-help-tests-to-cli-crate.md` — prior root-test shrink pattern for CLI
- `.worklogs/2026-03-25-104900-move-app-core-tests-off-root.md` — prior root-test shrink pattern for app-core

## Next Steps / Continuation Plan
1. Re-scan `apps/guardrail3/tests/unit.rs` and group the remaining entries by real owner crate instead of by file name.
2. Take the next owner-aligned sweep from the obvious Rust buckets, preferably a cluster that can move together without touching TS scope.
3. For remaining root tests that cannot move yet, at least replace `guardrail3::domain::report` / `guardrail3::adapters::outbound::fs` imports with direct crate imports to keep shrinking facade reliance.
4. Keep verifying narrow crate targets first, then `cargo check --manifest-path apps/guardrail3/Cargo.toml --workspace --lib`, and only then commit another batch with its own worklog.
