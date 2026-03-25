# Fix per-root Rust applicability and split shared AST ownership

**Date:** 2026-03-25 00:40
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/Cargo.lock`, `apps/guardrail3/crates/app/ts/{Cargo.toml,mod.rs}`, `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/app/rs/ast/**`, `apps/guardrail3/crates/app/rs/checks/rs/code/parse.rs`, `apps/guardrail3/crates/app/rs/checks/rs/test/**`, `apps/guardrail3/crates/app/rs/families/{code,test,hooks-rs}/**`, `apps/guardrail3/crates/app/rs/validate/{ast_helpers.rs,ast_visitors.rs,extra_visitors.rs}`, `apps/guardrail3/crates/lib.rs`

## Summary
Fixed the new Rust runtime so family enablement is no longer collapsed to a single repo-wide boolean at report time. Extracted the shared Rust AST helper substrate into a real `app/rs/ast` crate and rewired the promoted `code` and `test` family crates to depend on that owner directly instead of faking `app::rs::validate`. The commit also carries the still-pending `app/ts` workspace-member declaration already sitting in `apps/guardrail3/Cargo.toml` / `Cargo.lock`.

## Context & Problem
The workspace split had produced real family crates, but two structural issues remained acute. First, `app/rs/runtime` selected families using mixed-scope config but then ran each family once across the whole `ProjectTree`, so one enabled root could cause reports against another disabled root. Second, the `code` and `test` family crates still depended on legacy validator ownership because their parsing layer reached through `app::rs::validate::{ast_helpers,ast_visitors,extra_visitors}` via shim modules.

The user asked to commit and keep moving the split, and the review explicitly called out typed per-root applicability plus severing `app/rs/validate` backedges as the next real blockers.

`apps/guardrail3/Cargo.toml` and `Cargo.lock` also still had the uncommitted `app/ts` workspace-member promotion from the prior split pass. Because the AST cut needed those files anyway, this batch records that pending member wiring rather than trying to keep a partial file split out of history.

## Decisions Made

### Filter family results by resolved scope in the runtime
- **Chose:** Keep family orchestrators unchanged for now and apply per-root applicability as a post-family filter in `app/rs/runtime`.
- **Why:** This addresses the actual misfire without requiring a full redesign of every family’s orchestrator input model in the same batch.
- **Alternatives considered:**
  - Rewrite each family to run separately per configured root — rejected because it would explode the cut size and stall the split.
  - Leave applicability as repo-global until every result has perfect ownership metadata — rejected because the current behavior is observably wrong for mixed-root configs.

### Anchor per-root `rs/test` coverage results to root paths
- **Chose:** Add stable root-path anchors for `RS-TEST-04`, `05`, and `06`.
- **Why:** The runtime filter can only suppress results for disabled roots if those results carry a usable scope anchor.
- **Alternatives considered:**
  - Treat all rootless results as disabled unless global is enabled — rejected because tool-level and truly repo-global results would be incorrectly dropped.
  - Delay the runtime fix until every family result is fully scoped — rejected because the runtime bug should be narrowed immediately.

### Move AST ownership into a real shared crate
- **Chose:** Create `crates/app/rs/ast` as the real owner of `ast_helpers`, `ast_visitors`, and `extra_visitors`.
- **Why:** `code` and `test` only still depended on `validate` because the AST substrate had never been given an actual home.
- **Alternatives considered:**
  - Keep the files in `validate` and continue exposing them through crate-local shims — rejected because it preserves the wrong owner and invites new backedges.
  - Duplicate the AST helpers into each family crate — rejected because it would fork semantics immediately.

### Preserve old validator callers through compatibility re-exports
- **Chose:** Turn `app/rs/validate/{ast_helpers,ast_visitors,extra_visitors}.rs` into thin re-export stubs over the new AST crate.
- **Why:** Old validator code and root tests still reference those paths. Re-export stubs let the split proceed without breaking the whole legacy surface in the same commit.
- **Alternatives considered:**
  - Rewrite all legacy validator callers in one pass — rejected because it would be a much larger and less reviewable batch.
  - Leave the old implementations in place and dual-own the code — rejected because it would preserve the exact ownership ambiguity this split is meant to remove.

### Raise the hooks-rs recursion limit as compile fallout containment
- **Chose:** Increase the `hooks-rs` crate recursion limit.
- **Why:** The runtime/lib test target was hitting syn/type recursion overflow while compiling hook-family dependencies after the split.
- **Alternatives considered:**
  - Ignore the overflow and stop testing the runtime crate entirely — rejected because it blocks even targeted validation work.
  - Refactor hook shell parsing in the same batch — rejected because that is separate debt.

## Architectural Notes
- `app/rs/runtime` now owns a typed applicability layer:
  - `RustFamilyApplicability`
  - per-app flags
  - package-scope flag
  - global-only family bypass
- This is still a transitional model. It filters emitted results rather than changing family execution granularity.
- `app/rs/ast` is now the real shared owner for Rust AST analysis.
- `code` and `test` family crates no longer have any live `app::rs::validate` / `rs_validate` references in their family-crate shim layer or parse paths.
- Legacy validator modules still exist, but the AST implementation is no longer owned there.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-03-25-000414-workspace-split-crate-promotion.md`
- reviewer findings pasted in-session about per-root applicability and legacy `validate` backedges
- local code inspection of:
  - `apps/guardrail3/crates/app/rs/runtime.rs`
  - `apps/guardrail3/crates/app/rs/checks/rs/test/*.rs`
  - `apps/guardrail3/crates/app/rs/checks/rs/code/parse.rs`
  - `apps/guardrail3/crates/app/rs/checks/rs/test/parse.rs`
  - `apps/guardrail3/crates/app/rs/validate/{ast_helpers,ast_visitors,extra_visitors}.rs`

## Open Questions / Future Considerations
- The runtime filter only works as well as result scoping. More families still emit rootless results that should eventually gain explicit anchors or explicit repo-global classification.
- CLI coverage still depends on legacy `validate` baselines for clippy/deny.
- Root tests still target the root facade and legacy validator heavily, so the split has not yet delivered full test-target isolation.
- `hooks-rs` compile cost is still too high for a good fast loop in the runtime crate.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/runtime.rs` — current per-family applicability and result filtering
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — focused runtime applicability tests
- `apps/guardrail3/crates/app/rs/ast/src/lib.rs` — new AST crate root
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs` — real owner of shared Rust AST helpers
- `apps/guardrail3/crates/app/rs/checks/rs/code/parse.rs` — code family parse path now using `guardrail3_app_rs_ast`
- `apps/guardrail3/crates/app/rs/checks/rs/test/parse.rs` — test family parse path now using `guardrail3_app_rs_ast`
- `apps/guardrail3/crates/app/rs/families/code/src/lib.rs` — no more fake `rs_validate` shim
- `apps/guardrail3/crates/app/rs/families/test/src/lib.rs` — no more fake `rs_validate` shim
- `apps/guardrail3/crates/app/rs/validate/ast_helpers.rs` — compatibility re-export for legacy callers
- `.worklogs/2026-03-25-000414-workspace-split-crate-promotion.md` — prior crate-promotion context

## Next Steps / Continuation Plan
1. Move clippy and deny coverage baselines out of `app/rs/validate` so inbound CLI coverage no longer depends on legacy validator ownership.
2. Audit remaining families for rootless results that should be root-scoped, starting with `test`, `deps`, and `release`.
3. Reduce the runtime/lib test compile bottleneck by isolating or slimming the hooks-family dependency surface used by `guardrail3-app-rs-runtime`.
