# RS-CLIPPY

Rust Clippy policy family.

This family enforces `clippy.toml` policy inside routed Rust roots. It does not own repo-global Rust root discovery or routing.

## What This Family Owns

`RS-CLIPPY` owns:

- allowed `clippy.toml` coverage for routed Rust units
- allowed `clippy.toml` placement and shadowing rules
- applicable cargo-config override surfaces that can redirect Clippy config discovery
- exact managed threshold values
- required disallowed method/type/macro baseline
- ban-entry reason quality
- duplicate ban detection
- managed-key typo detection
- profile-specific Clippy policy such as library global-state bans
- fail-closed handling when active Clippy inputs are unreadable or malformed
  - including malformed `guardrail3.toml` policy context used to resolve profile/garde behavior

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
- applicable `.cargo/config.toml` / `.cargo/config` discovery for forbidden `CLIPPY_CONF_DIR` overrides
- managed-key normalization
- per-root coverage and shadowing analysis
- profile-aware baseline comparison
- input failure collection

That split is intentional:

- `placement` decides what Rust roots exist
- `FamilyMapper` decides which roots reach `clippy`
- `clippy` decides Clippy-policy facts inside those routed roots

The `CLIPPY_CONF_DIR` override stays here even though it lives in Cargo config files, because the semantic question is still Clippy-specific: whether Clippy config discovery has been redirected away from the routed policy-root model. Generic Cargo lint-table and manifest policy remains owned by `RS-CARGO`.

## Current Shape

At this checkpoint, `clippy` uses the family-container shape under the app-root workspace and the sidecar migration uses sibling assertions plus external `test_support`:

```text
apps/guardrail3/crates/app/rs/families/clippy/
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
        rs_clippy_24_*.rs
        rs_clippy_24_*_tests/
          mod.rs
    assertions/
      Cargo.toml
      src/
        lib.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

This means:

- the family container is not a nested workspace root
- the runtime crate builds and the family is green under `RS-ARCH` and `RS-CLIPPY`
- rule sidecars now prove behavior through sibling assertions modules instead of runtime-local helper plumbing
- generic fixture setup now lives in the sibling `test_support` crate rather than a private runtime shim
- fresh top-level `RS-TEST` validation now depends on the app-root workspace only

## Target Shape

The target is the same self-hosted family pattern now used by `test`, `arch`, `cargo`, `hexarch`, and `code`:

```text
apps/guardrail3/crates/app/rs/families/clippy/
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
- narrowly scoped test-only owner helpers such as `run_for_tests(...)`

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
- the family passes `RS-CLIPPY`
- the repo-owned policy root is [`apps/guardrail3/clippy.toml`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/clippy.toml), and the family root no longer carries a local `clippy.toml`
- the family now fail-closes on applicable `CLIPPY_CONF_DIR` override surfaces in `.cargo/config.toml` / `.cargo/config`
- the family no longer has a runtime-local `test_support.rs` shim
- rule clusters `02..24` now use owner helpers plus sibling assertions modules
- adversarial fixture configs that need a literal `clippy.toml` are now materialized in tempdirs during tests instead of living as active repo policy roots

So the next work on `clippy` is not rule rescue first. It is:

1. rerun `RS-TEST` on the family once the outer workspace is healthy again and confirm the remaining buckets exactly
2. fix any leftover structural fallout if the validator still finds it
3. then attack-review the live `RS-CLIPPY` rules the way `RS-CODE` and `RS-TEST` were hardened
