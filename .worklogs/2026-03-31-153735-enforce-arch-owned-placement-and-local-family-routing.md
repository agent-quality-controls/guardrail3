# Enforce Arch-Owned Placement And Local Family Routing

**Date:** 2026-03-31 15:37
**Scope:** `.plans/by_family/rs/{clippy,fmt}.md`, `.plans/todo/checks/rs/{clippy,fmt}.md`, `apps/guardrail3/crates/app/rs/{ownership,legality,family_mapper,runtime}`, `apps/guardrail3/crates/app/rs/families/{arch,fmt,clippy,deps,deny}` and root/app rustfmt configs

## Summary
This checkpoint finishes the shift to shared legality-owned placement for the active Rust local families in this tree state. `arch` now reports misplaced family files (including fmt and clippy/garde-owned clippy files), while `fmt`, `clippy`, and `deps` were tightened so family-local runs no longer make repo-level placement legality decisions.

## Context & Problem
The project already had structure/legality routing and per-family routes, but parts of local families still contained placement judgment logic. That weakened the architecture because legality ownership could drift back into family code and tests could silently re-encode old behavior.

The required direction was stricter:
- shared structure + legality decide legal/illegal placement
- `arch` reports illegal placement
- workspace-local families receive legal local surfaces and enforce family policy only
- misplaced files remain visible through `arch`, not through local-family fallback logic

## Decisions Made

### Move fmt placement ownership fully to legality + arch route
- **Chose:** route `fmt` through `FamilyMapper::map_rs_fmt()` and shared legality instead of family-local tree placement scanning.
- **Why:** keep `fmt` content checks local, and move placement legality to shared stage + `RS-ARCH-16`.
- **Alternatives considered:**
  - Keep nested/root placement checks inside `fmt` — rejected because it duplicates legality ownership.
  - Hide misplaced files from both arch and fmt — rejected because legality violations must stay visible.

### Keep RS-CLIPPY-12 only for same-root conflict, not global placement
- **Chose:** remove `NotAllowedRoot` / `UnparseableCargoRoot` placement branches from clippy facts/rule flow; keep only same-root precedence conflict reporting.
- **Why:** local clippy should consume legality-approved roots; illegal placement should be arch-owned.
- **Alternatives considered:**
  - Preserve clippy forbidden-placement errors — rejected because it keeps placement legality inside local family logic.
  - Delete RS-CLIPPY-12 entirely — rejected because same-root conflict policy is still clippy-owned behavior on legal roots.

### Remove deps placement-derived RS-DEPS-11 inputs
- **Chose:** drop `collect_guardrail_placement_failures` and `collect_cargo_placement_failures` from deps facts.
- **Why:** these were local-family placement legality judgments; they now belong to shared legality + arch reporting.
- **Alternatives considered:**
  - Keep deps placement failures as “input failures” — rejected because it reintroduces mixed ownership.
  - Move these exact messages into deps in a renamed rule — rejected for same reason.

### Update docs/plans to match the implemented contract
- **Chose:** revise clippy family docs/plans so placement legality is arch-owned and local clippy behavior is scoped to legal routed surfaces.
- **Why:** docs were still describing older behavior and would cause future regressions.
- **Alternatives considered:**
  - Leave docs unchanged until later — rejected because stale docs were actively misleading.

## Architectural Notes
- Shared flow remains: `ProjectTree -> structure -> legality -> mapper -> runner -> family`.
- `RS-ARCH-16` remains the placement legality reporting surface for workspace-local family files.
- Local routes now act as legal-surface contracts; family code should not backfill legality from raw paths.
- Stale clippy placement tests that encoded old forbidden-location behavior were removed to avoid policy backsliding.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-03-31-125334-rs-legality-first-local-family-migration.md`
- `.worklogs/2026-03-31-140740-rs-structure-legality-routing.md`
- Runtime/mapping/legality code under:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/{rs.rs,views.rs,lib.rs}`
  - `apps/guardrail3/crates/app/rs/legality/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/ownership/src/{discover.rs,kinds.rs}`
- Family code under:
  - `apps/guardrail3/crates/app/rs/families/fmt/...`
  - `apps/guardrail3/crates/app/rs/families/clippy/...`
  - `apps/guardrail3/crates/app/rs/families/deps/...`
  - `apps/guardrail3/crates/app/rs/families/arch/...`

## Open Questions / Future Considerations
- `RS-ARCH-16` currently emits one finding per owning family for shared files (for example `clippy.toml` for clippy + garde). This is correct semantically but may need message grouping/dedupe later.
- Other local families should continue being audited for any remaining placement-judgment residue hidden behind “input failure” wording.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/legality/src/lib.rs` — canonical legal/illegal family-file ownership decisions
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — legality-aware local route construction
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — per-family route fanout wiring
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_16_workspace_local_file_placement.rs` — placement reporting owner
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/facts.rs` — fmt route-consumption model
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — clippy local facts after placement-branch removal
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts.rs` — deps input failure collection after placement-removal
- `.plans/by_family/rs/clippy.md` — updated clippy ownership intent
- `.plans/todo/checks/rs/clippy.md` — updated RS-CLIPPY-12 contract
- `.worklogs/2026-03-31-140740-rs-structure-legality-routing.md` — prior architecture checkpoint this builds on

## Next Steps / Continuation Plan
1. Run a wider attack sweep across remaining local families (`toolchain`, `cargo`, `deny`, `garde`, `release`) specifically for hidden placement ownership in facts/input-failure paths.
2. If shared-file duplicate reporting becomes operationally noisy, add grouped reporting policy in `arch` while preserving ownership facts.
3. Continue migrating plan docs so all family contracts consistently describe `structure -> legality -> mapper -> runner -> family` with no family-local legality fallback language.
