# Strict Topology Rename Finalization

**Date:** 2026-03-31 21:45
**Scope:** `apps/guardrail3/crates/app/topology-helpers`, `apps/guardrail3/crates/app/ts/validate/topology`, `apps/guardrail3/crates/app/rs/validate/topology`, `apps/guardrail3/crates/app/rs/placement`, `apps/guardrail3/crates/app/rs/family_mapper`, `apps/guardrail3/crates/app/rs/runtime`, `apps/guardrail3/crates/domain/{config,report}`, `apps/guardrail3/tests/fixtures/full_golden`, `fuzz/Cargo.toml`, active READMEs and golden outputs`

## Summary
Finished the strict `arch` family rename beyond the earlier runtime/model pass. This batch renamed the remaining helper crate, TS legacy topology module, Rust legacy topology module, the shared fixture path, the last `arch_role` metadata use, and the remaining topology-family wording so the retired `arch` family name no longer survives anywhere live outside `.plans` and `.worklogs`.

## Context & Problem
The earlier commits (`9590ef16`, `04255c8f`) renamed the live Rust family from `arch` to `topology`, but the user explicitly wanted a stronger guarantee: no backward compatibility, no stale family naming anywhere live, TypeScript included, and the old topology golden fixture moved away from `r_arch_01`. A repo-wide grep showed that the remaining residue was broader than a few symbols: helper crate/package names still said `arch`, old TS/Rust legacy validate paths still lived under `architecture`, the fixture still lived under `tests/fixtures/r_arch_01/golden`, one auxiliary manifest still used `arch_role`, and multiple topology-family messages still described themselves as “architecture” checks.

## Decisions Made

### Rename the remaining live helper and legacy module surfaces instead of preserving compatibility
- **Chose:** Move `app/arch-helpers` to `app/topology-helpers`, `ts/validate/architecture/ts_arch_checks.rs` to `ts/validate/topology/ts_topology_checks.rs`, and the legacy Rust `app/rs/validate/architecture/*` files to `app/rs/validate/topology/*`.
- **Why:** Leaving those paths in place would keep the retired family name alive in live code discovery and keep teaching agents that `arch` still exists as a current product surface.
- **Alternatives considered:**
  - Keep the old paths and only rename public symbols — rejected because grep-based audits would still keep finding active `arch` family residue.
  - Leave legacy validate paths untouched because they are not the main runtime — rejected because the user explicitly wanted no backward compatibility and no live non-plan stale names.

### Move the golden fixture to a neutral name instead of preserving the old rule ID path
- **Chose:** Rename `apps/guardrail3/tests/fixtures/r_arch_01/golden` to `apps/guardrail3/tests/fixtures/full_golden` and update every in-repo consumer.
- **Why:** The old path encoded the retired family/rule naming directly into active tests. The fixture is shared and no longer belongs to one `arch`-named rule.
- **Alternatives considered:**
  - Keep the old fixture path as historical baggage — rejected because the user explicitly asked for `fixtures/full_golden`.
  - Duplicate the fixture and leave the old copy behind — rejected because that preserves backward compatibility and doubles maintenance.

### Rename topology-owned language from “architecture” to “topology”
- **Chose:** Rewrite topology-family README text, runtime findings, fail-closed messages, and legacy helper comments to say `topology` when they are describing the renamed family itself.
- **Why:** Even after symbol/path renames, wording like “Rust architecture required inputs” or “ambiguous architecture classification” still treated `architecture` as the live family identity.
- **Alternatives considered:**
  - Leave human-language strings alone because they are only messages/docs — rejected because the user wanted the family meaning cleared out completely before a future new `arch` family exists.
  - Rewrite every generic use of the English word “architecture” in the repo — rejected because many of those refer to actual software architecture concepts (`ArchUnit`, “architectural layers”) and are not the retired family name.

### Treat `arch_role` metadata as part of the family rename contract
- **Chose:** Rename the last remaining live metadata key usage in `fuzz/Cargo.toml` from `arch_role` to `topology_role`.
- **Why:** The broader grep showed that metadata still carried the old family term in one live manifest, and earlier rename work had already moved the code/config model to `topology_role`.
- **Alternatives considered:**
  - Ignore it as harmless manifest metadata — rejected because it is a live config key, not historical prose.

## Architectural Notes
This batch finishes the semantic separation that the earlier rename started:

- `topology` is now the only live global root/workspace-legality family name.
- The old `arch` family name no longer survives in active runtime wiring, helper crate/package names, legacy validator module paths, fixture paths, or topology-owned messages.
- The remaining non-plan/non-worklog `arch` hits are generic English or intentionally different concepts:
  - `hex arch` prose
  - `ArchUnit`
  - `architectural` descriptions
  - `hexarch` / `libarch`
  - archived external golden content from other repos

That matters because the next step in the broader migration is to introduce a genuinely new `arch` family for crate architecture. Leaving stale topology-era `arch` residue in current code would make that transition ambiguous.

## Information Sources
- Prior rename worklogs:
  - `.worklogs/2026-03-31-211227-rename-arch-to-topology.md`
  - `.worklogs/2026-03-31-212433-finish-topology-rename-cleanup.md`
- Strict grep passes run in this session:
  - targeted family-token grep over the repo excluding `.plans` / `.worklogs`
  - broader `arch` grep to identify whether residual hits were family-related or just generic English
- Primary files touched for the final cleanup:
  - `apps/guardrail3/Cargo.toml`
  - `apps/guardrail3/crates/app/topology-helpers/Cargo.toml`
  - `apps/guardrail3/crates/app/topology-helpers/src/lib.rs`
  - `apps/guardrail3/crates/app/ts/validate/mod.rs`
  - `apps/guardrail3/crates/app/ts/validate/topology/ts_topology_checks.rs`
  - `apps/guardrail3/crates/app/rs/validate/mod.rs`
  - `apps/guardrail3/crates/app/rs/validate/topology/*`
  - `apps/guardrail3/crates/app/rs/placement/src/{classification.rs,roots.rs}`
  - `apps/guardrail3/crates/domain/{config,report}/`
  - `apps/guardrail3/tests/fixtures/full_golden`
  - `fuzz/Cargo.toml`

## Open Questions / Future Considerations
- `README.md`, `GUARDRAIL3_GUIDE.md`, and some TS audit text still use phrases like `hex arch` and `architectural`. Those are now generic concepts, not stale family names. If you want those restyled too, that should be a separate terminology pass, not part of the old-family cleanup.
- The active repo is now ready for the next migration stage: creating a new real `arch` family without colliding with the old topology meaning.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — workspace member list updated to point at `topology-helpers`.
- `apps/guardrail3/crates/app/topology-helpers/src/lib.rs` — renamed helper crate and shared topology helper surface.
- `apps/guardrail3/crates/app/ts/validate/mod.rs` — TS validator now routes through `topology`, not `architecture`.
- `apps/guardrail3/crates/app/ts/validate/topology/ts_topology_checks.rs` — renamed TS topology module and IDs.
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy Rust validate surface now calls `run_topology_checks` and points at `validate/topology/*`.
- `apps/guardrail3/crates/app/rs/families/topology/README.md` — topology family wording updated to stop describing itself as `architecture`.
- `apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/{facts.rs,rs_topology_01_root_classification.rs,rs_topology_02_no_misplaced_roots.rs,rs_topology_03_no_dual_ownership.rs,rs_topology_07_required_inputs_fail_closed.rs}` — runtime messages renamed to topology terminology.
- `apps/guardrail3/tests/fixtures/full_golden` — new canonical shared golden fixture path.
- `fuzz/Cargo.toml` — last live metadata key updated from `arch_role` to `topology_role`.
- `.worklogs/2026-03-31-211227-rename-arch-to-topology.md` — earlier main rename checkpoint.
- `.worklogs/2026-03-31-212433-finish-topology-rename-cleanup.md` — earlier cleanup pass this batch finishes.

## Next Steps / Continuation Plan
1. If you want the repo fully free of the substring `arch` outside `libarch` / `hexarch`, do a separate terminology pass over generic prose like `hex arch`, `ArchUnit`, and `architectural`. That is not a family rename task anymore.
2. Start the new `arch` family on top of the now-clean `topology` namespace. Read the two prior topology rename worklogs plus this one before touching `libarch`.
3. When introducing the new `arch`, keep the same strict grep discipline used here: verify symbol/path/message/fixture/config names together instead of only renaming enum variants.
