# Arch / Topology / Libarch Migration Handoff

## Goal

Split the current overloaded meaning of `RS-ARCH` into two distinct families,
in order:

1. rename the current `RS-ARCH` family to `RS-TOPOLOGY`
2. only after `arch` is fully vacated, introduce a new `RS-ARCH`
3. then dismantle `RS-LIBARCH` by moving its surviving generalized concerns
   into the new `RS-ARCH`

This sequencing is intentional. Do not create the new `arch` while the old
`arch` naming is still present, or the repo will end up with mixed meanings and
broken handoffs.

## Current Meaning Split

### What current `RS-ARCH` actually is

Current `RS-ARCH` is mostly topology/governance, not architecture in the new
sense.

It owns things like:

- root placement legality
- app/package overlap
- workspace shape legality
- exact workspace-membership equivalence
- global-only config for the family itself
- fail-closed placement/config inputs

That family should be renamed to `topology`.

### What the new `RS-ARCH` should become

The new `RS-ARCH` should own repo-wide Rust architectural boundaries, such as:

- facade/public-entry ownership
- internal crate privacy across crate boundaries
- “do not bypass facade crates to depend on internals directly”
- “do not expose internal crates in the public surface”
- generalized escalation from flat library/package shape into structured
  internal architecture, if that concept survives

### What `RS-LIBARCH` currently still does

After removing workspace-membership exactness, `libarch` still owns:

- flat library escalation into layered mode
- layered root facade/workspace shape
- `crates/` existence
- exact layered crate set
- layer direction rules (`core/api/infra`)
- root facade export policy

The explicit layered-shape rules are expected to die if that structure is no
longer policy.

The facade/privacy ideas should survive, but in the new `arch`, not in
`libarch`.

## Hard Sequencing Rule

Do the migration in these phases only:

1. rename old `arch` to `topology`
2. finish the rename fully:
   - code
   - rule IDs
   - family enum/model
   - CLI/runtime/selection
   - docs
   - plans
   - tests
   - output section names
3. verify there is no active “old arch” meaning left
4. create the new `arch`
5. start dismantling `libarch`

Do not overlap phases 1 and 4.

## Phase 1: Rename Current `arch` to `topology`

### Intended End State

Everything that is currently “old arch” becomes topology:

- family name
- rule IDs
- runtime section name
- docs/handovers
- CLI family selection surface

Examples:

- `RS-ARCH-01` -> `RS-TOPOLOGY-01`
- `RustValidateFamily::Arch` -> `RustValidateFamily::Topology`
- section name `arch` -> `topology`
- `families/arch/` -> `families/topology/`

### Scope of the rename

This is not just a directory rename.

It includes:

- family crate paths
- Cargo package names where applicable
- module names
- runtime dispatch
- domain validation model enum and parsing
- config/help/docs strings
- README and plan ownership language
- tests and assertions
- handoff files

### Files to audit first

- `apps/guardrail3/crates/app/rs/families/arch/`
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
- `apps/guardrail3/crates/app/rs/runtime/src/registry.rs`
- `apps/guardrail3/crates/app/rs/runtime/Cargo.toml`
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- `apps/guardrail3/crates/domain/validation-model/src/families.rs`
- `apps/guardrail3/crates/bin/guardrail3/src/main.rs`
- `apps/guardrail3/crates/app/rs/README.md`
- `.plans/by_family/rs/README.md`
- `.plans/by_family/rs/arch.md`

### Rules that move unchanged in meaning

Current topology rules should move as-is in concept:

- root classification
- misplaced roots
- dual ownership
- zone overlap
- scoped-config forbidden
- owner-family enablement coherence
- fail-closed required inputs
- auxiliary roots declared
- top-level root must be workspace
- no loose top-level packages
- no nested workspaces
- exact workspace-membership equivalence
- workspace member path must not escape root
- auxiliary top-level root must be workspace
- workspace-local family file placement

### Verification for phase 1

Must prove:

- `cargo check --manifest-path apps/guardrail3/Cargo.toml --quiet`
- renamed family still runs lean:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-topology -- rs validate apps/guardrail3 --family topology --format json`
- old family name no longer exists in the live product surface
- output section is `topology`, not `arch`
- no rule IDs starting with `RS-ARCH-` remain for the old topology family
- no docs still describe old-arch topology rules as “architecture” unless
  explicitly historical

### Completion gate for phase 1

Do not start the new `arch` until all of this is true:

- old topology family is fully renamed
- `arch` is no longer the live name for topology
- runtime/CLI/docs/tests all agree on `topology`

## Phase 2: Introduce the New `arch`

### Intended End State

A brand-new `RS-ARCH` family exists and means crate architecture, not topology.

This family should be repo-wide Rust architecture, not app-only and not
package-only.

It should own concerns like:

- facade/public-entry ownership
- internal crates must not be exposed directly
- consumers must not bypass facades to depend on internals
- public surface must go through designated public crates
- generalized architecture escalation if retained

### Important constraint

Do not begin by copying `libarch` wholesale.

First define the generalized concepts, then map surviving `libarch` rules into
them.

### Minimal first cut for new `arch`

The first new `arch` should probably start with generalized versions of:

- root/public facade crate exists and is the intended entrypoint
- internal implementation crates are not publicly re-exported
- public surface comes from the designated public/facade crate
- direct cross-crate deps to internal crates are forbidden from outside the
  owning boundary

Avoid reintroducing `api/core/infra` names into the new family.

## Phase 3: Dismantle `libarch` into New `arch`

### Rules to delete outright if layered shape is no longer policy

- current `RS-LIBARCH-03`
- current `RS-LIBARCH-04`
- current `RS-LIBARCH-07`
- current `RS-LIBARCH-08`
- current `RS-LIBARCH-09`

These are specific to the old layered structure.

### Rules to evaluate for generalization into new `arch`

- current `RS-LIBARCH-01`
  - generalize from “flat library must become layered workspace”
  - to “flat package/crate architecture must split once structural thresholds
    are exceeded”, if that policy still makes sense

- current `RS-LIBARCH-02`
  - generalize from “layered root must be workspace facade”
  - to “split package root must remain the facade/public entry package”

- current `RS-LIBARCH-10`
  - generalize from “infra must not be public surface”
  - to “internal implementation crates must not become public surface”

- current `RS-LIBARCH-11`
  - generalize from “root facade exports api, not core”
  - to “root facade exports from designated public crate, not internal crates”

### `libarch` end-state options

Choose one explicitly:

1. delete `libarch` entirely after migration
2. keep `libarch` as a tiny temporary compatibility family during transition
3. keep `libarch` only if a truly package-only structure policy remains that is
   not repo-global architecture

Given the current direction, option 1 is the expected target.

## Recommended Execution Order

1. Build a rename checklist for every current `arch` touchpoint.
2. Rename current `arch` to `topology` in code first.
3. Rename rule IDs and runtime/report section names.
4. Update CLI/family selection/config parsing/help text.
5. Update READMEs, by-family plans, and handoff files.
6. Verify lean `topology` family run and full workspace compile.
7. Run a repo-wide grep proving old live `arch` naming is gone except in
   historical notes.
8. Create the new `arch` family skeleton and README with the new meaning only.
9. Decide which `libarch` ideas survive conceptually.
10. Move generalized surviving concepts into new `arch`.
11. Delete obsolete `libarch` rules.
12. Remove `libarch` from runtime/model if the family is fully retired.

## Required Proofs

### After topology rename

- lean `topology` run works
- runtime section is `topology`
- old `arch` topology rule IDs are gone
- shared docs now describe topology vs architecture distinctly

### After new `arch` introduction

- new `arch` has a README that defines crate-architecture ownership only
- no topology rule remains in new `arch`
- no `api/core/infra`-specific assumptions are baked into new generic rules

### After `libarch` dismantling

- deleted layered-shape rules no longer appear in code or output
- surviving generalized ideas appear only in new `arch`
- no duplicate ownership between new `arch` and any remaining `libarch`

## Non-goals

Do not do these in the same pass:

- broad TS cleanup
- unrelated family routing work
- new app-only `hexarch` semantics
- arbitrary code-style cleanup

Keep this migration semantic and naming-focused.

## Acceptance Criteria

This migration is done only when:

- old `arch` has been fully renamed to `topology`
- a new `arch` exists with a different, explicit meaning
- `libarch` no longer owns obsolete layered-shape rules
- surviving facade/privacy/generalization rules live in new `arch`
- runtime, lean family runs, tests, and docs all agree on the new boundaries
