# Use Direct Outbound Traits In Legacy Validators

**Date:** 2026-03-25 12:07
**Scope:** `apps/guardrail3/crates/app/rs/validate`, `apps/guardrail3/crates/app/ts/validate`, `apps/guardrail3/crates/app/core/gitignore.rs`

## Summary
Rewired the remaining live legacy validator and TS validator modules away from the old root `crate::ports::outbound` path and onto the real `guardrail3_outbound_traits` crate. This keeps the compatibility trees compiling against the actual trait owner instead of the old facade namespace.

## Context & Problem
After promoting the outbound trait surface into its own crate, several legacy Rust and TypeScript validator modules still imported `FileSystem` and `ToolChecker` through `crate::ports::outbound`. That path is exactly the kind of facade coupling the workspace split is supposed to eliminate.

Those imports were spread across many small modules, but they were all the same architectural problem:
- the code was already in decoupled crates
- the trait owner already existed
- the remaining dependency was just a stale path choice

Leaving those paths in place would preserve fake root ownership inside the biggest remaining compatibility islands.

## Decisions Made

### Switch legacy validators to `guardrail3_outbound_traits`
- **Chose:** Replaced legacy Rust validator imports of `FileSystem` and `ToolChecker` with direct imports from `guardrail3_outbound_traits`.
- **Why:** `guardrail3_outbound_traits` is the actual owner crate. The legacy validator crate should depend on the real owner even if other compatibility shims remain.
- **Alternatives considered:**
  - Leave `crate::ports::outbound` imports in place — rejected because it preserves root-facade coupling inside a crate that already exists specifically to isolate legacy code.
  - Rewrite the entire legacy validator surface to direct crate paths in one pass — rejected because the useful, low-risk improvement here is narrower than that.

### Apply the same normalization to TS validator compatibility code
- **Chose:** Replaced the same stale outbound-trait imports in `app/ts/validate`.
- **Why:** Even though TS is not the active roadmap, the TS compatibility tree still compiles inside the workspace and should not keep referencing deleted or fake root owners.
- **Alternatives considered:**
  - Ignore TS because it is lower priority — rejected because these were tiny, obvious path normalizations and leaving them behind would keep the branch inconsistent.

### Normalize shared core helper usage too
- **Chose:** Updated `app/core/gitignore.rs` to import `FileSystem` from `guardrail3_outbound_traits`.
- **Why:** That helper is live infrastructure used by both stacks. It should reflect the actual owner graph, not the old facade path.
- **Alternatives considered:**
  - Save it for a later root cleanup — rejected because it is the same dependency issue and fits naturally in this batch.

## Architectural Notes
This is still compatibility cleanup, not a redesign:
- `app/rs/validate` remains a legacy crate
- `app/ts/validate` remains legacy TS code
- but both now depend on the real outbound trait crate instead of the root facade path

That makes the crate graph more honest and reduces the amount of code that still pretends the root package owns core abstractions.

## Information Sources
- `rg -n "crate::ports::outbound" apps/guardrail3/crates` — confirmed the stale import sites.
- `apps/guardrail3/crates/app/rs/validate/*` — legacy Rust validator modules touched in this batch.
- `apps/guardrail3/crates/app/ts/validate/*` — legacy TS validator modules touched in this batch.
- `apps/guardrail3/crates/app/core/gitignore.rs` — shared helper still using the old path.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — prior crate-promotion context.
- `.worklogs/2026-03-25-115137-direct-crates-in-legacy-validate-entry.md` — prior direct-owner cleanup inside the legacy Rust validator crate.

## Open Questions / Future Considerations
- The larger remaining compatibility debt is still the broad root facade and the legacy tests that import `guardrail3::app::rs::validate::*`.
- There is still unrelated test/doc/worktree noise outside this batch that should either be committed coherently or reverted to restore a clean branch.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy Rust validator crate entrypoint.
- `apps/guardrail3/crates/app/ts/validate/mod.rs` — legacy TS validator entrypoint.
- `apps/guardrail3/crates/app/core/gitignore.rs` — shared gitignore helper now using the real traits crate.
- `apps/guardrail3/crates/ports/outbound/traits/fs_types.rs` — canonical outbound filesystem trait types.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — legacy validator crate promotion.
- `.worklogs/2026-03-25-115137-direct-crates-in-legacy-validate-entry.md` — prior direct-crate cleanup inside the same compatibility area.

## Next Steps / Continuation Plan
1. Audit the remaining modified files outside this batch and revert or recommit anything that does not materially advance crate decoupling.
2. Keep shrinking the live reliance on `guardrail3::app::rs::validate::*`, especially in the root facade and root tests.
3. Continue verifying with crate-local and workspace-lib builds so the branch converges toward clean compile boundaries instead of just moving code around.
