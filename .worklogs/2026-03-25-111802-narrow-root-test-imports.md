# Narrow Root Test Imports

**Date:** 2026-03-25 11:18
**Scope:** `apps/guardrail3/tests/unit/allow_checks_test.rs`, `apps/guardrail3/tests/unit/code_quality_checks_test.rs`, `apps/guardrail3/tests/unit/rs_structure_checks_test.rs`, `apps/guardrail3/tests/unit/rs_test_quality_checks_test.rs`, `apps/guardrail3/tests/unit/test_garde_checks.rs`, `apps/guardrail3/tests/unit/test_support/assertions.rs`

## Summary
Reduced root-facade dependence inside a small set of still-root-owned Rust tests by replacing report/fs/AST-helper imports with direct crate imports where clean owner crates already exist. This does not finish the root-harness problem, but it keeps the remaining root tests from deepening the facade debt while the larger clusters are still being dismantled.

## Context & Problem
The workspace split plan is not only about promoting crates; it also requires internal code and tests to stop depending on `guardrail3::...` once real owners exist. After the AST/report/release reductions, several root tests were still using the root facade for things that already have explicit crate owners:
- report model types
- real filesystem adapter
- Rust AST helpers

Those imports were easy to narrow without touching already-dirty files or dragging in a larger legacy-validator migration.

## Decisions Made

### Narrow imports only in still-clean root Rust tests
- **Chose:** Update only the root test files that were clean in the worktree and whose changes were purely import-path narrowing.
- **Why:** The worktree already contains unrelated edits in more complex files like `dependency_allowlist_test.rs`, `rs_test_checks_test.rs`, and `test_hex_arch_checks.rs`. Narrowing the clean files avoids merge risk while still making measurable progress.
- **Alternatives considered:**
  - Sweep all remaining root tests in one pass — rejected because it would overlap with already-dirty files and increase the chance of staging unrelated work.
  - Skip this and only delete bigger clusters — rejected because the plan also calls for removing transitional facade usage as crates become real.

### Use direct crate owners for report, fs, and AST helpers
- **Chose:** Switch to `guardrail3_domain_report`, `guardrail3_adapters_outbound_fs`, and `guardrail3_app_rs_ast` where applicable.
- **Why:** Those crates are already promoted and stable owners. Root tests that still exist should depend on the narrowest real crate they need.
- **Alternatives considered:**
  - Keep `guardrail3::domain::report` / `guardrail3::adapters::outbound::fs` / `guardrail3::app::rs::validate::ast_helpers` for convenience — rejected because it preserves exactly the compatibility path the split is trying to retire.

## Architectural Notes
This batch is intentionally small and low-risk. It does not claim a performance win by itself. The value is architectural hygiene:
- fewer root-facade imports survive in root tests
- direct crate owners become the default dependency path
- later test moves/deletions have less compatibility baggage to unwind

The remaining real bottleneck is still the root `tests/unit.rs` target itself, which pulls in unrelated heavy crates like `hooks-rs` during compilation.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — split plan and root-facade debt direction
- `apps/guardrail3/tests/unit.rs` — current root harness
- `apps/guardrail3/tests/unit/allow_checks_test.rs`
- `apps/guardrail3/tests/unit/code_quality_checks_test.rs`
- `apps/guardrail3/tests/unit/rs_structure_checks_test.rs`
- `apps/guardrail3/tests/unit/rs_test_quality_checks_test.rs`
- `apps/guardrail3/tests/unit/test_garde_checks.rs`
- `apps/guardrail3/tests/unit/test_support/assertions.rs`
- `apps/guardrail3/crates/domain/report/mod.rs` — report owner
- `apps/guardrail3/crates/adapters/outbound/fs/mod.rs` and crate manifest — fs adapter owner
- `apps/guardrail3/crates/app/rs/ast/src/lib.rs` — AST helper owner

## Open Questions / Future Considerations
- The biggest remaining root Rust debt is not these imports; it is the still-root-owned legacy test clusters and the `tests/unit.rs` aggregator itself.
- Some remaining root tests already have unrelated worktree changes, so the next sweep needs to avoid trampling those edits.
- `rs_arch_01` is still a large legacy fixture suite and should not be deleted casually just because the new `arch` family exists; it needs an explicit migration or retirement decision.

## Key Files for Context
- `apps/guardrail3/tests/unit.rs` — remaining root harness inventory
- `apps/guardrail3/tests/unit/allow_checks_test.rs` — example of narrowed report import
- `apps/guardrail3/tests/unit/rs_test_quality_checks_test.rs` — example of narrowed fs import
- `apps/guardrail3/tests/unit/test_garde_checks.rs` — example of narrowed AST-helper import
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — authoritative split plan
- `.worklogs/2026-03-25-110905-move-ast-and-report-tests-off-root.md`
- `.worklogs/2026-03-25-111118-drop-root-release-test-cluster.md`

## Next Steps / Continuation Plan
1. Re-scan `apps/guardrail3/tests/unit.rs` and separate remaining entries into:
   - deletable/redundant because owner crates already have stronger tests
   - movable to owner crates
   - true legacy debt that still blocks on validator migration
2. Avoid already-dirty root test files unless their existing edits are first understood and intentionally incorporated.
3. Keep deleting or relocating whole root clusters when owner-crate proof already exists, since that yields larger wins than more import-only batches.
4. Continue using owner-crate test targets plus `cargo check --workspace --lib` as the main verification loop; root `--test unit` remains a measurement of residual bottleneck, not a fast confidence loop.
