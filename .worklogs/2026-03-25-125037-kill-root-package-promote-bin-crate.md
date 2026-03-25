# Kill Root Package Promote Bin Crate

**Date:** 2026-03-25 12:50
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/Cargo.lock`, `apps/guardrail3/crates/bin/guardrail3/**`, `apps/guardrail3/crates/main.rs`, `apps/guardrail3/crates/lib.rs`, `AGENTS.md`

## Summary
Removed the fake root `guardrail3` package from `apps/guardrail3` and converted that manifest into a pure app-root workspace. Promoted the CLI entrypoint into a dedicated bin crate at `crates/bin/guardrail3`, preserved the `guardrail3` package name there, and deleted the dead root `lib.rs` / `main.rs` package targets.

## Context & Problem
The earlier workspace split had already turned almost every meaningful owner into a real crate, but the app root still carried a synthetic package that:

- owned the `guardrail3` binary
- used to own a broad root lib facade
- gave Cargo a giant synthetic integration-test/package boundary

That root package was the last big non-domain ownership shell in the app workspace. It also kept the root `tests/` topology attached to a package we no longer wanted architecturally. The user explicitly wanted to remove that package instead of keeping a “thin root package” compromise.

I first checked the real repo shape. There is only one live app root under `apps/*` right now: `apps/guardrail3`. There are no live top-level `packages/*` Rust package roots outside fixtures, so the practical implementation of the request was to fix `apps/guardrail3` now rather than invent package work that does not yet exist.

## Decisions Made

### Convert `apps/guardrail3` into a virtual workspace manifest
- **Chose:** Remove `[package]`, `[lib]`, `[[bin]]`, root dependencies, and dev-dependencies from `apps/guardrail3/Cargo.toml`, leaving only workspace membership/dependency/lint configuration.
- **Why:** The user wanted the root package gone, not merely thinner. A pure workspace manifest removes the synthetic package boundary entirely.
- **Alternatives considered:**
  - Keep a thin root package — rejected because that preserves the exact compile boundary we were trying to kill.
  - Leave the old root package but move more code out — rejected because ownership was already mostly split; the remaining problem was the package itself.

### Create a dedicated app-local bin crate for `guardrail3`
- **Chose:** Add `apps/guardrail3/crates/bin/guardrail3` as a real workspace member and give it the package name `guardrail3`.
- **Why:** The CLI still needs a concrete package owner. Moving it into its own crate keeps the product entrypoint while removing the fake workspace-root package.
- **Alternatives considered:**
  - Rename the product package — rejected because there was no need to break the CLI package identity.
  - Fold the binary into `adapters/inbound/cli` — rejected because that crate is still a library-style owner and not the right place to absorb product-bin concerns in this pass.

### Delete the dead root package entrypoint files
- **Chose:** Delete `apps/guardrail3/crates/main.rs` and `apps/guardrail3/crates/lib.rs` after copying the binary entrypoint into the new bin crate.
- **Why:** Leaving those files behind would preserve dead package-era paths and confuse future work. Once the root package is gone, those files are not real owners anymore.
- **Alternatives considered:**
  - Keep the files around as dead compatibility artifacts — rejected because they would immediately become misleading cold-start anchors.
  - Point the new bin crate at the old `crates/main.rs` path — rejected because that would keep the dead root layout alive by indirection.

### Keep the workspace root focused on real live app roots only
- **Chose:** Implement the app-root workspace cut only for `apps/guardrail3`.
- **Why:** There are no live top-level `packages/*` Rust roots in this repo outside fixtures, so there was nothing real to convert on the package side.
- **Alternatives considered:**
  - Manufacture empty `packages/*` workspaces — rejected because that would be architecture theater rather than actual ownership work.

## Architectural Notes
- `apps/guardrail3/Cargo.toml` is now the workspace root only. It is no longer a package.
- The product package `guardrail3` now lives at `apps/guardrail3/crates/bin/guardrail3`.
- This means root `apps/guardrail3/tests/**` is no longer attached to a package target. That is intentional fallout from killing the root package. Those tests now need to be moved to real owners or replaced by dedicated test crates later.
- The compile proof target that matters after this cut is the workspace and the dedicated bin crate, not the deleted root package.
- This cut aligns better with the architectural goal the user stated: each app root owns a workspace, and leaf crates belong to that app workspace rather than to a synthetic product shell.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-03-25-123323-thin-root-facade-and-direct-owner-imports.md`
- `apps/guardrail3/Cargo.toml`
- `apps/guardrail3/crates/main.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/Cargo.toml`
- `find apps -mindepth 1 -maxdepth 1 -type d | sort`
- `find packages -mindepth 1 -maxdepth 1 -type d | sort`
- `cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3`
- `cargo check --manifest-path apps/guardrail3/Cargo.toml --workspace`

## Open Questions / Future Considerations
- The root `apps/guardrail3/tests/**` tree is now orphaned from a package by design. The next architectural step is to move those tests onto owner crates or create explicit integration-test crates where cross-crate coverage is still needed.
- There are historical plan docs that still talk about a “thin root package.” Those are now historical, not current truth.
- The repo still has only one live app-root Rust workspace. If real `packages/*` Rust roots appear later, they should follow the same pattern: workspace root at the package root, leaf crates as members.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — now a pure workspace manifest for the app root
- `apps/guardrail3/crates/bin/guardrail3/Cargo.toml` — new product package manifest
- `apps/guardrail3/crates/bin/guardrail3/src/main.rs` — new CLI entrypoint owner
- `AGENTS.md` — updated cold-start pointers after deleting the root package files
- `.worklogs/2026-03-25-123323-thin-root-facade-and-direct-owner-imports.md` — prior step that removed root-facade callers before killing the package

## Next Steps / Continuation Plan
1. Stop treating `apps/guardrail3/tests/**` as if it still belongs to a root package. Decide test-by-test whether each file belongs on a real owner crate or in a future dedicated integration-test crate.
2. Trim historical references to the deleted root package where they are still misleading operational docs, especially any current handoff text that still points at `crates/main.rs` or `crates/lib.rs` as live entrypoints.
3. Continue removing remaining callers of `guardrail3-app-rs-legacy-validate`, since the package-boundary cleanup is now done and the next meaningful decoupling work is compatibility-surface reduction.
