# Shared Ownership Routing Migration

**Date:** 2026-03-31 00:21
**Scope:** `apps/guardrail3/crates/app/rs/ownership`, `apps/guardrail3/crates/app/rs/family_mapper`, `apps/guardrail3/crates/app/rs/runtime`, Rust family runtimes/tests for `toolchain`, `clippy`, `deny`, `cargo`, `deps`, `garde`, and `release`, plus workspace manifests/lockfile under `apps/guardrail3`

## Summary
Implemented the shared ownership routing pass across the Rust checker stack, then migrated the affected workspace-local families to the new contract. The work tightened subtree behavior, removed stale nested-workspace expectations from family suites, and reverified the families through package tests, lean-family check/build runs, and black-box CLI validation on temp fixtures.

## Context & Problem
The repo had already moved to per-family compilable units and a `ProjectTree`-based architecture, but the actual routing contract was still inconsistent. The mapper knew about routed roots, the families still rediscovered too much for themselves, and subtree runs could either leak sibling surfaces or go blind to malformed/misplaced family-owned files.

The user’s direction was stricter than the older implementation:
- shared layers own topology and attachment facts
- local families are workspace-root families, not package-root families
- nested workspaces are not a valid steady-state assumption
- subtree runs must stay workspace-wide for the owning workspace without leaking unrelated siblings
- green tests are not enough unless the adversarial cases are also pinned

That forced two kinds of changes:
- shared routing/ownership changes so families get a coherent view
- family-specific fact/test rewrites where the old suite still encoded the weaker multi-root or standalone-package model

## Decisions Made

### Added a shared ownership layer instead of widening mapper guesses
- **Chose:** Introduce and wire `apps/guardrail3/crates/app/rs/ownership` as the shared family-file discovery/attachment layer, then feed those facts through `family_mapper`.
- **Why:** The mapper needed globally discovered family-owned files plus attachment facts, but the families still needed to decide legality themselves. This preserves the fact-vs-judgment split.
- **Alternatives considered:**
  - Let each family rescan the tree — rejected because that reintroduces drift and duplicate topology logic.
  - Give families only routed roots — rejected because misplaced family-owned files become invisible.

### Kept workspace-local families workspace-local and repaired them locally where needed
- **Chose:** Keep `toolchain`, `clippy`, `deny`, `cargo`, `deps`, `garde`, and `release` on workspace-local routing, then update each family’s facts/discovery layer to derive the correct workspace-wide surface from the routed workspace roots.
- **Why:** The user explicitly rejected the older “standalone package roots everywhere” model. The correct fix was not to widen mapper routing back to packages, but to make each family derive member crates/configs from the owning workspace surface.
- **Alternatives considered:**
  - Reintroduce package-root routing in `family_mapper` — rejected because it fights the new arch direction.
  - Push all member/package rediscovery into mapper — rejected for now because it would couple family-specific semantics too tightly into shared code before the shape was proven.

### Updated tests to the stricter topology instead of preserving stale nested-workspace fixtures
- **Chose:** Rewrite failing `deps`, `garde`, and `release` fixtures/expectations that assumed nested workspaces were still legal roots.
- **Why:** The old fixtures were proving the wrong thing. Keeping them would lock the weaker architecture back in through tests.
- **Alternatives considered:**
  - Make the code continue to support nested-workspace expectations — rejected because that contradicts the current arch policy.
  - Delete the tests without replacement — rejected because the behaviors still needed coverage, just with valid workspace-root fixtures.

### Verified through both family-package tests and lean binary runs
- **Chose:** Re-run the affected family crates directly, then run lean `guardrail3` `check`/`build`/`run` flows for representative changed families.
- **Why:** Crate-local tests catch family logic regressions, but the user explicitly wanted proof that families still work when built alone through the lean binary path.
- **Alternatives considered:**
  - Stop at family crate tests — rejected because it does not prove the routed runtime/binary wiring.

## Architectural Notes
- `ownership` now acts as the shared discovery surface for family-owned files and attachments.
- `family_mapper` now routes workspace-local families through legal workspace roots plus attached family-file views.
- `runtime` was updated so global-vs-local family applicability matches the new model.
- `cargo`, `deps`, and `release` now rebuild member/package surfaces from the routed workspace roots instead of expecting the mapper to hand them package roots directly.
- `clippy` and `deny` were tightened so scoped runs stop bleeding into sibling areas while still staying fail-closed on active malformed inputs.
- `garde` and `deps` were adjusted so malformed workspace roots do not disappear just because the root could not be promoted into a routed workspace view.

## Information Sources
- `AGENTS.md`
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `.plans/by_family/rs/README.md`
- prior checkpoint commit `8131a276`
- family/runtime code and tests under:
  - `apps/guardrail3/crates/app/rs/ownership`
  - `apps/guardrail3/crates/app/rs/family_mapper`
  - `apps/guardrail3/crates/app/rs/runtime`
  - `apps/guardrail3/crates/app/rs/families/{toolchain,clippy,deny,cargo,deps,garde,release}`
- black-box CLI validation on temp fixtures using:
  - `guardrail3 --no-default-features --features family-clippy`
  - `guardrail3 --no-default-features --features family-deps`
  - `guardrail3 --no-default-features --features family-release`

## Open Questions / Future Considerations
- `release` still carries repo-root assumptions in `collect_repo_facts()` for `release-plz.toml`, `cliff.toml`, workflows, and license discovery. That is compatible with current tests and repo layout, but it is still broader than the newer workspace-local theory for deeper non-root workspaces.
- The shared ownership layer is still attachment-oriented, not full legality-aware workspace ownership. Families like `release` and `deps` still do some local workspace-member derivation. That is acceptable for this pass, but the next hardening step is to lift more of that into shared facts once the contract is frozen.
- Some older clippy attack findings were invalidated by later fixes in this same session, but subtree/runtime attack coverage there can still be expanded further.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — current shared Rust scope/ownership spec
- `apps/guardrail3/crates/app/rs/ownership/src/discover.rs` — shared family-file discovery and attachment facts
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — routed family mapping and workspace-root filtering
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — route/file attachment view types
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — global-vs-local runtime applicability
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/discover.rs` — cargo family workspace/member rediscovery under routed roots
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts.rs` — deps family fact collection
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts/workspaces.rs` — deps workspace/member resolution under the stricter model
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/facts.rs` — garde routed-root/policy handling
- `apps/guardrail3/crates/app/rs/families/release/src/facts/cargo_roots.rs` — release workspace-wide manifest discovery
- `apps/guardrail3/crates/app/rs/families/release/src/facts/collect.rs` — release crate/repo fact assembly
- `.worklogs/2026-03-30-223032-rs-scope-docs-and-ownership-spec.md` — previous doc/spec checkpoint this implementation built on

## Next Steps / Continuation Plan
1. Move the generic workspace-topology rules fully into `arch`, then delete the remaining family-local assumptions that still treat nested workspaces as meaningful shapes.
2. Lift more workspace ownership resolution into shared facts so families like `deps` and `release` do less local member/package derivation.
3. Revisit `release` repo-root policy discovery and decide whether non-root workspaces need first-class workspace-local release policy files or whether repo-root policy remains intentional.
4. Expand runtime-level subtree attack coverage for the migrated families, especially `clippy`, so the CLI-level scope expectations are pinned directly in the shared runtime suite.
