# Rename Arch To Topology

**Date:** 2026-03-31 21:12
**Scope:** `apps/guardrail3/crates/domain/validation-model`, `apps/guardrail3/crates/domain/config`, `apps/guardrail3/crates/domain/report`, `apps/guardrail3/crates/adapters/inbound/cli`, `apps/guardrail3/crates/app/commands`, `apps/guardrail3/crates/app/rs/runtime`, `apps/guardrail3/crates/app/rs/family_mapper`, `apps/guardrail3/crates/app/rs/family_selection`, `apps/guardrail3/crates/app/rs/families/topology`, active Rust family READMEs, active by-family plans`

## Summary
Renamed the current global Rust `arch` family to `topology` across the live product surface, runtime wiring, mapper APIs, family code, and active agent-facing docs. Verified that the renamed family compiles, its unit corpus passes, runtime tests stay green, and a lean `guardrail3 --family topology` run emits a `topology` section end to end.

## Context & Problem
The repo had already started the rename by moving the family directory from `families/arch` to `families/topology`, but the migration was half-landed. The enum, config toggles, report names, CLI family name, runtime helpers, mapper routes, family-local rule internals, and multiple active docs still said `arch`. That left the project in exactly the confusing state the user wanted to avoid before introducing a brand-new future `arch` family for crate-architecture concerns.

The immediate requirement was:
- finish the rename cleanly
- verify the product still works
- make the live handoff/docs surface stop teaching `arch` as the current topology family

## Decisions Made

### Rename The Live Family Completely Instead Of Preserving Backward-Compatible Aliases
- **Chose:** Move the live user-facing name, config key, CLI flag, report section name, and runtime family variant fully to `topology`.
- **Why:** The user explicitly wants old `arch` out of the way before a new architecture-focused family is created. Keeping aliases like `--family arch` or `arch = true` would preserve the ambiguity.
- **Alternatives considered:**
  - Preserve `arch` as a deprecated alias — rejected because it keeps two meanings of `arch` alive at once.
  - Rename only docs and leave code/config as `arch` — rejected because it would not actually free the namespace.

### Rename Shared Runtime And Mapper APIs Alongside The Family
- **Chose:** Rename `RustValidateFamily::Arch` to `RustValidateFamily::Topology`, `map_rs_arch()` to `map_rs_topology()`, `RsArchRoute`/`RsArchRootView`/`RsArchTopologyIssueKindView` to `RsTopology*`, and the topology-family facts/input types away from `Arch*`.
- **Why:** Leaving old route/type/helper names would preserve the old family meaning in the active architecture layer and make future `arch` work harder to reason about.
- **Alternatives considered:**
  - Only rename the enum and CLI surface — rejected because internal `map_rs_arch` / `RsArchRoute` names would continue teaching the old model.
  - Wait to rename internals until later — rejected because the user specifically wants the old `arch` concept cleared out first.

### Treat Topology Unit Failures As Fixture Drift, Not Logic Regressions
- **Chose:** Update topology-family fixtures from `arch = ...` to `topology = ...` rather than weakening the parser or adding compatibility logic.
- **Why:** The failing tests were stale by design after the config-key rename. The renamed parser correctly rejected the old key, so the right fix was to update the fixtures.
- **Alternatives considered:**
  - Accept both `arch` and `topology` in config temporarily — rejected because it undermines the namespace cleanup.
  - Skip topology-family tests and rely on `cargo check` only — rejected because this rename touched config parsing and runtime behavior directly.

### Update Active Agent-Facing Docs In The Same Commit
- **Chose:** Rewrite the active family READMEs and by-family plan files that still referred to `arch` as the live topology family.
- **Why:** The repo is being used for parallel agent handoffs right now. Leaving active plans/help text stale would immediately reintroduce the old mental model.
- **Alternatives considered:**
  - Defer docs to a later cleanup commit — rejected because agents would keep receiving contradictory instructions in the meantime.

## Architectural Notes
- The current global legality/topology family is now explicitly `topology`.
- The runtime still forces this family on for Rust runs, just under the new name.
- The config key is now `[rust.checks].topology`, not `arch`.
- Lean runtime builds now use `family-topology`.
- The family mapper now exposes topology-specific routing types instead of `arch`-named ones.
- The topology family still owns the same legal-root/workspace-topology policy as before; this change is naming and product-surface cleanup, not a semantics rewrite.
- This clears the namespace for a later new `arch` family focused on crate architecture rather than repo topology.

## Information Sources
- Recent worklogs that set the state before this rename:
  - `.worklogs/2026-03-31-200335-arch-workspace-membership-exactness.md`
  - `.worklogs/2026-03-31-204311-runtime-exactness-separation-proof.md`
  - `.worklogs/2026-03-31-205649-arch-topology-libarch-migration-plan.md`
- Migration handoff:
  - `.plans/todo/arch-topology-libarch-migration-handoff.md`
- Primary implementation files:
  - `apps/guardrail3/crates/domain/validation-model/src/families.rs`
  - `apps/guardrail3/crates/domain/config/types.rs`
  - `apps/guardrail3/crates/domain/report/mod.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`
  - `apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/lib.rs`

## Open Questions / Future Considerations
- `arch-helpers` remains named for “hex arch” structure helpers. That is a separate concern from the renamed Rust family and was left alone.
- `arch_role` metadata also remains unchanged because it is existing manifest metadata, not the family name.
- After this rename, the next structural step is the planned split:
  1. keep `topology` as the repo-global root/workspace legality family
  2. introduce a brand-new `arch` family for crate facade/privacy/dependency-boundary rules
  3. then dismantle `libarch` into that new architecture family

## Key Files for Context
- `apps/guardrail3/crates/domain/validation-model/src/families.rs` — canonical Rust family enum and ordering
- `apps/guardrail3/crates/domain/config/types.rs` — Rust check toggle keys, including the rename from `arch` to `topology`
- `apps/guardrail3/crates/domain/report/mod.rs` — CLI/config/section naming for Rust families
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — runtime orchestration and family forcing behavior
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — per-family runtime dispatch wiring
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — topology route construction
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — topology route/view types
- `apps/guardrail3/crates/app/rs/families/topology/crates/runtime/src/lib.rs` — topology family entrypoint
- `apps/guardrail3/crates/app/rs/families/topology/README.md` — current family-local contract after the rename
- `.plans/by_family/rs/topology.md` — current agent handoff for the topology family
- `.plans/todo/arch-topology-libarch-migration-handoff.md` — broader migration sequence for new `arch` / old `libarch`
- `.worklogs/2026-03-31-205649-arch-topology-libarch-migration-plan.md` — prior planning context

## Next Steps / Continuation Plan
1. Start introducing the new real `arch` family only after all family-agent handoff templates and CI invocations are using `topology`.
2. Use `.plans/todo/arch-topology-libarch-migration-handoff.md` as the execution order for:
   - new `arch` family creation
   - `libarch` rule triage
   - generalized facade/privacy/dependency-boundary migration
3. When creating the new `arch`, audit active docs for any remaining human-language ambiguity between:
   - topology/root legality
   - crate architecture/facade policy
