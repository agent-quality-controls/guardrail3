# Move Legacy Rust Validator Out of Active RS Surface

**Date:** 2026-03-31 22:07
**Scope:** `AGENTS.md`, `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/legacy/validate/**`

## Summary
Moved the old Rust validator crate from `crates/app/rs/validate` to `crates/app/rs/legacy/validate`, excluded it from the workspace root, and fixed its local dependency paths after the move. The active workspace build stayed green, which confirms the new runtime path does not depend on the old crate anymore.

## Context & Problem
The repo still contained a pre-family Rust validator crate under `app/rs/validate`, and its old topology entrypoint was still bundling hexarch-style checks. That made the repo structure misleading even though the active Rust runtime already uses the new per-family pipeline. The user explicitly asked to preserve the old code but move it under a legacy location to prove it is no longer part of the live system.

## Decisions Made

### Park The Old Validator Under `rs/legacy/validate`
- **Chose:** Moved `apps/guardrail3/crates/app/rs/validate` to `apps/guardrail3/crates/app/rs/legacy/validate`.
- **Why:** The old crate is migration residue, not part of the live runtime architecture. Moving it under `legacy/` makes that status explicit without deleting any historical code.
- **Alternatives considered:**
  - Leave it in place — rejected because it keeps implying the old validator is still a first-class part of the active Rust surface.
  - Delete it outright — rejected because the user explicitly asked not to delete it.

### Exclude The Legacy Crate From The Workspace
- **Chose:** Added `crates/app/rs/legacy/validate` to `[workspace].exclude` in `apps/guardrail3/Cargo.toml`.
- **Why:** The crate is intentionally outside the live workspace and should stay that way after the move. Excluding it makes the boundary explicit and avoids future confusion if someone tries to inspect the moved manifest directly.
- **Alternatives considered:**
  - Add it back as a workspace member — rejected because that would put dead migration code back into the active build graph.
  - Leave it unexcluded — rejected because Cargo would still treat it as under the workspace tree but unmanaged.

### Keep The Legacy Manifest Coherent
- **Chose:** Updated relative dependency paths in the moved `Cargo.toml`.
- **Why:** Even though the crate is legacy, its manifest should still point at real crates from its new location. That way any future direct check fails for substantive legacy-code reasons rather than broken path bookkeeping.
- **Alternatives considered:**
  - Ignore the moved manifest — rejected because that would make the move look half-finished and hide whether the crate is structurally isolated.

## Architectural Notes
This change clarifies the intended layering:
- active Rust validation lives under the runtime plus per-family crates
- the old bundled validator is retained only as historical migration residue

The verification result matters more than the move itself: the workspace still compiles cleanly after the relocation, which is strong evidence that the active binary/runtime path no longer depends on `app/rs/validate`.

The moved legacy crate still fails a direct `cargo check`, but the failure is now inside old source files (`source/allow_checks.rs`) rather than from workspace membership or broken relative paths. That is the right failure mode for parked legacy code.

## Information Sources
- `AGENTS.md` — current project direction and explicit migration away from old `rs/validate/*`
- `apps/guardrail3/Cargo.toml` — active workspace membership boundary
- `apps/guardrail3/crates/app/rs/legacy/validate/Cargo.toml` — moved legacy crate manifest
- `apps/guardrail3/crates/app/rs/validate/mod.rs` before move — old bundled validator orchestration

## Open Questions / Future Considerations
- `apps/guardrail3/tests/golden-tests/golden/self-validate.json` still contains historical `src/rs/validate/*` file paths. Those are generated legacy artifacts, not live wiring, but they will need a conscious cleanup plan if the repo wants zero visible legacy path residue outside archival docs.
- If the team later decides the old validator has no remaining research value, the next step would be deletion rather than more maintenance.

## Key Files for Context
- `AGENTS.md` — project direction and the migration boundary between old and new Rust validation systems
- `apps/guardrail3/Cargo.toml` — workspace membership and explicit exclusion of the moved legacy crate
- `apps/guardrail3/crates/app/rs/legacy/validate/Cargo.toml` — preserved legacy crate manifest after relocation
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — live family-based Rust runtime dispatch
- `.worklogs/2026-03-31-215704-hexarch-naming-correction.md` — the preceding rename cleanup that prompted verification of the old validator surface

## Next Steps / Continuation Plan
1. Decide whether generated golden artifacts that still reference `src/rs/validate/*` should be regenerated, preserved as historical fixtures, or moved under a more explicit legacy test area.
2. If the goal is zero live legacy residue, grep for `rs/validate` outside `.plans`, `.worklogs`, and generated goldens, then either rename or remove any remaining active references.
3. If the repo eventually wants to retire the old validator fully, delete `apps/guardrail3/crates/app/rs/legacy/validate` in one dedicated cleanup commit and update any archival notes that still point at it.
