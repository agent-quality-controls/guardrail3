# RS-CLIPPY

Rust Clippy policy family.

This family enforces `clippy.toml` policy inside legal workspaces. It does not
own repo-global Rust root legality, discovery, or routing.

## What This Family Owns

`RS-CLIPPY` owns:

- allowed `clippy.toml` coverage for legal workspaces
- allowed `clippy.toml` placement and shadowing rules
- applicable cargo-config override surfaces that can redirect Clippy config discovery
- exact managed threshold values
- required disallowed method/type/macro baseline
- ban-entry reason quality
- duplicate ban detection
- managed-key typo detection
- profile-specific Clippy policy such as library global-state bans
- positive inventory proof for clean policy state
- fail-closed handling when active Clippy inputs are unreadable or malformed
  - including malformed `guardrail3.toml` policy context used to resolve profile/garde behavior
  - including malformed allowed `clippy.toml` / `.clippy.toml`
  - including malformed routed `Cargo.toml` when coverage or placement depends on it

It does not own:

- repo-global Rust root placement legality
- Cargo lint-table policy in `Cargo.toml`
- generic source-structure rules
- test architecture

Those belong to:

- shared Rust `arch`/`placement`
- shared Rust `FamilyMapper`
- `RS-CARGO`
- `RS-CODE`
- `RS-TEST`

## Shared Placement And Routing

This family must not decide which Rust roots are legal.

It consumes:

- legality-aware workspace facts from `arch`/`placement`
- legal workspaces plus Clippy-relevant files from `FamilyMapper::map_rs_clippy()`

Inside a routed workspace, the family may then do family-local discovery:

- allowed Clippy policy root selection
- `clippy.toml` / `.clippy.toml` parsing
- applicable `.cargo/config.toml` / `.cargo/config` discovery for forbidden `CLIPPY_CONF_DIR` overrides
- managed-key normalization
- per-root coverage and shadowing analysis
- profile-aware baseline comparison
- input failure collection
- no pure-layer-specific Clippy baseline forks; pure-layer semantics stay owned by architecture checks

Malformed inputs are owned at the point where the rule depends on them:

- malformed allowed `clippy.toml` / `.clippy.toml` is owned by `RS-CLIPPY-25`
- malformed `guardrail3.toml` is owned by the policy-context rule
- malformed applicable `.cargo/config.toml` / `.cargo/config` override surfaces are owned by the Clippy override rule
- malformed routed `Cargo.toml` is fail-closed by `RS-CLIPPY-01` for coverage and by `RS-CLIPPY-12` for placement of attached configs

Positive inventory results are the normal "clean state" proof for this family. That includes the clean-path inventory emitted by `RS-CLIPPY-06` and `RS-CLIPPY-07` when there are no extra bans. They are not extra warnings and they are not emitted when the required input is broken.

That split is intentional:

- `arch` decides what Rust roots are legal
- `FamilyMapper` decides which legal workspaces and Clippy-relevant files reach `clippy`
- `clippy` decides Clippy-policy facts inside those routed workspaces

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
        facts/
          cargo.rs
          configs.rs
          policy.rs
        inputs.rs
        clippy_support.rs
        rs_clippy_01_*.rs
        rs_clippy_01_*_tests/
          mod.rs
        ...
        rs_clippy_25_*.rs
        rs_clippy_25_*_tests/
          mod.rs
    assertions/
      Cargo.toml
      src/
        lib.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
      fixtures.rs
      fs.rs
      mutations.rs
      tree.rs
```

This means:

- the family container is workspace-local and lives under the app workspace
- the runtime crate builds and the family is green under `RS-ARCH` and `RS-CLIPPY`
- the facts layer is split into `facts.rs` plus narrow helper modules under `facts/` so the family stays under repo-root file-length guardrails without weakening rule ownership
- rule sidecars now prove behavior through sibling assertions modules instead of runtime-local helper plumbing
- generic fixture setup now lives in the sibling `test_support` crate rather than a private runtime shim, and that crate is split into small helper modules for filesystem, tree-building, fixture scenarios, and TOML edits
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
        facts/
          cargo.rs
          configs.rs
          policy.rs
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
      fixtures.rs
      fs.rs
      mutations.rs
      tree.rs
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
- the family currently exposes 25 production `RS-CLIPPY-*` rules
- the repo-owned policy root is [`apps/guardrail3/clippy.toml`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/clippy.toml), and the family root no longer carries a local `clippy.toml`
- parseability of active Clippy configs is owned once by `RS-CLIPPY-25`, so threshold/baseline rules no longer fan out duplicate parse errors
- the family now fail-closes on applicable `CLIPPY_CONF_DIR` override surfaces in `.cargo/config.toml` / `.cargo/config`
- routed Cargo-root parse failures now fail closed instead of silently erasing policy roots
- the family no longer has a runtime-local `test_support.rs` shim
- rule clusters `02..25` now use owner helpers plus sibling assertions modules
- adversarial fixture configs that need a literal `clippy.toml` are now materialized in tempdirs during tests instead of living as active repo policy roots
- base type coverage is 21 paths, and library profiles add 4 global-state paths on top of that base
- pure-layer service roots do not change the managed Clippy baseline; those semantics are left to architecture checks
- `allow-expect-in-tests = true` is the only test relaxation kept on by default; the other test relaxations stay off

So the next work on `clippy` is maintenance, not rule rescue:

1. rerun `RS-TEST` on the family when the outer workspace is healthy and keep the family self-hosted layout green
2. keep `clippy.md`, `README.md`, and `domain/modules/clippy` aligned whenever the managed baseline changes
3. attack-review future `RS-CLIPPY` edits before landing them so fail-open drift does not accumulate again
