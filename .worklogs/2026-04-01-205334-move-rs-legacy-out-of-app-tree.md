# Move RS Legacy Out Of App Tree

**Date:** 2026-04-01 20:53
**Scope:** `.gitignore`, `apps/guardrail3/Cargo.toml`, `legacy/apps/guardrail3/crates/app/rs/legacy/validate`

## Summary
Moved the old Rust legacy validator crate out of `apps/guardrail3` into repo-root `legacy/...`, added `/legacy/` to `.gitignore`, and removed the stale workspace exclude entry that referenced the old in-tree location. Then reran `family-code` to confirm the legacy path no longer contributes governed findings.

## Context & Problem
The code-family audit showed that part of the remaining `RS-CODE` debt was not active application code at all, but the old `apps/guardrail3/crates/app/rs/legacy/validate` migration baggage. Inspection showed the current Rust entrypoint uses the new runtime pipeline and that the legacy crate is excluded from the workspace and not referenced by any live package dependency.

Keeping the old crate inside the governed app tree had two bad effects:
- `RS-CODE` still scanned it as normal Rust source
- `RS-CODE-30` parse-failure findings were coming from dead migration code instead of the live architecture

The user direction was to move the whole thing to repo-root `/legacy` and ignore that root.

## Decisions Made

### Move the legacy crate physically out of the governed app tree
- **Chose:** Moved `apps/guardrail3/crates/app/rs/legacy/validate` to `legacy/apps/guardrail3/crates/app/rs/legacy/validate`.
- **Why:** The current validator runs against `apps/guardrail3`, not the entire repo root. Moving the old crate outside that governed root removes it from live Rust-family scanning without needing any family-local carveout.
- **Alternatives considered:**
  - Leave it in place and add a code-family exclusion — rejected because this is not a `code` concern; dead migration code should not stay inside the governed app tree.
  - Delete it immediately — rejected because the user asked to archive it under `/legacy`, not remove it entirely.

### Ignore the archive root explicitly
- **Chose:** Added `/legacy/` to repo `.gitignore`.
- **Why:** The archive root is not active product code. Ignoring it makes the intent visible and prevents casual local work under `legacy/` from surfacing as normal repo noise.
- **Alternatives considered:**
  - Keep it tracked without an ignore entry — rejected because it would look like normal active source and invite accidental edits.

### Remove stale workspace metadata
- **Chose:** Removed the old `exclude = ["crates/app/rs/legacy/validate"]` entry from `apps/guardrail3/Cargo.toml`.
- **Why:** After the move, that path no longer exists under the workspace root. Keeping the exclude would just preserve dead metadata.
- **Alternatives considered:**
  - Leave the exclude as a historical breadcrumb — rejected because stale workspace metadata is misleading and serves no build purpose.

## Architectural Notes
This change does not alter the active Rust runtime architecture. It only removes dead migration code from the governed source root.

Because guardrail validation is run against `apps/guardrail3`, the simplest clean boundary is physical separation:
- live code stays under `apps/guardrail3`
- archived migration code moves under repo-root `legacy/`

That is better than piling more special-case exclusion logic into families or the walker.

## Information Sources
- `apps/guardrail3/crates/app/rs/mod.rs` — confirmed the live Rust entrypoint re-exports the new runtime.
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — confirmed the current run path is walker -> structure -> legality -> mapper.
- `apps/guardrail3/Cargo.toml` — showed the old legacy crate was already excluded from the workspace.
- `apps/guardrail3/crates/app/rs/legacy/validate/Cargo.toml` — confirmed the archived code was still a standalone crate, not part of the live runtime.
- `rg -n "guardrail3-app-rs-legacy-validate|crates/app/rs/legacy/validate"` — confirmed no live package references remained outside the archived crate itself.
- `cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3` — verified the app still builds after the move.
- `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-code -- rs validate apps/guardrail3 --family code --format json` — verified the legacy path disappeared from governed `RS-CODE` findings and `RS-CODE-30` dropped to zero.
- `.worklogs/2026-04-01-202718-harden-walker-config-recovery.md` — prior work that removed fake `.claude/worktrees` noise from `RS-CODE`.

## Open Questions / Future Considerations
- `/legacy/` is now ignored, but if the project later decides that archives should never be tracked either, a follow-up could move this material completely outside the repo or into a dedicated archive branch.
- There may still be docs or plans that refer to the old in-tree legacy path. Those were not updated in this move-only pass.

## Key Files for Context
- `.gitignore` — now marks repo-root `legacy/` as archive space.
- `apps/guardrail3/Cargo.toml` — workspace membership after removing the stale legacy exclude.
- `apps/guardrail3/crates/app/rs/mod.rs` — live Rust entrypoint proving the legacy crate is not part of current runtime wiring.
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — current Rust runtime pipeline.
- `legacy/apps/guardrail3/crates/app/rs/legacy/validate/Cargo.toml` — archived crate entrypoint after the move.
- `.worklogs/2026-04-01-202718-harden-walker-config-recovery.md` — immediate prior context for the `RS-CODE` cleanup work.

## Next Steps / Continuation Plan
1. Rerun the `family-code` grouping and continue with the real remaining top bucket, `RS-CODE-24`, now that fake worktree noise and dead legacy parse failures are gone.
2. Decide whether the repeated `#[path = "..."]` registry pattern should be explicitly blessed or should carry required inline reasons everywhere.
3. If the archive root is meant to stay permanently outside governance, consider adding a shared repo policy note so future agents do not move dead migration code back under `apps/guardrail3`.
