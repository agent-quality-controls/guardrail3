# Use Direct Crates In Legacy Validate Entry

**Date:** 2026-03-25 11:51
**Scope:** `apps/guardrail3/crates/app/rs/validate/mod.rs`

## Summary
Rewired the live entrypoint of the legacy Rust validator crate to use direct crate owners for core crawling, hook orchestration, config, and report types. This keeps the compatibility shims available for older submodules but stops the main orchestrator path from depending on fake `crate::app` / `crate::domain` ownership.

## Context & Problem
`guardrail3-app-rs-legacy-validate` is now a real crate, but its entrypoint still imported `crate::app::core`, `crate::app::hooks`, `crate::domain::config`, and `crate::domain::report` through local shim modules added during promotion. Those shims are still useful for broad legacy compatibility, but the main orchestrator path itself no longer needs to pretend those owners are local.

This is a small but important follow-up: once a compatibility island becomes a crate, the live top-level paths should start depending on real owners directly so the shims shrink to true compatibility scope.

## Decisions Made

### Use direct crate imports in `validate/mod.rs`
- **Chose:** Replaced top-level imports and hook/project-walker calls in `app/rs/validate/mod.rs` with direct crate paths.
- **Why:** The orchestrator entrypoint is live code. It should reflect the actual owner graph now that those owners are real crates.
- **Alternatives considered:**
  - Leave the shim-based imports in place — rejected because it keeps fake local ownership in the most important entrypoint.
  - Rewrite every legacy file in the crate at once — rejected because that is much broader than this cleanup and would collide with unrelated dirty worktree state.

### Keep shim modules for legacy submodules
- **Chose:** Left the local `app` and `domain` shim modules in place.
- **Why:** Many older submodules still reference them, and removing them now would create unnecessary churn. The value here is shrinking live shim usage, not forcing a whole-crate rewrite in one pass.
- **Alternatives considered:**
  - Delete the shims now — rejected because too many legacy modules still rely on them.

## Architectural Notes
This is an ownership cleanup inside the already-promoted legacy validate crate, not a new crate split. It pushes the codebase toward the plan’s end state where compatibility glue is thin and live code uses real crate boundaries directly.

## Information Sources
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — live legacy validator orchestrator.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — crate-promotion record for the legacy validator.
- `.worklogs/2026-03-25-114827-collapse-root-hook-facade.md` — latest hook-side facade cleanup showing the same pattern of collapsing wrappers onto real owners.

## Open Questions / Future Considerations
- There are many more legacy submodules in `app/rs/validate` that still use shim namespaces. They can be cleaned incrementally, but only when touching those files is worth the churn.
- The bigger remaining win is still routing more live callers off `app/rs/validate` entirely and onto runtime/family crates.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy validator entrypoint after direct owner imports.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — legacy crate promotion.
- `.worklogs/2026-03-25-114827-collapse-root-hook-facade.md` — recent adjacent facade cleanup.

## Next Steps / Continuation Plan
1. Keep identifying live compatibility entrypoints that can use direct owner crates without rewriting whole legacy trees.
2. Avoid touching heavily dirty legacy submodules unless the ownership improvement is worth the merge risk.
3. Continue prioritizing crate-boundary truth over test relayering or broad cosmetic cleanup.
