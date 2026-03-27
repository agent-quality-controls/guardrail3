# RS-HEXARCH

Rust app-internal hexagonal architecture family.

This family enforces app-local Rust structure inside routed app roots. It does not own repo-global root placement.

## What This Family Owns

`RS-HEXARCH` owns:

- required `crates/` presence for Rust apps
- exact top-level hex container shape
- directional inbound/outbound container shape
- loose-file and empty-container structure checks
- workspace member coverage for every live app-local Cargo root
- nested workspace prohibition inside one routed app
- dependency-direction and cross-app dependency rules
- source-level ports/adapter surface rules
- fail-closed dependency blocking when member manifests are malformed

For ports source surfaces, the contract is direct rather than heuristic:

- ports may define passive public DTO/error/type declarations
- ports may define `pub trait` contracts
- ports may define trait impls on passive types
- ports should not expose public free functions
- ports should not expose public inherent methods on concrete types

It does not own:

- repo-global misplaced Rust roots
- app/package overlap classification
- auxiliary-root declarations
- generic Cargo policy

Those belong to:

- `RS-ARCH`
- shared Rust `placement`
- shared Rust `FamilyMapper`
- `RS-CARGO`

## Shared Placement And Routing

This family must not decide which Rust roots are live.

It consumes:

- shared root scope from `placement`
- routed roots from `FamilyMapper::map_rs_hexarch()`
- explicit repo-level config surfaces from `FamilyMapper::map_rs_hexarch()` for the rules that truly own them:
  - root `Cargo.toml` for `RS-HEXARCH-11`
  - root `guardrail3.toml` for `RS-HEXARCH-15`

Inside a routed root, the family may then do family-local discovery:

- app hex roots
- workspace/member coverage across all live app-local Cargo roots
- dependency edges/cycles
- source facts

That split is intentional:

- `placement` decides what Rust roots exist
- `FamilyMapper` decides which app roots and repo-level support files reach `hexarch`
- `hexarch` decides app-local hex facts inside those routed roots, plus the explicit repo-level checks it owns

## Current Workspace Shape

This family is self-hosted under the same `RS-TEST` contract it enforces elsewhere.

```text
apps/guardrail3/crates/app/rs/families/hexarch/
  Cargo.toml
  crates/
    runtime/                   # family orchestrator + rule implementations
      Cargo.toml
      src/
        lib.rs
        facts.rs
        dependency_facts.rs
        source_facts.rs
        inputs.rs
        rs_hexarch_01_*.rs
        rs_hexarch_01_*_tests/
          mod.rs
        ...
        rs_hexarch_27_*.rs
        rs_hexarch_27_*_tests/
          mod.rs
    assertions/                # rule-owned reusable semantic assertions
      Cargo.toml
      src/
        lib.rs
        dependency_facts.rs
        rs_hexarch_01_*.rs
        ...
        rs_hexarch_27_*.rs
    assertions_common/         # current shared assertions-only result matchers
      Cargo.toml
      src/
        lib.rs
  test_support/                # generic tree/fs setup only
    Cargo.toml
    src/
      lib.rs
```

## Ownership Boundaries

### `crates/runtime`

Owns:

- family orchestration
- family-local fact collection inside routed roots
- rule execution
- sidecar test scenario setup

Must not own:

- reusable semantic assertion contracts
- repo-global root discovery
- family routing

### `crates/assertions`

Owns:

- rule-local reusable semantic assertions
- proof-bearing assertion helpers called by runtime sidecars

May depend on:

- sibling `runtime` public API
- `assertions_common`
- `test_support`

Must not own:

- route construction
- placement access
- scenario generation
- runtime private internals

### `crates/assertions_common`

Current intended role:

- shared assertions-only result matchers used by multiple `hexarch` rules

This crate exists because `hexarch` currently has a large repeated result-shape surface, and keeping those matchers in `test_support` would violate `RS-TEST` genericity.

This is a current implementation shape, not a permanently blessed architectural primitive. The next `hexarch` audit should still verify that this crate is justified and stays narrower than `test_support`.

It must stay:

- assertions-only
- free of route construction and family discovery
- free of scenario builders

### `test_support`

Owns only:

- generic `ProjectTree` builders
- generic filesystem/tree setup helpers

Must not own:

- rule semantics
- expected `RS-HEXARCH-*` contracts
- direct calls into sibling `runtime` / `assertions`

## Self-Hosting Expectations

For this family itself:

- every `RS-HEXARCH-*` production rule lives in exactly one runtime file
- every production rule has exactly one rule-specific sidecar test directory
- every production rule has a sibling assertions module
- sidecars must prove through owned assertions helpers
- external root scope and route construction stay outside the family except for narrowly-scoped test entrypoints

## Current Status

At the current checkpoint this family:

- passes `RS-ARCH`
- passes `RS-TEST`
- is routed through shared `placement` + `FamilyMapper`

The next work on `hexarch` is no longer basic self-hosting. It is architecture tightening:

- verify the family does not still own scope it should receive externally
- verify `assertions_common` stays a legitimate assertions-only helper crate
- verify runtime test-only helpers do not backdoor route/placement behavior into assertions
- keep the documented rule inventory aligned with the live runtime rule set
