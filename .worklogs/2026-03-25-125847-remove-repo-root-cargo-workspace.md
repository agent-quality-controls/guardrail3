# Remove Repo Root Cargo Workspace

**Date:** 2026-03-25 12:58
**Scope:** `Cargo.toml`, `apps/guardrail3/Cargo.toml`

## Summary
Removed the top-level repo Cargo workspace entirely and moved the only useful shared Rust profile setting (`profile.mutants`) into the real app-root workspace at `apps/guardrail3`. This makes the Cargo ownership model match the intended architecture: the app root is the workspace root, not the repository root.

## Context & Problem
I had previously removed the `apps/guardrail3` root package and promoted the CLI into a dedicated bin crate, but I left the repository root `Cargo.toml` in place as a Cargo workspace with `exclude = ["apps/guardrail3"]`.

That was still wrong for the requested architecture. The user explicitly wanted Cargo workspaces rooted at real app/package roots:

- `apps/<app>/Cargo.toml`
- `packages/<package>/Cargo.toml`

Leaving a repo-root Cargo workspace in place meant the repository still had a top-level Cargo owner, even if it excluded the child app workspace. That is not the parent-root ownership model the user asked for.

## Decisions Made

### Delete the repo-root Cargo workspace
- **Chose:** Delete `/Cargo.toml` completely.
- **Why:** There are no repo-root Rust packages left to own, and keeping a repo-root Cargo workspace would continue to contradict the requested topology.
- **Alternatives considered:**
  - Keep the root workspace and exclude child app workspaces — rejected because it preserves a top-level Cargo owner.
  - Convert the root manifest into a non-workspace package — rejected because there is no real repo-root Rust package to own.

### Move the useful profile setting into the real app workspace
- **Chose:** Add `[profile.mutants]` to `apps/guardrail3/Cargo.toml`.
- **Why:** This was the only nontrivial Cargo setting in the deleted root manifest worth preserving.
- **Alternatives considered:**
  - Drop the profile entirely — rejected because it was cheap to preserve and still relevant to the live app workspace.
  - Keep a root manifest just for this profile — rejected because that would reintroduce the dead top-level Cargo owner.

## Architectural Notes
- The repository root is no longer a Cargo root.
- `apps/guardrail3` is now the only live Cargo workspace in this repo.
- This is much closer to the target topology you described:
  - parent app root owns its leaf crates
  - no synthetic repo-root Cargo owner
- There are still no live top-level `packages/*` Rust roots outside fixtures, so there was nothing real to convert on the package side yet.

## Information Sources
- `Cargo.toml`
- `apps/guardrail3/Cargo.toml`
- `cargo check --manifest-path apps/guardrail3/Cargo.toml --workspace`
- `cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3`
- `.worklogs/2026-03-25-125037-kill-root-package-promote-bin-crate.md`

## Open Questions / Future Considerations
- If real Rust package roots appear later under `packages/*`, they should each get their own workspace-root `Cargo.toml` and leaf-crate membership, just like `apps/guardrail3`.
- Any docs or scripts that assume a repo-root Cargo workspace will need to be corrected over time. The live build proof now runs from `apps/guardrail3`.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — the real Cargo workspace root
- `.worklogs/2026-03-25-125037-kill-root-package-promote-bin-crate.md` — prior step removing the app-root package shell
- `AGENTS.md` — current live workspace entrypoint references

## Next Steps / Continuation Plan
1. Keep developing Rust work from `apps/guardrail3/Cargo.toml` as the only live Cargo workspace root.
2. Continue moving remaining tests/callers off compatibility surfaces like `guardrail3-app-rs-legacy-validate`.
3. If new real Rust roots are introduced under `packages/*`, create one workspace per package root rather than reintroducing a repo-root Cargo manifest.
