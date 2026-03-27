# RS-CLIPPY

Rust Clippy policy family.

This family enforces `clippy.toml` policy inside routed Rust roots. It does not own repo-global Rust root discovery or routing.

## What This Family Owns

`RS-CLIPPY` owns:

- allowed `clippy.toml` coverage for routed Rust units
- allowed `clippy.toml` placement and shadowing rules
- exact managed threshold values
- required disallowed method/type/macro baseline
- ban-entry reason quality
- duplicate ban detection
- managed-key typo detection
- profile-specific Clippy policy such as library global-state bans
- fail-closed handling when active Clippy inputs are unreadable or malformed

It does not own:

- repo-global Rust root placement
- Cargo lint-table policy in `Cargo.toml`
- generic source-structure rules
- test architecture

Those belong to:

- shared Rust `placement`
- shared Rust `FamilyMapper`
- `RS-CARGO`
- `RS-CODE`
- `RS-TEST`

## Shared Placement And Routing

This family must not decide which Rust roots are live.

It consumes:

- shared root scope from `placement`
- routed roots from `FamilyMapper::map_rs_clippy()`

Inside a routed root, the family may then do family-local discovery:

- allowed Clippy policy root selection
- `clippy.toml` / `.clippy.toml` parsing
- managed-key normalization
- per-root coverage and shadowing analysis
- profile-aware baseline comparison
- input failure collection

That split is intentional:

- `placement` decides what Rust roots exist
- `FamilyMapper` decides which roots reach `clippy`
- `clippy` decides Clippy-policy facts inside those routed roots

## Current Shape

At this checkpoint, `clippy` is still in the old single-crate family shape:

```text
apps/guardrail3/crates/app/rs/families/clippy/
  Cargo.toml
  src/
    lib.rs
    facts.rs
    inputs.rs
    clippy_support.rs
    test_support.rs
    rs_clippy_01_*.rs
    rs_clippy_01_*_tests/
      mod.rs
    ...
    rs_clippy_22_*.rs
    rs_clippy_22_*_tests/
      mod.rs
```

This means the family logic is alive, but the family is not yet self-hosted under the stricter `RS-TEST` contract.

## Target Shape

The target is the same self-hosted family pattern now used by `test`, `arch`, `cargo`, `hexarch`, and `code`:

```text
apps/guardrail3/crates/app/rs/families/clippy/
  Cargo.toml
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        clippy_support.rs
        rs_clippy_01_*.rs
        rs_clippy_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_clippy_01_*.rs
        ...
  test_support/
    Cargo.toml
    src/
      lib.rs
```

## Ownership Boundaries

### `crates/runtime`

Owns:

- family orchestration
- family-local Clippy policy discovery inside routed roots
- rule execution
- scenario setup in runtime sidecars

Must not own:

- reusable semantic assertions
- repo-global root discovery
- family routing

### `crates/assertions`

Owns:

- rule-local reusable semantic assertions
- proof-bearing exported assertion helpers used by runtime sidecars

May depend on:

- sibling `runtime` public API
- shared `test_support`

Must not own:

- route construction
- placement access
- scenario generation
- runtime private internals

### `test_support`

Owns only:

- generic tempdir/tree builders
- generic policy/config fixture helpers
- generic result helpers

Must not own:

- rule semantics
- expected `RS-CLIPPY-*` findings
- direct mapper/placement access
- direct runtime/assertions behavior

## Current Baseline

At the current checkpoint:

- the family unit tests pass
- the family passes `RS-ARCH`
- the family still fails `RS-TEST`
- the family still has a self-hit under `RS-CLIPPY-01`
- there is no family README-derived self-host migration yet

So the next work on `clippy` is not rule rescue first. It is:

1. document the family contract
2. migrate to the self-hosted runtime/assertions/test_support shape
3. make the family pass `RS-TEST`
4. then attack-review the live `RS-CLIPPY` rules the way `RS-CODE` and `RS-TEST` were hardened
