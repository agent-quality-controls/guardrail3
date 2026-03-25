# Cut Over Arch Code Garde Hexarch Release Family Crates

**Date:** 2026-03-25 02:43
**Scope:** `apps/guardrail3/crates/app/rs/families/arch`, `apps/guardrail3/crates/app/rs/families/code`, `apps/guardrail3/crates/app/rs/families/garde`, `apps/guardrail3/crates/app/rs/families/hexarch`, `apps/guardrail3/crates/app/rs/families/release`

## Summary
Promoted the remaining non-shell Rust families in the current sweep from shim crates to real crate-owned source trees, then closed the migration by fixing the family-owned tests and the small number of implementation bugs exposed by the move. The batch now verifies with green family test suites for `arch`, `code`, `garde`, `hexarch`, and `release`, plus a workspace-level library compile.

## Context & Problem
The workspace split had real Cargo crates for these families, but they were still structurally transitional: wrappers, copied ownership without enough verification, and family-local tests that still encoded assumptions from the legacy monolith. The current plan requires real crate ownership and fast crate-local test loops, not a facade-backed split that still depends on the old validator tree for confidence.

The broad sweep target from the plan review was:
- `arch`
- `code`
- `garde`
- `hexarch`
- `release`

These were the last high-value non-shell Rust families before the shell parser tranche (`test`, `hooks-shared`, `hooks-rs`). The goal for this batch was to finish the cutover in one coherent pass instead of continuing one-family-at-a-time iterations.

## Decisions Made

### Promote the families as real crate-owned source trees
- **Chose:** Move the family source trees into the crate roots under `apps/guardrail3/crates/app/rs/families/*/src` and make the crate `lib.rs` files the real owners.
- **Why:** The plan is about severing legacy ownership, not just adding new manifests. Fast per-family testing only appears once the crate owns its production/test sources directly.
- **Alternatives considered:**
  - Keep `#[path]` shims over `checks/rs/*` longer — rejected because it preserves the monolith dependency graph and hides ownership.
  - Split one family per commit for the rest of the sweep — rejected because the overhead was too high and the remaining families had enough pattern parity to move together.

### Treat failing migrated tests as contract checks, not “copy drift” by default
- **Chose:** Fix real implementation bugs where the migrated tests exposed contract violations, and update tests only where they were stale about ordering, fixture shape, or inventory/noise expectations.
- **Why:** Some failures were cosmetic, but some were genuine:
  - `release` was still emitting `RS-PUB-09` in non-thorough mode because the orchestrator always called the rule.
  - `release` incorrectly accepted any non-empty docs.rs table instead of requiring supported docs.rs keys.
  - `release` mishandled `package.workspace` membership when the workspace lived outside the package subtree.
  - `hexarch` treated broken `Cargo.toml` symlinks as app roots.
- **Alternatives considered:**
  - Update all failing tests to match current output — rejected because that would freeze real semantic bugs into the split.
  - Rewrite the families again before fixing the tests — rejected because the failing tests already gave precise guidance on the remaining gaps.

### Normalize path semantics at the family boundary instead of encoding odd paths in tests
- **Chose:** Normalize manifest-relative paths in `release_support::resolve_manifest_relative_path`.
- **Why:** Workspace-relative README paths like `../README.md` should resolve to a stable repo-relative path like `README.md`, not leak lexical `ws/../README.md` artifacts into facts and results.
- **Alternatives considered:**
  - Change tests to expect the non-normalized path — rejected because that would preserve a poor fact model and make downstream rule output less stable.

### Tighten `package.workspace` resolution to require actual membership
- **Chose:** Change `workspace_root_for_package(...)` / workspace-member matching so `package.workspace` only inherits metadata when the referenced workspace actually includes the package, including sibling-member patterns like `../packages/pub`.
- **Why:** The previous logic had two bad edges:
  - it could incorrectly inherit from any referenced workspace root even when the crate was not a member
  - it could incorrectly reject valid sibling-member workspaces because membership matching assumed the package lived under the workspace root directory
- **Alternatives considered:**
  - Leave `package.workspace` support partial and relax the tests — rejected because the plan and the tests explicitly exercise this path.
  - Require all package workspaces to be ancestor directories — rejected because Cargo allows sibling member patterns.

## Architectural Notes
- `arch`, `code`, `garde`, `hexarch`, and `release` are now real family owners in their crate roots.
- The migrated family suites are now viable fast loops:
  - `guardrail3-app-rs-family-arch`
  - `guardrail3-app-rs-family-code`
  - `guardrail3-app-rs-family-garde`
  - `guardrail3-app-rs-family-hexarch`
  - `guardrail3-app-rs-family-release`
- `release` now has clearer orchestrator behavior:
  - `RS-PUB-09` only runs in thorough mode at the family level
  - docs.rs presence is based on supported docs.rs configuration keys, not just table non-emptiness
  - package/workspace inheritance behaves correctly for both nested and sibling-member workspace layouts
- `hexarch` app discovery is slightly stricter now: a broken `Cargo.toml` symlink no longer counts as a real app boundary just because the path exists lexically.

## Information Sources
- Current split plan: `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md`
- Prior split worklogs:
  - `.worklogs/2026-03-25-015759-fmt-and-clippy-family-cutover.md`
  - `.worklogs/2026-03-25-021238-cargo-deny-deps-family-cutover.md`
  - `.worklogs/2026-03-25-004019-runtime-applicability-and-rs-ast-split.md`
  - `.worklogs/2026-03-25-011916-runtime-shim-and-toolchain-cutover.md`
- Family-owned source and tests:
  - `apps/guardrail3/crates/app/rs/families/code/src`
  - `apps/guardrail3/crates/app/rs/families/garde/src`
  - `apps/guardrail3/crates/app/rs/families/hexarch/src`
  - `apps/guardrail3/crates/app/rs/families/release/src`
  - `apps/guardrail3/crates/app/rs/families/arch/src`

## Open Questions / Future Considerations
- The remaining split blocker is the shared shell parser / hook-shell substrate that `test`, `hooks-shared`, and `hooks-rs` still need.
- The root facade and legacy root test topology are still broader than the final target. This batch improved family isolation, but it did not yet dismantle the root `tests/unit.rs` harness.
- There are still many dirty, older, uncommitted compatibility-path edits elsewhere in the repo. This commit should stage only the family-crate batch and the worklog, not the unrelated workspace noise.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/src/lib.rs` — family-owned `arch` entrypoint after the sweep
- `apps/guardrail3/crates/app/rs/families/code/src/lib.rs` — family-owned `code` entrypoint after the sweep
- `apps/guardrail3/crates/app/rs/families/garde/src/lib.rs` — family-owned `garde` entrypoint after the sweep
- `apps/guardrail3/crates/app/rs/families/hexarch/src/lib.rs` — family-owned `hexarch` entrypoint after the sweep
- `apps/guardrail3/crates/app/rs/families/release/src/lib.rs` — family-owned `release` entrypoint after the sweep
- `apps/guardrail3/crates/app/rs/families/release/src/facts.rs` — fixed package/workspace inheritance and docs.rs semantics
- `apps/guardrail3/crates/app/rs/families/release/src/release_support.rs` — fixed normalized manifest-relative path resolution
- `apps/guardrail3/crates/app/rs/families/hexarch/src/facts.rs` — fixed broken-symlink app discovery behavior
- `.worklogs/2026-03-25-021238-cargo-deny-deps-family-cutover.md` — prior non-shell family sweep context
- `.worklogs/2026-03-25-015759-fmt-and-clippy-family-cutover.md` — prior planned-order sweep context

## Next Steps / Continuation Plan
1. Commit this family sweep cleanly by staging only:
   - `apps/guardrail3/crates/app/rs/families/arch/**`
   - `apps/guardrail3/crates/app/rs/families/code/**`
   - `apps/guardrail3/crates/app/rs/families/garde/**`
   - `apps/guardrail3/crates/app/rs/families/hexarch/**`
   - `apps/guardrail3/crates/app/rs/families/release/**`
   - this worklog
2. Re-sync to the workspace-split plan and verify which shell-facing helpers are still sourced from legacy hook/check paths.
3. Extract the shared shell parser substrate out of the legacy hook/check tree.
4. Use that substrate to cut over:
   - `apps/guardrail3/crates/app/rs/families/test`
   - `apps/guardrail3/crates/app/rs/families/hooks-shared`
   - `apps/guardrail3/crates/app/rs/families/hooks-rs`
5. After the shell tranche, reassess the root facade and root test harness reduction, because at that point most Rust family logic will already be owned by independent crates.
