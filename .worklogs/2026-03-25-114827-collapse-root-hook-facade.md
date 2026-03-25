# Collapse Root Hook Facade

**Date:** 2026-03-25 11:48
**Scope:** `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs`, `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs`, `apps/guardrail3/crates/app/rs/checks/hooks/shell_tests.rs`, `apps/guardrail3/crates/lib.rs`

## Summary
Collapsed the remaining live root hook façade so it no longer owns a local hook orchestrator or shell parser copy. The root now reexports hook checking from `guardrail3-app-hooks`, and the duplicated root `shell.rs` parser was deleted in favor of the existing `hooks-shared` crate owner.

## Context & Problem
After promoting `app/rs/validate` and `app/arch-helpers`, the next remaining Rust-side fake boundary was `app/rs/checks/hooks`. It looked small, but it still did two things locally:
- defined its own `check(...)` wrapper even though `app-hooks` already owned that orchestration
- compiled a local `shell.rs` parser even though `hooks-shared` already owned the canonical shell parser

That is exactly the kind of transitional glue that tends to stick around and quietly preserve the root facade as a real owner instead of a thin adapter.

## Decisions Made

### Reexport hook checking from `app-hooks`
- **Chose:** Replaced the local `check(...)` implementation in `app/rs/checks/hooks/mod.rs` with `pub use guardrail3_app_hooks::check;`.
- **Why:** `app-hooks` is already the actual orchestrator crate. Keeping a parallel wrapper in the root adds no real value and preserves unnecessary root-owned code.
- **Alternatives considered:**
  - Keep the wrapper for convenience — rejected because it preserves duplicate orchestration logic in the facade.
  - Move more hook logic back into the root crate — rejected because it directly fights the workspace split.

### Delete the root-local shell parser copy
- **Chose:** Deleted `app/rs/checks/hooks/shell.rs` and `shell_tests.rs` instead of forwarding them.
- **Why:** The canonical parser already lives in `guardrail3-app-rs-family-hooks-shared::hook_shell`. The root copy was duplicate compiled code, not a necessary compatibility surface.
- **Alternatives considered:**
  - Reexport the shell module from the root path — rejected because the root build immediately proved that nothing live uses it anymore, so preserving the alias would just be noise.
  - Keep both copies in sync — rejected because duplicate parser ownership is precisely the maintenance problem the split is trying to remove.

### Keep root package dependency linting green explicitly
- **Chose:** Added underscore imports for `guardrail3_domain_project_tree` and `guardrail3_outbound_traits` in `crates/lib.rs`.
- **Why:** Once the local hook wrapper disappeared, those package dependencies stopped being referenced by the root lib target even though they are still package-level dependencies elsewhere. The workspace lint requires making that explicit.
- **Alternatives considered:**
  - Ignore the lint — rejected because it would hide real dependency drift.
  - Remove the dependencies immediately — rejected because that is a separate package-ownership cleanup and not part of this narrow hook-facade cut.

## Architectural Notes
This change is intentionally narrow. It does not redesign hook tests or move more hook rule code around. It just removes one more case where the root crate was still acting like a real implementation owner.

The result is cleaner:
- `guardrail3-app-hooks` owns hook orchestration
- `guardrail3-app-rs-family-hooks-shared` owns shell parsing
- the root `app::rs::checks::hooks` path is now just compatibility glue

## Information Sources
- `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs` — root hook façade before cleanup.
- `apps/guardrail3/crates/app/hooks/mod.rs` — real hook orchestrator crate owner.
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/lib.rs` — canonical hook shared family crate.
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shell.rs` — canonical shell parser owner.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — prior crate-boundary cut that led directly to this cleanup.

## Open Questions / Future Considerations
- The old source trees under `app/rs/checks/hooks/**` and `app/rs/checks/rs/**` still exist on disk as compatibility residue even where they are no longer compiled by live root modules. That is mostly codebase cleanliness debt now, not a compile-boundary blocker.
- The bigger remaining compatibility island is still `app/rs/validate`, now isolated as its own crate but still large internally.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs` — root compatibility façade after collapse.
- `apps/guardrail3/crates/app/hooks/mod.rs` — real hook owner crate.
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/lib.rs` — hook shared family owner surface.
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shell.rs` — canonical shell parser.
- `apps/guardrail3/crates/lib.rs` — root package dependency marker imports.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — immediately preceding crate split decision record.

## Next Steps / Continuation Plan
1. Audit the remaining live root compatibility modules and remove any other local wrappers that simply duplicate promoted crate owners.
2. Keep isolating the remaining Rust compatibility debt around `app/rs/validate` by routing more live callers to family/runtime crates instead of the legacy crate.
3. Avoid deleting dead compatibility trees blindly; first confirm they are no longer compiled or imported by live runtime/CLI paths.
4. Keep verification at crate scope and workspace-lib scope after each cut so the compile-boundary wins remain measurable.
