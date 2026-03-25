# Thin The App-Hooks Crate API

**Date:** 2026-03-25 03:05
**Scope:** `apps/guardrail3/crates/app/hooks/mod.rs`, `apps/guardrail3/crates/app/hooks/validate.rs`, `apps/guardrail3/crates/app/hooks/hook_checks.rs`, `apps/guardrail3/crates/app/hooks/hook_script_checks.rs`, `apps/guardrail3/crates/app/hooks/deploy_checks.rs`, `apps/guardrail3/crates/app/hooks/tool_checks.rs`

## Summary
Reduced `guardrail3-app-hooks` from a nested compatibility wrapper to a direct crate API. The crate now exposes a top-level `check(...)` function and uses direct crate imports internally instead of faking `domain` and `app::rs::checks::hooks` subtrees.

## Context & Problem
After the shell-family cutover, `app-hooks` was still carrying the exact compatibility shape the workspace split plan warns against:
- fake `domain` reexports
- fake `app::rs::checks::hooks` nesting
- internal files depending on those fake module paths

That wrapper no longer served a real ownership purpose because the actual hook families already lived in:
- `guardrail3-app-rs-family-hooks-shared`
- `guardrail3-app-rs-family-hooks-rs`

Leaving `app-hooks` in that state would keep inviting new code to depend on a compatibility fiction instead of the real crate boundaries.

## Decisions Made

### Replace the nested shim with one real top-level API
- **Chose:** expose `pub fn check(...) -> Vec<CheckResult>` directly from `app/hooks/mod.rs`.
- **Why:** the crate’s real job is composing shared + Rust hook family results. A single top-level function matches that responsibility without preserving the old fake tree.
- **Alternatives considered:**
  - Keep `app::rs::checks::hooks::check` for compatibility — rejected because this crate is already a real workspace member and should not keep re-inventing nested monolith paths.
  - Re-export the family crates directly and make callers compose them — rejected because `app-hooks` still has a legitimate aggregation role.

### Move the whole crate onto direct crate imports
- **Chose:** update `validate.rs` and the helper files to import `guardrail3_app_core` and `guardrail3_domain_report` directly.
- **Why:** once the nested wrapper is removed, all internal code needs to align with real owners or the crate immediately stops compiling.
- **Alternatives considered:**
  - Keep some local compatibility reexports only for `validate.rs` — rejected because that would just recreate the same wrapper debt at a smaller scale.

## Architectural Notes
- `app-hooks` is now a thin integration crate instead of a nested pseudo-namespace.
- The live composition path is explicit:
  - `guardrail3_app_rs_family_hooks_shared::check(...)`
  - `guardrail3_app_rs_family_hooks_rs::check(...)`
  - merged by `guardrail3_app_hooks::check(...)`
- `validate.rs` now calls the local crate API plus direct `guardrail3_app_core::project_walker::walk_project`.

## Information Sources
- `.worklogs/2026-03-25-030222-shell-family-cutover.md` — prior shell-family cutover that made this thinning possible.
- `apps/guardrail3/crates/app/hooks/mod.rs` — pre-change nested compatibility wrapper.
- `apps/guardrail3/crates/app/hooks/validate.rs` — main live caller that needed rewiring.
- `apps/guardrail3/crates/app/core/mod.rs` — confirmation that `crawl` and `project_walker` are already exported directly from `guardrail3-app-core`.

## Open Questions / Future Considerations
- `app-hooks` is thinner now, but the broader root facade is still too wide.
- The old `app/rs/validate/mod.rs` compatibility path still references the old nested hook-check path and should eventually be rewritten or retired with the rest of the legacy validator.

## Key Files for Context
- `apps/guardrail3/crates/app/hooks/mod.rs` — current thin `app-hooks` crate surface.
- `apps/guardrail3/crates/app/hooks/validate.rs` — hook validation entrypoint now using direct crate owners.
- `.worklogs/2026-03-25-030222-shell-family-cutover.md` — prior shell-family cutover that handed off into this thinning pass.

## Next Steps / Continuation Plan
1. Commit this `app-hooks` thinning pass by staging only:
   - `apps/guardrail3/crates/app/hooks/**`
   - this worklog
2. Re-sync to the workspace split plan and choose the next thin-facade target, likely:
   - `app/commands`
   - `app/rs/generate`
   - further root-facade reduction
3. Keep deleting compatibility-only nested module shapes as soon as their real crate owners are already in place.
