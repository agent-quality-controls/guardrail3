# Topology Selection And Wording Cleanup

**Date:** 2026-04-01 10:44
**Scope:** `apps/guardrail3/crates/app/rs/family_selection`, family READMEs, family test wording for cargo/clippy/toolchain

## Summary
Finished the last real topology coupling bug by stopping empty-request family resolution from always enabling `topology`. Then scrubbed the stale family docs/tests that still described routed legality as if it came from the topology family instead of the shared legality/topology substrate.

## Context & Problem
After the previous decoupling batch, explicit non-topology family runs were isolated correctly, but a test-attack review found one remaining automatic behavior: `topology` was still always enabled in the empty-request selection path. That meant the runtime contract was still inconsistent with the intended architecture even though explicit single-family runs were already fixed.

The same review also found stale wording in READMEs and family tests. The code behavior was mostly right, but the docs/tests were still teaching the wrong model:

- “topology should always be selected”
- “topology owns placement/root legality”
- “family passes `RS-TOPOLOGY`”

That wording would make the same architectural confusion reappear later.

## Decisions Made

### Respect config for topology in empty-request selection
- **Chose:** Remove the unconditional `Topology` special-case in `family_enabled_for_runtime(...)`.
- **Why:** Empty-request selection should use the same enablement model as other families. `topology` remains explicitly runnable even when disabled in config, but it should not be auto-enabled just because the runtime is resolving “enabled families.”
- **Alternatives considered:**
  - Keep topology always enabled for empty-request runs — rejected because it preserves a hidden special case after the family was intentionally decoupled.
  - Re-add a separate runtime-only force-enable path — rejected because it would recreate the same architectural confusion under a different implementation.

### Rewrite stale wording to point at shared legality instead of topology-the-family
- **Chose:** Update the family-selection assertions, targeted family READMEs, and test names/messages to say “shared legality” / “shared legality/topology substrate” instead of implying that non-topology families depend on the topology family.
- **Why:** The code no longer couples those families to topology. Leaving the stale wording would keep the wrong model alive in tests and docs and make regressions easier.
- **Alternatives considered:**
  - Change behavior only and leave wording for later — rejected because the stale wording was exactly what the user and test-attack pass identified as misleading.
  - Mass-sweep every mention of `topology` in all docs — rejected because most remaining mentions are legitimate references to the actual topology family.

## Architectural Notes
The current contract is now:

- shared substrate: `structure` + `legality` + `FamilyMapper`
- `topology`: independent checker family consuming that substrate
- non-topology families: consume the shared substrate directly, not the topology family

This batch closes the last remaining selection-layer exception. The previous batch had already removed explicit-family build/runtime coupling; this one removes the leftover “empty request always includes topology” behavior.

The wording cleanup is not cosmetic only. In this repo, tests and READMEs are a major architecture communication surface, so incorrect explanatory text is a real maintenance risk.

## Information Sources
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — remaining topology special-case
- `apps/guardrail3/crates/app/rs/family_selection/assertions/src/selection.rs` — stale “topology should always be selected” assertion wording
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` and prior runtime worklogs — prior explicit-run decoupling state
- `apps/guardrail3/crates/app/rs/families/*/README.md` — family ownership wording
- targeted family tests in `cargo`, `clippy`, and `toolchain` — stale “topology owns legality” messages
- `.worklogs/2026-03-31-231627-decouple-topology-from-family-runs.md` — previous decoupling batch this work builds on

## Open Questions / Future Considerations
- Some family READMEs still legitimately mention `topology` as a neighboring family or as the explicit reporter of topology findings. Those are fine as long as they do not imply hidden dependency.
- TS still uses `topology` as its own family name/surface. That is outside the active Rust-only direction and was not changed here.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — empty-request family resolution and enablement filtering
- `apps/guardrail3/crates/app/rs/family_selection/assertions/src/selection.rs` — selection contract assertions
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — representative example of the wording cleanup from “RS-TOPOLOGY” dependency language to shared legality language
- `apps/guardrail3/crates/app/rs/families/cargo/README.md` — same ownership wording cleanup for a routed family
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_12_allowed_placement_tests/unparseable_routed_root.rs` — representative stale test wording cleanup
- `.worklogs/2026-03-31-231627-decouple-topology-from-family-runs.md` — prior decoupling worklog that established the explicit-run isolation model

## Next Steps / Continuation Plan
1. If this area is touched again, keep using adversarial checks on both behavior and explanatory text: grep for “topology owns” / “passes `RS-TOPOLOGY`” outside the actual topology family.
2. Continue the larger migration by dismantling `libarch` into the new `arch` family, now that topology coupling is no longer muddying the ownership boundaries.
3. If future work revisits default “run enabled families” semantics, preserve the invariant that explicit family independence and empty-request config enablement use the same family model.
