# Continue RS-CODE Allow Removal

**Date:** 2026-03-29 21:50
**Scope:** `apps/guardrail3/crates/adapters/inbound/cli`, `apps/guardrail3/crates/adapters/outbound/report`, `apps/guardrail3/crates/app/core/discover_tests.rs`, `apps/guardrail3/crates/app/rs/validate/mod.rs`, `apps/guardrail3/crates/bin/guardrail3/src/main.rs`

## Summary
Continued the `RS-CODE-04` cleanup by removing or collapsing more avoidable `#[allow(...)]` sites in CLI helpers, report renderers, legacy validation orchestration, and core tests. The repo-root `RS-CODE-04` inventory dropped again from `193` to `173` without weakening checker behavior.

## Context & Problem
After the previous allow-cleanup slice, the remaining `RS-CODE-04` inventory had shifted from dead test scaffolding into a mix of:
- true CLI boundary output/exit allowances
- repeated helper-level exceptions that could be collapsed into a single real boundary
- legacy orchestration code still carrying “Phase 2 later” excuses
- test setup code still using convenience allows instead of cleaner helper structure

The goal for this pass was to keep targeting real removal and structural consolidation, not reason-text cleanup.

## Decisions Made

### Make helper layers pure and move side effects back to real CLI boundaries
- **Chose:** Make `generate.rs::load_config` return `Result<Option<...>, String>` and update callers, and make `diff.rs::show_smart_diff` return a boolean instead of exiting internally.
- **Why:** These helpers were carrying local `print_*` / `disallowed_methods` or exit-related debt even though the side effects belong at the outer command layer.
- **Alternatives considered:**
  - Leave the helpers impure and keep the existing allows — rejected because the behavior was structurally removable.
  - Push everything into a larger command object first — rejected as unnecessary for this slice.

### Collapse repeated git-command allowances into one real boundary
- **Chose:** Add a single `run_git_command()` helper in `cli/validate.rs` and route staged/dirty/commit file discovery through it.
- **Why:** Four separate `disallowed_methods` allowances for the same exact git subprocess pattern were unnecessary. One helper is the actual boundary.
- **Alternatives considered:**
  - Keep separate helpers with separate allows — rejected because it preserved duplicated exception surface.
  - Replace git subprocesses entirely in this pass — rejected because it would be a broader behavior change, not an allow cleanup.

### Remove convenience allows from core discovery tests
- **Chose:** Replace direct `std::fs` test setup with `guardrail3_shared_fs` helpers, remove `expect_used`, and fix the assert formatting directly.
- **Why:** These tests were a clear case of setup convenience masquerading as justified exceptions.
- **Alternatives considered:**
  - Keep the test-only allows because they are “just tests” — rejected because the user explicitly asked to remove allows wherever possible.
  - Add more local helper wrappers without removing the underlying allow — rejected because the direct cleanup was straightforward.

### Refactor legacy validate orchestration instead of keeping “Phase 2” allowances
- **Chose:** Introduce `RunInput` and `CodeCheckInput` structs in legacy `app/rs/validate/mod.rs`, remove the legacy config-loader allow, and split architecture config merging/allowlist execution into helper functions.
- **Why:** The old allowances were not principled exceptions; they were deferred cleanup in a file that is still scanned by `RS-CODE`.
- **Alternatives considered:**
  - Ignore the file because the live binary now uses the newer family runtime — rejected because `RS-CODE` still audits it and the debt was real.
  - Do a larger delete/migration of the whole legacy module — rejected because that is a separate architectural task.

## Architectural Notes
This pass reinforces the current cleanup principle:
- if the behavior is a true boundary, keep the allow at the boundary
- if the behavior is just helper plumbing, make the helper pure or collapse repeated boundaries

The main practical result is that the remaining high-count files are more honestly concentrated:
- CLI entry/boundary files (`generate.rs`, `main.rs`, `init.rs`, `map.rs`)
- centralized filesystem adapter (`shared/fs`)
- a few AST and reporting helpers that still need closer scrutiny

That is a healthier remaining surface than “random helpers and tests everywhere.”

## Information Sources
- Prior cleanup context:
  - `.worklogs/2026-03-29-213614-continue-rs-code-allow-cleanup.md`
- Live validation:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json`
- Local code review of:
  - `apps/guardrail3/crates/adapters/inbound/cli/diff.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/generate.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/init.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/map.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/validate.rs`
  - `apps/guardrail3/crates/adapters/outbound/report/text.rs`
  - `apps/guardrail3/crates/adapters/outbound/report/markdown.rs`
  - `apps/guardrail3/crates/app/core/discover_tests.rs`
  - `apps/guardrail3/crates/app/rs/validate/mod.rs`
  - `apps/guardrail3/crates/bin/guardrail3/src/main.rs`
- Targeted subagent audits:
  - `generate.rs` / `main.rs` removable allow candidates
  - `report/text.rs` / `report/markdown.rs` type-complexity cleanup

## Open Questions / Future Considerations
- The remaining biggest `RS-CODE-04` piles are now mostly true CLI boundary files. Further reductions there will require broader error-return / output-routing cleanup, not just helper extraction.
- `shared/fs` is still high-count, but that may be a legitimately centralized boundary rather than removable debt.
- `app/rs/ast/src/ast_helpers.rs` still has wildcard-match allowances on large `syn` enums. Those need a deliberate judgement call instead of mechanical churn.
- There is still unrelated dirty work in the repo (`Cargo.lock`, hooks-rs, code family tests, `project-tree`, etc.). This commit intentionally excludes it.

## Key Files for Context
- `apps/guardrail3/crates/adapters/inbound/cli/generate.rs` — pure config loading pushed output back to CLI boundary
- `apps/guardrail3/crates/adapters/inbound/cli/diff.rs` — dry-run helper now returns change state instead of exiting
- `apps/guardrail3/crates/adapters/inbound/cli/validate.rs` — git subprocess boundary collapsed to one helper
- `apps/guardrail3/crates/adapters/outbound/report/text.rs` — removed local type-complexity suppression via reference simplification
- `apps/guardrail3/crates/adapters/outbound/report/markdown.rs` — same cleanup as text renderer
- `apps/guardrail3/crates/app/core/discover_tests.rs` — removed test-only setup allowances
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy orchestration now uses real context structs instead of “later” excuses
- `apps/guardrail3/crates/bin/guardrail3/src/main.rs` — kept on live runtime path while preserving helper extraction cleanup
- `.worklogs/2026-03-29-213614-continue-rs-code-allow-cleanup.md` — prior cleanup slice and bucket counts before this pass

## Next Steps / Continuation Plan
1. Continue auditing remaining `RS-CODE-04` files in descending count order, but distinguish true boundary allowances from helper debt before refactoring.
2. Start with `generate.rs`, `main.rs`, `init.rs`, and `map.rs` to see whether more inner error/exit handling can be pushed outward and consolidated.
3. Review `shared/fs` and `app/rs/ast/src/ast_helpers.rs` explicitly to decide whether those are legitimate permanent boundaries or still removable structure debt.
4. Once the remaining `RS-CODE-04` surface is mostly legitimate, shift to the next dominant real error buckets: `RS-CODE-24`, `RS-CODE-32`, and `RS-CODE-15`.
