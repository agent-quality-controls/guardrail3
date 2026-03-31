# Cut Production Tree Ingress

**Date:** 2026-03-31 20:15
**Scope:** `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/hooks/mod.rs`

## Summary
Removed the remaining production uses of `RsProjectSurface::from_tree(...)` from Rust runtime entrypoints and the app hooks wrapper, replacing them with explicit routed surfaces. The final shape keeps the intended family contracts intact: `arch` still gets repo-global config context, `code` remains repo-global, `test` remains global, and hooks now receive an explicit file/dir slice instead of raw repo authority.

## Context & Problem
The user asked to eliminate direct family access to the full project tree and to stop families from implicitly deciding their own authority. Earlier work had removed most direct `ProjectTree` ingress from Rust families, but production runtime still had raw full-surface handoffs in important places:

- `arch` runner
- `code` runner
- `test` runner history had recently been corrected but needed to be preserved
- `hooks-shared` and `hooks-rs` runners
- app-level hooks wrapper

At the same time, the first strict cut can easily overshoot and accidentally change family semantics:

- `arch` still needs repo-level `guardrail3.toml`
- `code` is global and must not be narrowed by subtree scope
- `test` is global and must not be narrowed by subtree scope
- hooks need directory presence as well as file presence

So the task was not “ban `from_tree` everywhere” blindly. It was “remove raw production ingress while preserving the intended routed contracts.”

## Decisions Made

### Remove production `from_tree(...)` calls, not test helpers yet
- **Chose:** cut `RsProjectSurface::from_tree(ctx.tree)` out of production runtime/hook entrypoints first, while leaving test helper uses alone.
- **Why:** the user’s immediate concern was production family authority. Test helper cleanup is still worthwhile, but it is a separate layer and should not be mixed into this first boundary commit.
- **Alternatives considered:**
  - Delete all `from_tree(...)` uses in one pass — rejected because many remaining uses are test helpers and would expand the scope sharply.
  - Leave hooks on raw tree and only fix active Rust families — rejected because hooks were still a live production raw-tree ingress path.

### Add explicit routed directory support to `RsProjectSurface`
- **Chose:** extend `RsProjectSurface` with `from_route_scope_with_dirs(...)`, allowing routed surfaces to preserve explicit empty directories in addition to routed files.
- **Why:** hooks need `.githooks/pre-commit.d` and override directory presence to stay visible even when the directory has no files. File-only routing would have forced a raw-tree fallback.
- **Alternatives considered:**
  - Keep hooks on `from_tree(...)` — rejected because it preserves the exact boundary leak this patch is meant to close.
  - Fake directory presence by inventing placeholder files — rejected because it distorts the surface instead of modeling it honestly.

### Preserve global-family behavior while removing raw ingress
- **Chose:** restore `code` and `test` global semantics after the first strict cut briefly over-narrowed them, and explicitly attach repo-global context needed by `arch`.
- **Why:** the runtime tests proved that narrowing those families changed intended behavior. The correct boundary is “explicit global owned surface,” not “narrow it until tests fail.”
- **Alternatives considered:**
  - Keep the narrower surfaces and update the tests — rejected because the tests were expressing the intended global-family contract.
  - Reintroduce raw full-tree ingress — rejected because it solves the symptom by reopening the boundary leak.

## Architectural Notes
- `RsProjectSurface` is still a broad tree-style surface object, but production entrypoints now only receive explicit routed slices built from:
  - workspace roots plus routed family files
  - repo-global owned file sets for global families
  - explicit directory attachments for hooks
- `arch_surface(...)` now attaches:
  - routed Cargo roots
  - routed illegal family files
  - repo `guardrail3.toml` when present
- `code_surface(...)` now builds a repo-global routed file set instead of taking the raw tree:
  - all `.rs` files
  - repo/root config files that code-family fact collection inspects
  - routed Cargo manifests
- `test_surface(...)` stays global and does not use subtree narrowing
- hooks now get explicit routed files and explicit routed directories from both runtime and app-level wrapper paths

## Information Sources
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- `apps/guardrail3/crates/app/hooks/mod.rs`
- `.worklogs/2026-03-31-191822-runtime-scope-attack-hardening.md`
- `.worklogs/2026-03-31-200335-arch-workspace-membership-exactness.md`
- `.plans/2026-03-31-rs-family-audit-fix-plan.md`

## Open Questions / Future Considerations
- `RsProjectSurface` still implements `ProjectTreeView`. That means shared crates can still treat it as a general tree-like object. The production raw-tree ingress is now cut, but the surface type is still broader than ideal.
- Test helpers still use `RsProjectSurface::from_tree(...)`. Those uses should be audited next so tests stop normalizing a too-powerful family surface.
- `code` still discovers too much inside the family from a repo-global routed surface. This patch fixes production raw-tree ingress, not the deeper “family decides too much” problem.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — production family surface construction; now the main ingress boundary file.
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — `RsProjectSurface` constructor surface, including the new explicit-dir builder.
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — route-shaping contract for global families after preserving intended `code`/`test` behavior.
- `apps/guardrail3/crates/app/hooks/mod.rs` — app-level hooks wrapper, which previously still used raw `from_tree(...)`.
- `.plans/2026-03-31-rs-family-audit-fix-plan.md` — broader follow-up plan for remaining boundary and ownership cleanup.
- `.worklogs/2026-03-31-191822-runtime-scope-attack-hardening.md` — prior runtime-routing hardening that this boundary pass had to preserve.

## Next Steps / Continuation Plan
1. Remove test-helper uses of `RsProjectSurface::from_tree(...)` where they are proving routed behavior rather than generic helper semantics.
2. Decide whether `RsProjectSurface` should keep implementing `ProjectTreeView`, or whether shared pre-family layers should consume a narrower read trait that families cannot re-use freely.
3. Continue the deeper boundary cleanup in the global families:
   - `code` should consume an owned routed source inventory rather than re-discovering the repo from the routed surface
   - `test` should consume an owned routed test inventory rather than broad rediscovery
4. Add attack coverage specifically for “production entrypoint cannot regain raw tree authority” so this does not regress quietly.
