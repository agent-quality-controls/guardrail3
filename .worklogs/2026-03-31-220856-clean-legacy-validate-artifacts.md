# Clean Parked Legacy Validator Build Artifacts

**Date:** 2026-03-31 22:08
**Scope:** `apps/guardrail3/crates/app/rs/legacy/validate`, `.worklogs/2026-03-31-220856-clean-legacy-validate-artifacts.md`

## Summary
Removed the accidental `Cargo.lock` and `target/` tree that were created inside the parked legacy validator crate during standalone verification. This restores the intended state: the old validator code is preserved, but it is not treated like an active independently-built subproject inside the repo tree.

## Context & Problem
After moving `app/rs/validate` to `app/rs/legacy/validate`, I ran `cargo check --manifest-path .../legacy/validate/Cargo.toml` to verify that the move exposed only legacy-code failures, not path/workspace failures. Because the crate is now excluded from the main workspace, Cargo created a local lockfile and build output under the legacy directory. Those artifacts were accidentally committed in the first move commit.

## Decisions Made

### Remove Local Cargo Artifacts From The Parked Legacy Crate
- **Chose:** Deleted `apps/guardrail3/crates/app/rs/legacy/validate/Cargo.lock` and the entire local `target/` directory.
- **Why:** The user asked to preserve the legacy code, not to turn it into a separately managed mini-workspace with checked-in build products.
- **Alternatives considered:**
  - Keep the local lockfile and target tree — rejected because they are generated noise and make the preserved legacy area look actively maintained.
  - Re-add the legacy crate to the workspace to avoid local artifacts — rejected because that would undo the architectural isolation we just established.

## Architectural Notes
The right steady state is:
- legacy source preserved
- no active workspace membership
- no checked-in local build output

Directly checking the parked manifest is still useful for spot inspection, but it should be treated as an ad hoc debugging action, not a normal project build path.

## Information Sources
- `.worklogs/2026-03-31-220757-move-legacy-rs-validate.md` — the move that created the parked legacy location
- `git status` after the standalone `cargo check` — showed the accidental lockfile and `target/` tree

## Open Questions / Future Considerations
- If the team expects to inspect the parked legacy crate repeatedly, adding a local `.gitignore` under `app/rs/legacy/validate` may be worthwhile to guard against this recurring.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/legacy/validate/Cargo.toml` — the parked legacy crate manifest
- `.worklogs/2026-03-31-220757-move-legacy-rs-validate.md` — why the crate was moved and excluded from the active workspace
- `.worklogs/2026-03-31-220856-clean-legacy-validate-artifacts.md` — this artifact cleanup record

## Next Steps / Continuation Plan
1. Keep using the active workspace checks (`apps/guardrail3/Cargo.toml`) as the real verification path.
2. If the parked legacy crate needs future inspection, expect direct manifest checks to recreate local build output unless a local `.gitignore` is added.
3. Decide later whether that local `.gitignore` is worth adding or whether the legacy crate should simply stop being built at all.
