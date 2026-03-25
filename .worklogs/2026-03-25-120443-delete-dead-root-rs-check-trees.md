# Delete Dead Root Rs Check Trees

**Date:** 2026-03-25 12:04
**Scope:** `apps/guardrail3/crates/app/rs/checks/hooks`, `apps/guardrail3/crates/app/rs/checks/rs`

## Summary
Removed the old root-owned Rust and hook check source trees now that the live `app::rs::checks` surfaces only reexport family crates. This eliminates the duplicate on-disk architecture that was keeping the branch dirty and obscuring which code is actually compiled.

## Context & Problem
After the family and hook crates were promoted, the root `app/rs/checks/**` modules had already been collapsed to thin reexport facades. The old production files and sidecar tests under `app/rs/checks/rs/*` and `app/rs/checks/hooks/{rs,shared}` were no longer on the live compile path, but they still existed on disk and in git status.

That created two problems:
- the source tree still looked like there were two active architectures at once
- the dirty worktree was dominated by deletions and edits in files that no longer had any live ownership role

The user asked for one clean branch that is fully managed and architecturally coherent. Leaving dead duplicate trees around directly fights that goal.

## Decisions Made

### Delete the dead root check trees instead of preserving aliases
- **Chose:** Removed the old root check files under `app/rs/checks/rs/*`, `app/rs/checks/hooks/rs/*`, `app/rs/checks/hooks/shared/*`, and `app/rs/checks/rs/rust_root_placement.rs`.
- **Why:** The real owners are now the promoted family crates. Keeping the old trees as dead copies would preserve ambiguity without providing compatibility value.
- **Alternatives considered:**
  - Leave the files on disk as historical residue — rejected because it keeps the branch noisy and hides the real owner graph.
  - Reexport or forward each old module path — rejected because the root facades already reduced to crate reexports, and there were no live imports that needed file-level aliases.

### Verify live ownership before deletion
- **Chose:** Confirmed the only remaining files under `app/rs/checks` are `mod.rs` reexport facades and verified the workspace lib build after deletion.
- **Why:** The cleanup had to be a real architecture simplification, not a risky mass delete.
- **Alternatives considered:**
  - Delete first and sort out breakage later — rejected because the branch already has enough moving parts and this cleanup was easy to prove safely.

## Architectural Notes
This commit makes the source tree match the actual crate architecture:
- `app::rs::checks::rs` is a compatibility facade that reexports the real family crates
- `app::rs::checks::hooks` is a compatibility facade that reexports the real hook crates and `app-hooks`
- the rule implementations no longer exist twice in the repository

This is intentionally branch-hygiene and ownership cleanup, not test architecture work.

## Information Sources
- `apps/guardrail3/crates/app/rs/checks/rs/mod.rs` — confirmed the Rust checks surface is now only crate reexports.
- `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs` — confirmed the hook checks surface is now only crate reexports plus `app-hooks::check`.
- `find apps/guardrail3/crates/app/rs/checks -maxdepth 3 -type f | sort` — confirmed only the three facade files remain after deletion.
- `cargo check --manifest-path apps/guardrail3/Cargo.toml --workspace --lib` — verified the workspace lib boundary after removing the dead trees.
- `.worklogs/2026-03-25-114827-collapse-root-hook-facade.md` — prior worklog that identified these trees as compatibility residue.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — prior worklog that pushed the split far enough for this deletion to become safe.

## Open Questions / Future Considerations
- The remaining structural cleanup is now concentrated in `app/rs/validate`, the broad root facade, and root-level tests.
- There are still many modified files in the worktree outside this deletion batch. They need to be normalized into coherent commits or removed if they are stale residue.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/mod.rs` — top-level checks facade.
- `apps/guardrail3/crates/app/rs/checks/rs/mod.rs` — Rust-family reexport facade.
- `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs` — hook-family reexport facade.
- `apps/guardrail3/Cargo.toml` — workspace crate membership that now owns the real implementations.
- `.worklogs/2026-03-25-114827-collapse-root-hook-facade.md` — explains the preceding root hook facade collapse.
- `.worklogs/2026-03-25-114457-promote-legacy-validate-and-arch-helpers.md` — explains the preceding crate promotions that made this safe.

## Next Steps / Continuation Plan
1. Audit the remaining dirty files under `apps/guardrail3/crates/app/rs/validate` and separate true live compatibility work from stale residue.
2. Thin the root facade in `apps/guardrail3/crates/lib.rs` and related module facades so the promoted crates remain the only real owners.
3. Keep verifying with narrow crate targets and `--workspace --lib` checks after each cleanup batch so the branch converges toward a single coherent architecture rather than accumulating more parallel trees.
