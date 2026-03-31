# Rust Structure Before Legality Routing

**Date:** 2026-03-31 14:07
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/{structure,legality,family_mapper,runtime,ownership}`, `apps/guardrail3/crates/app/rs/families/{release,libarch}`, shared Rust README, affected family test helpers

## Summary
This change finishes the architecture shift from separate `placement`/`ownership` thinking to one shared Rust structure stage followed by one shared Rust legality stage. Runtime, legality, and mapper now flow in that order, `libarch` now receives legal package members instead of only legal workspace roots, and stale `release`/`libarch` tests were rewritten so local families only prove behavior on legal routed surfaces.

## Context & Problem
The repo had already started moving toward legality-first routing, but the mental model and part of the implementation were still split incorrectly:
- `placement` and `ownership` existed as separate concepts even though they are both just pre-family structural evidence collection.
- docs still described old flows where families effectively consumed raw `placement`/`ownership` directly instead of one shared structure stage followed by legality.
- `libarch` still routed only legal workspace roots, which dropped legal package members under legal workspaces.
- `release` and `libarch` still had tests that assumed local families should see illegal non-members, loose top-level packages, nested workspaces, or other shapes that the legality-first pipeline now intentionally filters out.

The user requirement was explicit: structure must run before legality, structure must stay family-agnostic except for file-kind discovery, mapper/runners must slice from legality-aware facts, and local families must not regain global placement ownership through test backdoors.

## Decisions Made

### Shared Rust Flow Is `ProjectTree -> structure -> legality -> mapper -> runner -> family`
- **Chose:** Keep `placement` plus `ownership` as implementation details under a single `structure` stage and document/runtime-wire the system that way.
- **Why:** The old split was confusing the architecture. Both pieces do structural discovery and attachment; neither should be thought of as a separate semantic subsystem.
- **Alternatives considered:**
  - Keep `placement` and `ownership` as first-class architectural stages — rejected because it keeps the same conceptual muddle that caused the routing confusion.
  - Push legality into families — rejected because that recreates family-order dependence and local/global placement hacks.

### `libarch` Must Route Legal Package Members, Not Just Legal Workspace Roots
- **Chose:** Add legality-aware manifest-root routing for `libarch` in `family_mapper`, backed by legal `Cargo.toml` ownership facts, and extend ownership discovery so `libarch` gets `Cargo.toml` family-file facts.
- **Why:** `libarch` governs package roots under `packages/*`, including legal members of an enclosing workspace. Restricting it to legal workspace roots dropped real valid inputs like `packages/shared-types` inside the golden fixture.
- **Alternatives considered:**
  - Leave `libarch` workspace-root only and rewrite the fixture — rejected because it would be papering over a production routing bug.
  - Teach `libarch` to rediscover package members itself — rejected because ownership/routing must stay shared.

### Local-Family Tests Must Use Legal Routed Shapes or Direct Typed Inputs
- **Chose:** Rewrite stale `release` tests so illegal non-members and illegal root shapes are ignored by the local family, and convert the flat-library `RS-LIBARCH-01` tests to direct typed-input rule tests.
- **Why:** Those tests were proving old leakage behavior, not the new contract. The correct split is:
  - illegal topology/placement belongs to shared legality + `arch`
  - local family tests prove behavior on legal routed surfaces
  - pure rule semantics use direct typed inputs
- **Alternatives considered:**
  - Reintroduce synthetic local-family routing for tests — rejected because it would drift from production behavior.
  - Keep failing expectations and weaken legality filtering — rejected because it would undo the architecture change.

### Promoted Nested Workspace Fixtures Stay Illegal
- **Chose:** Keep the `libarch` golden promoted-shared-types nested-workspace cases quiet where the nested workspace remains present, but keep the post-removal case as a real `RS-LIBARCH-02` error once the file becomes a legal package member again.
- **Why:** This preserves the legality-first meaning of the fixture instead of treating every mutation as either fully visible or fully invisible.
- **Alternatives considered:**
  - Force all promoted-shared-types golden cases quiet — rejected because one mutation (`[workspace]` removed) returns the fixture to a legal local surface that `libarch` should still judge.
  - Rewrite the helper to delete the enclosing repo root workspace — rejected because the user asked to follow the new contract, not to hide important legality distinctions by mutating the fixture substrate.

## Architectural Notes
- `guardrail3_app_rs_structure` is now the architectural pre-family structure stage. `placement` and `ownership` are internal evidence collectors underneath it.
- `guardrail3_app_rs_legality` is the only shared Rust legality stage. It produces:
  - legal workspace roots
  - topology issues
  - legal family files
  - illegal family files
- `RS-ARCH` reports those legality facts; it is not the imperative “family that must run first.”
- `FamilyMapper` now depends on shared legality for local-family slicing.
- `libarch` now uses legal manifest-root routing instead of legal-workspace-root routing, which is important for legal package members beneath a workspace.
- `release` stays local-family only. Its old tests around undeclared non-members, loose roots, or unreadable root Cargo no longer belong in local routed-family behavior.

## Information Sources
- Runtime wiring:
  - `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
- Shared Rust structure and legality:
  - `apps/guardrail3/crates/app/rs/structure/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/ownership/src/discover.rs`
  - `apps/guardrail3/crates/app/rs/legality/src/lib.rs`
- Family routing:
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs_tests/mod.rs`
- Family implementations/tests:
  - `apps/guardrail3/crates/app/rs/families/release/src/...`
  - `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/...`
- Prior worklogs:
  - `.worklogs/2026-03-31-002143-shared-ownership-routing-migration.md`
  - `.worklogs/2026-03-31-125334-rs-legality-first-local-family-migration.md`

## Open Questions / Future Considerations
- The shared Rust README is now aligned with the structure-before-legality model, but some plan files under `.plans/todo/checks/rs/arch.md` and `.plans/by_family/rs/arch.md` still mention `placement/` more explicitly than the new conceptual model. They should be normalized to `structure -> legality`.
- `libarch` still has some routed tests that use the current repo golden fixture as a legal package-member substrate. If repo-global workspace banning becomes fully enforced in production fixtures, those tests should be revisited with purpose-built legal fixtures rather than the historical golden tree.
- More families may want the same “legal member manifest roots” treatment if they are package-root families rather than workspace-root families. Right now `libarch` is the concrete one that needed it.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/structure/src/lib.rs` — the shared Rust structure stage wrapper over root and owned-file discovery
- `apps/guardrail3/crates/app/rs/legality/src/lib.rs` — shared legal/illegal routing facts used by mapper and `arch`
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — legality-aware family surface slicing; includes the new `libarch` manifest-root routing
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — actual runtime pipeline ordering (`structure` before `legality`)
- `apps/guardrail3/crates/app/rs/ownership/src/discover.rs` — file-kind discovery; now includes `libarch` Cargo ownership facts
- `apps/guardrail3/crates/app/rs/families/release/src/test_fixtures.rs` — release routed-family test harness built on shared structure/mapping
- `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/rs_libarch_01_escalation_required_tests/mod.rs` — example of converting stale routed tests to direct rule-input tests
- `apps/guardrail3/crates/app/rs/README.md` — updated conceptual pipeline and acceptance criteria
- `.worklogs/2026-03-31-125334-rs-legality-first-local-family-migration.md` — prior legality-first routing migration that this work completes

## Next Steps / Continuation Plan
1. Normalize the remaining Rust plan docs to the same conceptual language used in `apps/guardrail3/crates/app/rs/README.md`: one shared structure stage, one legality stage, mapper surfaces, runner invocations.
2. Audit the other local families for any remaining routed tests that still expect illegal non-members or illegal top-level package roots to leak through local family routing. `release` and `libarch` were the visible holdouts; `hexarch` and any future package-root family should be checked next.
3. When the repo-wide top-level-workspace-only rules finish moving fully into shared legality/`arch`, replace any remaining golden-fixture dependence on legacy repo-root workspace layouts with purpose-built legal fixtures that match the final contract exactly.
