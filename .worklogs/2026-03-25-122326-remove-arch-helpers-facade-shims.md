# Remove Arch Helpers Facade Shims

**Date:** 2026-03-25 12:23
**Scope:** `apps/guardrail3/crates/lib.rs`, `apps/guardrail3/crates/app/ts/mod.rs`, `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`, `apps/guardrail3/crates/app/rs/validate/mod.rs`, `apps/guardrail3/crates/app/rs/validate/arch/rs_arch_01/helpers.rs`

## Summary
Removed the remaining live `arch_helpers` facade shims by switching the last TS and legacy-RS callers to direct `guardrail3_app_arch_helpers` imports. With those callers gone, the root and nested facade modules no longer reexport `arch_helpers`.

## Context & Problem
After the previous caller-migration pass, the remaining root-facade debt was concentrated in one shared helper surface: `arch_helpers`. The helper had already been promoted into its own crate, but live code still reached it through facade namespaces:
- `app/ts/validate/ts_arch_checks.rs` used `crate::app::arch_helpers`
- `app/rs/validate/arch/rs_arch_01/helpers.rs` used `crate::app::arch_helpers`

That kept three unnecessary reexports alive:
- `guardrail3::app::arch_helpers`
- `app-ts::app::arch_helpers`
- `app-rs-legacy-validate::app::arch_helpers`

As long as those remained, the facade still looked like an owner for a helper that already has its own crate.

## Decisions Made

### Switch live helper callers to the owner crate directly
- **Chose:** Replaced `crate::app::arch_helpers` with `guardrail3_app_arch_helpers` in the last two live call sites.
- **Why:** The helper already has a real owner crate, so the direct dependency is the correct architecture.
- **Alternatives considered:**
  - Leave the nested shim modules in place — rejected because they preserve fake ownership for no real compatibility benefit.
  - Reintroduce local forwarding modules — rejected because it directly fights the goal of a thin facade.

### Remove the now-unused `arch_helpers` reexports
- **Chose:** Deleted the `arch_helpers` reexport from the root facade and from the nested `app` shims in `app-ts` and `app-rs-legacy-validate`.
- **Why:** Once there are no live callers, the facade should stop advertising the helper as part of its public module tree.
- **Alternatives considered:**
  - Keep the reexports as convenience aliases — rejected because they keep the facade broader without any demonstrated need.

### Keep the root package dependency marker for now
- **Chose:** Left the underscore dependency marker import for `guardrail3_app_arch_helpers` in `crates/lib.rs`.
- **Why:** The root package still depends on the crate at package scope, and the project enforces `unused_crate_dependencies = "deny"`. Removing the marker would require a separate dependency-graph cleanup in `Cargo.toml`.
- **Alternatives considered:**
  - Remove the dependency marker immediately — rejected because this commit is about live facade removal, not package dependency rebalancing.

## Architectural Notes
This is a small but important honesty cleanup:
- `guardrail3-app-arch-helpers` is now the only live owner for those shared arch helper functions
- the root facade and nested compatibility crates no longer pretend to own or re-export that helper surface

It continues the same pattern as the earlier `validate` and `hooks` cleanup: once callers are moved, the facade should narrow immediately instead of carrying compatibility aliases indefinitely.

## Information Sources
- `rg -n "crate::app::arch_helpers|guardrail3::app::arch_helpers|pub use guardrail3_app_arch_helpers as arch_helpers"` across `apps/guardrail3/crates` and `apps/guardrail3/tests` — identified the remaining live shim edges.
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs` — last TS caller.
- `apps/guardrail3/crates/app/rs/validate/arch/rs_arch_01/helpers.rs` — last legacy-RS caller.
- `apps/guardrail3/crates/lib.rs` — root facade.
- `apps/guardrail3/crates/app/ts/mod.rs` — nested TS facade.
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — nested legacy validator facade.
- `.worklogs/2026-03-25-121859-cut-root-tests-off-legacy-facades.md` — prior caller-migration commit that made this safe.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — original promotion of the helper into its own crate.

## Open Questions / Future Considerations
- The root package still carries an underscore dependency marker for `guardrail3_app_arch_helpers`. That can potentially be removed later if the root package manifest is tightened, but this commit does not attempt that dependency cleanup.
- The branch is now much closer to a truly thin root facade, but `lib.rs` still reexports broad app/domain/adapters surfaces by design.

## Key Files for Context
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs` — direct owner import for TS arch helpers.
- `apps/guardrail3/crates/app/rs/validate/arch/rs_arch_01/helpers.rs` — direct owner import for legacy RS arch helpers.
- `apps/guardrail3/crates/lib.rs` — root facade after the `arch_helpers` reexport removal.
- `apps/guardrail3/crates/app/ts/mod.rs` — nested TS facade after the `arch_helpers` reexport removal.
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — nested legacy validator facade after the `arch_helpers` reexport removal.
- `.worklogs/2026-03-25-121859-cut-root-tests-off-legacy-facades.md` — preceding internal caller migration.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — helper-crate promotion backstory.

## Next Steps / Continuation Plan
1. Reassess whether any remaining root-facade reexports are now similarly unused by internal callers and can be narrowed without changing the package contract unexpectedly.
2. If the goal remains even thinner package boundaries, audit whether the root package can drop direct dependencies now kept alive only by underscore marker imports.
3. Keep verifying with crate-local and workspace-lib checks after each facade reduction so the branch stays clean and defensible.
