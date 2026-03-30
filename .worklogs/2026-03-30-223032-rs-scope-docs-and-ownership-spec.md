# Rust Family Hardening Checkpoint And Scope Architecture Spec

**Date:** 2026-03-30 22:30
**Scope:** Rust family docs and plans under `.plans/by_family/rs` and `.plans/todo/checks/rs`; shared Rust scope spec under `apps/guardrail3/crates/app/rs/README.md`; runtime and routing updates under `apps/guardrail3/crates/app/rs/runtime` and `apps/guardrail3/crates/app/rs/family_mapper`; family hardening across `clippy`, `toolchain`, `deps`, `test`, `release`, `cargo`, `garde`, and supporting walker/runtime crates

## Summary
This checkpoint captures two threads that converged in the same tree state: continued hardening of several Rust families against scoped-run false positives and fail-open behavior, and a repo-level clarification of family scope architecture so future implementation can add a shared ownership layer instead of letting each family rediscover workspace/file ownership for itself.

The tree now includes stricter routing/runtime behavior, expanded adversarial test coverage, and updated docs that describe the intended split between global families, workspace-local families, and zoned workspace-local families. The code does not yet contain the new shared ownership crate, but the current spec and family docs were updated so that implementation can proceed from a stable contract instead of ad hoc family logic.

## Context & Problem
The Rust checker architecture has been moving from legacy family-local discovery toward shared `ProjectTree`-based discovery and typed routes. At the same time, several families still had real hardening gaps:

- scoped runs could leak sibling config
- families could fail open on malformed but parseable inputs
- routing/runtime code still carried over-broad or root-only assumptions
- family docs still described inconsistent scope models such as mixed-scope or standalone-package ownership

During this session the user pushed on the architectural intent, not just green tests. The important requirement was:

- guardrails should be strict and hard to bypass
- no family should silently stop seeing files it owns
- root/topology legality belongs in shared layers and `arch`
- family-owned config legality belongs in the family
- docs must describe the real intended model before more implementation proceeds

That led to a broader checkpoint commit rather than a narrow family-only patch.

## Decisions Made

### Record the family scope model explicitly before implementing ownership
- **Chose:** write the new scope contract into `apps/guardrail3/crates/app/rs/README.md` and align primary family docs to it first.
- **Why:** implementation was already approaching a routing change, but without a frozen contract each family would likely drift into its own interpretation of discovery and ownership.
- **Alternatives considered:**
  - implement the ownership layer first and backfill docs later — rejected because the scope contract was still being actively clarified with the user
  - leave family docs inconsistent until each family is migrated — rejected because that invites incompatible local fixes

### Separate shared facts from family legality judgments
- **Chose:** define the shared layers as:
  - `placement` for workspace/root topology facts
  - a new ownership/surface layer for family-relevant file discovery and file-to-workspace attachment facts
  - `FamilyMapper` as a packager of typed family routes
- **Why:** families need to see all relevant files, including misplaced ones, but they should not each reinvent path ownership logic.
- **Alternatives considered:**
  - let each family rediscover ownership from raw `ProjectTree` paths — rejected because it recreates drift and inconsistent legality decisions
  - hide illegal/out-of-place files by routing only legal workspaces — rejected because that makes family-owned stray files invisible

### Classify families by actual policy scope
- **Chose:** document the family classes as:
  - global: `arch`, `fmt`, `code`, `test`
  - workspace-local: `toolchain`, `clippy`, `deny`, `cargo`, `garde`, `deps`, `release`
  - workspace-local under zone: `hexarch` under `apps/*`, `libarch` under `packages/*`
- **Why:** this matches the intended ownership model more closely than the previous mixed-scope and standalone-package descriptions.
- **Alternatives considered:**
  - keep `deps` mixed-scope — rejected because a truly global dependency policy collapses to the weakest common denominator across the repo
  - keep `release` mixed or repo-global — rejected because the actual release unit is the workspace/app, not the whole repo

### Move universal workspace topology rules toward `arch`
- **Chose:** update the `arch` detailed plan to absorb the generic workspace-shape rules that had been living in `hexarch`.
- **Why:** rules such as top-level roots must be workspaces, nested workspaces are forbidden, and nested crates must be explicit workspace members are topology rules, not app-shape rules.
- **Alternatives considered:**
  - leave them in `hexarch` — rejected because then package/library/tool areas would not share the same universal structural enforcement
  - push them into `cargo` — rejected because `cargo` should enforce Cargo policy inside already-accepted roots, not decide which root shapes are legal

### Treat this checkpoint as a broad tree snapshot
- **Chose:** commit the full current workspace state rather than carve out only the doc/spec edits.
- **Why:** the working tree already contains intertwined family hardening, runtime changes, and the new architecture docs; splitting it here would create an artificial and misleading history.
- **Alternatives considered:**
  - commit only the docs/spec files — rejected because the workspace already contains a much larger coherent checkpoint and the user explicitly requested to commit everything first

## Architectural Notes
- `ProjectTree` remains the shared snapshot. Families should not receive raw discovery responsibility.
- The missing implementation piece is a shared ownership/surface layer that:
  - discovers family-relevant files across the non-excluded tree
  - attaches each file to a legal workspace, or to “no workspace”, or to some structurally illegal relation
  - leaves legality judgments to `arch` and the family
- The intended route model is:
  - global families get global typed surfaces
  - workspace-local families get all legal workspaces plus all family-relevant files and precomputed attachment facts
- `clippy`, `toolchain`, `deps`, `test`, and `release` received additional adversarial coverage in this checkpoint around scope bleed, malformed inputs, and routed-run behavior.
- `family_mapper` and `runtime` already contain scope-related changes, but they are not yet the final ownership architecture. They should be treated as an intermediate step toward the explicit ownership layer described in the updated Rust README.

## Information Sources
- User direction in this session about:
  - one authoritative workspace topology model
  - moving generic workspace-shape rules out of `hexarch` and into `arch`
  - strict family scope classes
  - preventing invisible misplaced files
- Existing shared Rust docs and plans:
  - `apps/guardrail3/crates/app/rs/README.md`
  - `.plans/by_family/rs/README.md`
  - `.plans/todo/checks/rs/arch.md`
- Current implementation surfaces:
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/core/project_walker.rs`
  - `apps/guardrail3/crates/domain/project-tree/src/lib.rs`
- Prior same-day worklogs:
  - `.worklogs/2026-03-30-222258-unify-escape-hatch-policy.md`
  - `.worklogs/2026-03-30-211747-deny-second-hardening-pass.md`
  - `.worklogs/2026-03-30-210129-reason-policy-artifact-cleanup.md`

## Open Questions / Future Considerations
- The shared ownership/surface layer is specified but not implemented yet.
- Secondary or historical planning docs may still contain stale mixed-scope or standalone-package language and should be cleaned as follow-up if they are still part of active reading paths.
- The exact route/input type design for the ownership layer still needs to be translated from the README into code without bloating rule inputs.
- Hybrid top-level workspace roots (`[workspace]` plus `[package]`) remain a policy decision that should be settled as part of the broader `arch` topology migration.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — current source of truth for shared Rust family scope, routing, and the planned ownership layer
- `.plans/by_family/rs/README.md` — family scope classification index
- `.plans/todo/checks/rs/arch.md` — detailed migration plan for moving universal workspace-topology rules into `arch`
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — current mapper route construction that will need to consume ownership facts later
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — current runtime wiring that will need to integrate the ownership layer
- `apps/guardrail3/crates/app/core/project_walker.rs` — current `ProjectTree` population path
- `.worklogs/2026-03-30-222258-unify-escape-hatch-policy.md` — earlier same-day architectural checkpoint touching multiple Rust families
- `.worklogs/2026-03-30-211747-deny-second-hardening-pass.md` — recent adversarial hardening checkpoint that this broader snapshot builds on

## Next Steps / Continuation Plan
1. Clean any remaining active Rust family docs that still describe standalone-package ownership, mixed-scope behavior, or family-local rediscovery if those files are still part of the main handoff path.
2. Create the shared ownership/surface layer under `apps/guardrail3/crates/app/rs`:
   - define the file-kind and attachment fact types
   - collect family-relevant files from `ProjectTree`
   - attach them to workspace/topology facts from `placement`
3. Update `family_mapper` to consume ownership facts instead of only roots/scoped files.
4. Migrate one workspace-local family first, likely `toolchain` or `clippy`, onto the new route model and prove:
   - legal files are still seen
   - misplaced files no longer disappear
   - the family does not infer ownership from raw paths
5. After the ownership layer is in place, continue moving the generic workspace-topology rules from `hexarch` into `arch` and move the corresponding tests with them.
