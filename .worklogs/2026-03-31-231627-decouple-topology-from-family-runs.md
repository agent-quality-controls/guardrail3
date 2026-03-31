# Decouple Topology From Family Runs

**Date:** 2026-03-31 23:16
**Scope:** `apps/guardrail3/crates/app/rs/family_selection`, `apps/guardrail3/crates/app/rs/runtime`, runtime test assertions, legacy topology comments, `apps/guardrail3/crates/app/hexarch-helpers`

## Summary
Removed the accidental coupling that forced the `topology` family into unrelated Rust family builds and reports. Updated the runtime tests to assert the new contract, fixed one hidden `cfg` regression exposed by the decoupling, and cleaned a small amount of rename fallout that still conflated `topology` with `hexarch`.

## Context & Problem
After creating the new live `arch` family, lean `arch` and `libarch` runs still emitted a `topology` section. That was architecturally wrong: `topology` is an independent family, while shared `structure` / `legality` / mapper routing are the substrate consumed by all routed families. The runtime had two separate stale couplings:

- build coupling in `runtime/Cargo.toml`, where most family features depended on `family-topology`
- selection coupling in `family_selection`, where `Topology` was always inserted into the selected family set

The user requirement was explicit: topology must be fully decoupled, and other families must not import or auto-run it just to get shared legality.

## Decisions Made

### Remove topology from non-topology feature chains
- **Chose:** Make every routed family depend on `routing` directly instead of depending on `family-topology`.
- **Why:** `routing` is the actual shared substrate. Depending on `family-topology` was using a checker family as a proxy for infrastructure, which violated the family separation model.
- **Alternatives considered:**
  - Keep `family-topology` as a prerequisite and hide the section — rejected because it would preserve the wrong compile-time coupling.
  - Introduce a second “internal topology” feature — rejected because `routing` already exists and is the correct shared layer.

### Remove unconditional topology auto-selection
- **Chose:** Delete the unconditional `selection.insert(RustValidateFamily::Topology);`.
- **Why:** The runtime should only run `topology` when it is explicitly requested or selected through config. Shared legality remains available through routing without forcing a visible topology report section.
- **Alternatives considered:**
  - Keep auto-selection and suppress the output section — rejected because it would still blur family boundaries and keep implicit runtime behavior.
  - Auto-select topology only for some families — rejected because it still treats topology as special substrate rather than an independent family.

### Rewrite runtime tests to the new contract
- **Chose:** Update runtime tests to assert that topology is absent during isolated non-topology runs, and rename the bad `hextopology_*` / `libtopology_*` test names.
- **Why:** The failing tests encoded the old wrong behavior. The decoupling work is not complete unless the test suite proves the new contract.
- **Alternatives considered:**
  - Delete the failing tests — rejected because the runtime needs explicit proof that isolated family runs stay isolated.
  - Leave the stale names and only change assertions — rejected because the names themselves preserved the wrong mental model.

### Keep legacy naming cleanup minimal and explicit
- **Chose:** Only touch the few legacy/support comments that still implied live `topology` ownership of hexarch structure.
- **Why:** The active runtime/test surface needed cleanup now; broad legacy churn was unnecessary for this batch.
- **Alternatives considered:**
  - Ignore legacy comments entirely — rejected because they were directly adjacent to the rename fallout being fixed.
  - Sweep all legacy docs/code for every topology/hexarch wording issue — rejected because that is a separate cleanup task.

## Architectural Notes
The runtime separation now matches the intended model:

- `routing` is the shared substrate: structure, legality, mapper, scoped routes
- `topology` is just one family consuming routed data
- `arch`, `libarch`, and other routed families consume `routing` directly
- no family needs `topology` the checker in order to compile or run

This fixes two independent layers:

1. Compile/build layer: non-topology family features no longer drag in the topology family crate.
2. Runtime selection layer: unrelated family runs no longer auto-insert a topology report section.

One hidden regression surfaced during this work: `runners.rs` had a `cfg` helper that had been accidentally relying on the old topology feature chain. Adding `family-arch` to the helper gate restored the lean `arch` build.

## Information Sources
- `apps/guardrail3/crates/app/rs/runtime/Cargo.toml` — existing feature dependency graph
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — forced topology selection
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` — stale always-on topology tests and rename fallout
- `apps/guardrail3/crates/app/rs/runtime/assertions/src/runtime.rs` — runtime assertion helpers
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — hidden `cfg` dependency exposed by decoupling
- `.worklogs/2026-03-31-223941-new-arch-family-cutover.md` — prior `arch` family introduction that exposed the coupling more clearly

## Open Questions / Future Considerations
- `libarch` still exists as a legacy family. The next architectural step is deciding how much more of it moves into the new `arch` family versus being deleted outright.
- There is still legacy `rs/legacy/validate` code carrying old topology/hexarch structure semantics. It is now out of the live workspace path, but future cleanup can simplify or archive it further.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/family_selection/src/selection.rs` — selected-family resolution; the topology auto-insert was removed here
- `apps/guardrail3/crates/app/rs/runtime/Cargo.toml` — family feature graph; non-topology features now depend on `routing` directly
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — routed helper gating; contains the `family-arch` cfg fix
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` — runtime proof tests for isolated family runs and the exact rename fallout cleaned in this batch
- `apps/guardrail3/crates/app/rs/runtime/assertions/src/runtime.rs` — `assert_section_absent` helper added for isolation assertions
- `.worklogs/2026-03-31-223941-new-arch-family-cutover.md` — prior worklog for the new `arch` family cutover that set the stage for this correction

## Next Steps / Continuation Plan
1. Keep using lean `cargo run -p guardrail3 --no-default-features --features family-<family>` as the proof path for family isolation whenever new family work lands.
2. Audit remaining runtime tests for assumptions that shared legality must always appear as a visible topology section.
3. Continue dismantling `libarch` by moving any still-valid generic crate-architecture policy into the new `arch` family and deleting dead layered-shape rules.
