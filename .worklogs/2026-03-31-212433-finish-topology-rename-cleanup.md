# Finish Topology Rename Cleanup

**Date:** 2026-03-31 21:24
**Scope:** `.plans/todo/checks/rs/*`, `apps/guardrail3/crates/app/rs/README.md`, `apps/guardrail3/crates/app/rs/family_mapper/*`, Rust family test modules for `topology` / `toolchain` / `cargo` / `clippy`, `apps/guardrail3/crates/app/rs/validate/*`, `apps/guardrail3/crates/app/ts/validate/mod.rs`

## Summary
Finished the stale `arch` to `topology` cleanup that remained after the main family rename. This batch removed leftover standalone `arch` references from active Rust docs, mapper type names, test/assertion strings, and the unused legacy `app/rs/validate/arch` module path so the codebase no longer teaches two names for the same Rust family.

## Context & Problem
The prior rename commit (`9590ef16`) moved the live Rust family from `arch` to `topology`, but a follow-up audit with test-attack subagents still found stale standalone `arch` mentions. Some were harmless docs and test names, but one important residue remained in live mapper naming (`RsArchOverlapView`) and one more in legacy code (`app/rs/validate/arch`). Leaving those behind would keep confusing future agents and make grep-based audits unreliable.

## Decisions Made

### Rename the remaining live mapper symbols
- **Chose:** Rename `RsArchOverlapView` to `RsTopologyOverlapView` and update the shared Rust architecture README examples to match.
- **Why:** The mapper is part of the live production surface. Leaving `RsArch*` names there would undermine the family rename and keep stale type names in current code.
- **Alternatives considered:**
  - Leave the type name alone as an internal detail — rejected because it is exported from `family_mapper` and surfaced in docs/tests.
  - Rename only the docs — rejected because the code itself would still carry stale `arch` naming.

### Clean assertion and test language where topology owns the behavior
- **Chose:** Update test names and failure strings in `topology`, `toolchain`, `cargo`, and `clippy` to say `topology` rather than `arch`.
- **Why:** These tests describe ownership boundaries. If they keep saying `arch`, future refactors and audits will misread the actual family owner.
- **Alternatives considered:**
  - Ignore test strings because they do not affect runtime — rejected because the user explicitly asked for repo cleanup and the stale language obscures ownership.

### Rename the legacy `app/rs/validate/arch` module path
- **Chose:** Move the unused legacy module tree from `validate/arch` to `validate/topology` and rename its internal `rs_arch_01` module to `rs_topology_01`.
- **Why:** Even though the legacy code is not the active roadmap, it is still current code in the repo. Keeping `validate/arch` would leave a code-level stale family name behind after the rename.
- **Alternatives considered:**
  - Leave legacy code untouched — rejected because repo-wide grep would still keep finding a standalone `arch` family module.
  - Delete the legacy module entirely — rejected because this task was cleanup, not legacy-surface removal.

## Architectural Notes
The important structural change in this batch is the mapper rename: `topology` is now consistently the route/family name at the shared surface, including overlap views. The old family name survives only where it is semantically different (`libarch`, `hexarch`, `arch_role`, `arch-helpers`) or in historical planning documents that were not part of the active/live surface cleanup.

The legacy `app/rs/validate/topology` rename is intentionally narrow. It preserves the old implementation but stops reintroducing the retired family name into current code discovery.

## Information Sources
- Test-attack subagent audit results in this session for stale `arch` mentions.
- [apps/guardrail3/crates/app/rs/family_mapper/src/views.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/family_mapper/src/views.rs)
- [apps/guardrail3/crates/app/rs/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md)
- [apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/rs_topology_02_no_misplaced_roots_tests/enablement_matrix.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/rs_topology_02_no_misplaced_roots_tests/enablement_matrix.rs)
- [apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/rs_topology_16_workspace_local_file_placement_tests/mod.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/rs_topology_16_workspace_local_file_placement_tests/mod.rs)
- [apps/guardrail3/crates/app/rs/validate/topology/rs_topology_01/mod.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/validate/topology/rs_topology_01/mod.rs)
- Prior worklog: [.worklogs/2026-03-31-211227-rename-arch-to-topology.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.worklogs/2026-03-31-211227-rename-arch-to-topology.md)

## Open Questions / Future Considerations
- Historical plan files under `.plans/` still contain older `arch` wording in many places. I left those alone in this batch because they are history/reference material rather than live runtime/docs, but a later archival pass could normalize them too if you want grep silence across the entire repository history.
- `arch-helpers` still uses that crate name intentionally. Renaming that crate would be a separate decision because it is now more about legacy helper naming than family identity.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — live exported topology route/view types, including overlap view naming.
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — topology route construction from legality/structure facts.
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust architecture doc that agents use as the main handoff surface.
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy module map, now pointing at `topology` instead of `arch`.
- `apps/guardrail3/crates/app/rs/validate/topology/rs_topology_01/mod.rs` — renamed legacy topology module.
- `.worklogs/2026-03-31-211227-rename-arch-to-topology.md` — the main family rename worklog this cleanup completes.

## Next Steps / Continuation Plan
1. If you want true repo-history grep silence, do a separate historical-doc pass over `.plans/` to rename stale `arch` family references that are no longer the current contract.
2. If you want the new cross-crate `arch` family next, start only after the topology rename is fully stable and use the live `topology` docs as the baseline ownership surface.
3. Keep family-agent handoffs pointing at `topology`, not `arch`, and use the cleaned mapper README/types from this batch as the canonical naming surface.
