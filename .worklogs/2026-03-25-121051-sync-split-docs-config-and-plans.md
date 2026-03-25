# Sync Split Docs Config And Plans

**Date:** 2026-03-25 12:10
**Scope:** `.plans/**`, `guardrail3.toml`, `GUARDRAIL3_GUIDE.md`, `apps/guardrail3/crates/domain/modules/guide.rs`, `apps/guardrail3/Cargo.lock`

## Summary
Updated the in-repo plans, generated guide content, self-config, and workspace lockfile so they describe the crate-split, family-based Rust validation model that now exists on the branch. This removes a large amount of stale guidance that still spoke in grouped-domain or pre-split terms.

## Context & Problem
The codebase has been moved onto real family crates, a real legacy-validate compatibility crate, and family-based Rust validation routing. But the branch still carried several stale or partial descriptive artifacts:
- plans that still described older grouped-domain or incomplete-cutover states
- generated guide content that did not match the current CLI/family model
- root `guardrail3.toml` keys that still used removed grouped Rust check names
- an inner-workspace `Cargo.lock` missing the newly promoted crates

That mismatch is dangerous. It makes the branch look architecturally inconsistent even when the code is already split.

## Decisions Made

### Sync self-config to family-based Rust checks
- **Chose:** Updated `guardrail3.toml` to use the current family keys (`fmt`, `toolchain`, `clippy`, `deny`, `cargo`, `code`, `hexarch`, `deps`, `garde`, `test`, `release`, `hooks_shared`, `hooks_rs`).
- **Why:** The root config should describe the current runtime model, not the removed grouped keys such as `architecture`, `tests`, and `hooks`.
- **Alternatives considered:**
  - Leave the old keys for compatibility — rejected because it preserves a misleading self-example in the main repo config.
  - Change runtime back toward grouped flags — rejected because it fights the cutover that is already underway.

### Commit the lockfile updates from the promoted crate graph
- **Chose:** Included `apps/guardrail3/Cargo.lock` updates for the promoted crates such as `guardrail3-app-arch-helpers` and `guardrail3-app-rs-legacy-validate`.
- **Why:** The inner workspace is real now. Its lockfile should reflect the actual crate graph rather than remaining half-updated in the working tree.
- **Alternatives considered:**
  - Leave the lockfile dirty until the very end — rejected because it keeps the branch perpetually out of sync with the buildable workspace.

### Sync guide and plan text to the live split architecture
- **Chose:** Included the generated guide surfaces and the tracked plan documents that now describe the family-based Rust model, the crate split, and the `arch` family.
- **Why:** The branch should not carry a code architecture that materially disagrees with the repository’s own planning and guidance artifacts.
- **Alternatives considered:**
  - Revert all plan/doc updates and leave only code changes — rejected because the repo already uses `.plans` and embedded guide text as active project state, not optional notes.
  - Delay doc/plan synchronization until every test is clean — rejected because stale guidance actively increases confusion while the architecture work is already real.

## Architectural Notes
This commit does not introduce new crate boundaries by itself. It makes the repository’s descriptive surfaces consistent with the crate boundaries and runtime model already present on the branch.

The important consistency points are:
- Rust validation is family-based
- `arch` is part of the current family model
- the new inner workspace crate graph is real and reflected in `Cargo.lock`
- the generated guide no longer advertises only the older grouped-domain contract

## Information Sources
- `guardrail3.toml` — root self-config and family keys.
- `GUARDRAIL3_GUIDE.md` — generated guide surface at repo root.
- `apps/guardrail3/crates/domain/modules/guide.rs` — embedded generated guide content shipped by the product.
- `.plans/todo/checks/2026-03-24-rust-validation-cutover.md` — active cutover plan.
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — active workspace split plan.
- `.plans/2026-03-24-rs-arch-handoff.md` and `.plans/todo/checks/rs/arch.md` — new `arch` family planning/handoff context.
- `apps/guardrail3/Cargo.lock` — inner workspace lockfile reflecting promoted crates.

## Open Questions / Future Considerations
- The remaining dirty tracked changes after this commit are root test compatibility updates. Those should either be committed coherently or reverted, but they should not remain as unmanaged residue.
- There may still be user-facing help or remediation text outside the guide that references older command shapes; those should be audited separately if they show up again.

## Key Files for Context
- `guardrail3.toml` — canonical example config for the current Rust family model.
- `GUARDRAIL3_GUIDE.md` — generated external guide content.
- `apps/guardrail3/crates/domain/modules/guide.rs` — embedded guide content used by the product.
- `.plans/todo/checks/2026-03-24-rust-validation-cutover.md` — current cutover definition.
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — current workspace split plan.
- `.plans/todo/checks/rs/arch.md` — `arch` family scope.
- `.plans/2026-03-24-rs-arch-handoff.md` — implementation handoff for `arch`.
- `apps/guardrail3/Cargo.lock` — current inner-workspace dependency snapshot.

## Next Steps / Continuation Plan
1. Normalize the remaining root test updates into one coherent commit focused on compatibility with the split architecture and new CLI family model.
2. Re-run targeted test commands only for the updated root test surfaces if that proves useful, but do not reopen broad test-relayering work.
3. Finish by making the git worktree clean so the branch state is fully managed rather than carrying residual local edits.
