# RS-RELEASE

Rust release-readiness family.

This family enforces release and publishability policy inside legal
releaseable workspaces. It is workspace-local, not a repo-global release-unit
family.

## What This Family Owns

`RS-RELEASE` owns:

- per-crate publish metadata for publishable crates inside a releaseable
  workspace
- local release-edge checks between publishable crates in that workspace
- binary-release workflow detection for publishable binary crates
- release-family input-failure reporting when active manifests, release config, workflow YAML, or README content are unreadable or malformed

That may include shared release inputs when a routed workspace owns them:

- root license file presence
- `release-plz.toml` existence and baseline coverage/content
- `cliff.toml` existence and canonical git-cliff baseline
- parsed workflow checks for release-plz execution, publish dry-run execution, registry token wiring, binary release flow, and Linux target coverage
- publishable crate metadata such as description, license, repository, README existence/quality, keywords, categories, semver, docs.rs metadata, and include/exclude inventory
- local dependency-edge checks for publishability and interdependent version compatibility
- thorough-mode `cargo publish --dry-run` outcomes
- fail-closed ownership through `RS-RELEASE-12`

It does not own:

- repo-global Rust root placement
- family routing
- generic Cargo lint-table policy
- generic source-structure rules
- app/package architecture

Those belong to:

- shared Rust `placement`
- shared Rust `FamilyMapper`
- `RS-CARGO`
- `RS-CODE`
- `RS-ARCH`

## Shared Placement And Routing

This family must not decide which Rust roots or release-owned files are live.

It consumes:

- shared topology facts from `placement`
- legal workspaces plus release-owned files from `FamilyMapper::map_rs_release()`

Inside routed workspaces, the family may then do family-local work:

- Cargo root collection and workspace/member inheritance resolution
- repo-level release config parsing
- workflow parsing and normalization
- publishable crate fact collection
- local dependency-edge collection
- input-failure collection
- per-rule fan-out over repo, crate, edge, and failure inputs

That split is intentional:

- `placement` decides what Rust roots exist
- `FamilyMapper` decides which legal workspaces and release-owned files reach `release`
- `release` decides release-readiness facts inside that workspace-local scope

## Current Shape

This family is self-hosted with the runtime/orchestrator at the family root and sibling assertion/test-support crates:

```text
apps/guardrail3/crates/app/rs/families/release/
  Cargo.toml
  src/
    lib.rs
    facts.rs
    facts/
      cargo_roots.rs
      collect.rs
      inheritance.rs
      types.rs
    inputs.rs
    release_support.rs
    release_support/
      binaries.rs
      dependencies.rs
      workflows.rs
      workflows/
        analysis.rs
        detection.rs
        types.rs
    rs_bin_01_*.rs
    rs_bin_01_*_tests/
      mod.rs
    ...
    rs_pub_14_*.rs
    rs_pub_14_*_tests/
      mod.rs
    rs_release_12_*.rs
    rs_release_12_*_tests/
      mod.rs
  assertions/
    Cargo.toml
    src/
      lib.rs
      common.rs
      rs_bin_01_*.rs
      ...
      rs_release_12_*.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

## Ownership Boundaries

### Family root crate

Owns:

- family orchestration
- release fact collection inside routed roots
- typed rule inputs
- release-specific normalization helpers
- rule execution
- rule-specific sidecar tests

Must not own:

- repo-global root discovery
- family routing
- reusable assertion contracts

### `assertions/`

Owns:

- rule-local reusable semantic assertions
- proof-bearing helpers used by runtime sidecars

May depend on:

- sibling family crate public API
- `test_support`

Must not own:

- route construction
- placement access
- scenario generation
- runtime private internals

### `test_support/`

Owns only:

- generic tempdir/tree builders
- generic fixture file helpers
- stub tool-checker helpers

Must not own:

- rule semantics
- expected `RS-RELEASE-*`, `RS-PUB-*`, or `RS-BIN-*` findings
- direct runtime/assertions behavior

## Current Baseline

At the current checkpoint:

- the family is routed through shared `placement` and `FamilyMapper`
- the family exposes 29 production rules:
  - 12 `RS-RELEASE-*`
  - 14 `RS-PUB-*`
  - 3 `RS-BIN-*`
- the family fact model is split into repo facts, publishable-crate facts, release-edge facts, and input-failure facts
- `RS-PUB-09` runs only in thorough mode
- `RS-RELEASE-12` owns malformed or unreadable active inputs instead of letting other rules fail open
- workflow checks operate on parsed workflow structure plus release-family helper matching rather than raw substring search
- the family unit suite is green on the current tree

The main remaining work here is maintenance and later hardening, not broad migration rescue. The known later-hardening target is richer workflow execution semantics for `RS-RELEASE-05..07` and `RS-BIN-01..02`; baseline ownership and fail-closed behavior are already in place.
