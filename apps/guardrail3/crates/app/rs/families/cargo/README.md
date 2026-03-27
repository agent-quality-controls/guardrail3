# RS-CARGO

Rust Cargo policy family.

This family enforces workspace-level and member-level Cargo policy inside routed Rust roots. It does not own repo-global root placement.

## What This Family Owns

`RS-CARGO` owns:

- workspace lint table presence and shape
- lint level policy and allow inventory
- lint inheritance from workspace to members
- workspace metadata policy
- resolver policy
- member edition and rust-version drift checks
- missing member `Cargo.toml` fail-closed reporting
- member-local weakenings and unapproved allow entries
- fail-closed reporting when active Cargo inputs are unreadable or malformed

It does not own:

- repo-global Rust root placement
- app/package overlap classification
- app-internal hex structure
- generic source-code quality

Those belong to:

- `RS-ARCH`
- `RS-HEXARCH`
- `RS-CODE`
- shared Rust `placement`
- shared Rust `FamilyMapper`

## Shared Placement And Routing

This family must not decide which Rust roots are live.

It consumes:

- shared root scope from `placement`
- routed roots from `FamilyMapper::map_rs_cargo()`

Inside a routed root, the family may then do family-local discovery:

- workspace policy root selection
- workspace/member manifest parsing
- lint table normalization
- per-member policy comparison
- input failure collection

That split is intentional:

- `placement` decides what Rust roots exist
- `FamilyMapper` decides which roots reach `cargo`
- `cargo` decides Cargo-policy facts inside those routed roots

## Current Workspace Shape

This family is self-hosted under the same `RS-TEST` contract it enforces elsewhere.

```text
apps/guardrail3/crates/app/rs/families/cargo/
  Cargo.toml
  crates/
    runtime/                   # family orchestrator + rule implementations
      Cargo.toml
      src/
        lib.rs
        discover.rs
        facts.rs
        inputs.rs
        lint_support.rs
        rs_cargo_01_*.rs
        rs_cargo_01_*_tests/
          mod.rs
        ...
        rs_cargo_15_*.rs
        rs_cargo_15_*_tests/
          mod.rs
    assertions/                # rule-owned reusable semantic assertions
      Cargo.toml
      src/
        lib.rs
        common.rs
        rs_cargo_01_*.rs
        ...
        rs_cargo_15_*.rs
  test_support/                # generic tree/fs setup only
    Cargo.toml
    src/
      lib.rs
```

## Ownership Boundaries

### `crates/runtime`

Owns:

- family orchestration
- family-local Cargo fact collection inside routed roots
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
- `test_support`

Must not own:

- route construction
- placement access
- scenario generation
- runtime private internals

### `test_support`

Owns only:

- generic `ProjectTree` builders
- generic filesystem/tree setup helpers
- generic Cargo/TOML fixture helpers

Must not own:

- rule semantics
- expected `RS-CARGO-*` contracts
- direct calls into sibling `runtime` / `assertions`

## Self-Hosting Expectations

For this family itself:

- every `RS-CARGO-*` production rule lives in exactly one runtime file
- every production rule has exactly one rule-specific sidecar test directory
- every production rule has a sibling assertions module
- sidecars must prove through owned assertions helpers
- external root scope and route construction stay outside the family except for narrowly-scoped test entrypoints

## Current Status

At the current checkpoint this family:

- passes `RS-ARCH`
- passes `RS-TEST`
- is routed through shared `placement` + `FamilyMapper`

The next work on `cargo` is not basic self-hosting. It is contract and coverage tightening:

- verify routed-root scope stays the only active scope
- verify input-failure handling stays fail-closed for active Cargo surfaces
- verify the rule inventory and README stay aligned with the live runtime rule set
