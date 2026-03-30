# RS-CODE

Rust source-code policy family.

This family enforces source-level Rust policy inside routed Rust roots. It does not own repo-global Rust root placement, app/package ownership, or hex-architecture boundaries.

## What This Family Owns

`RS-CODE` owns:

- source-file AST policy checks over routed `.rs` files
- lint-suppression and exception-comment policy
- source-structure thresholds and public-surface organization checks
- code-quality and bypass checks such as `include!`, `#[path]`, `panic!`, and direct `std::fs`
- test-only assertion quality checks such as useful `expect(...)` messages in test contexts
- fail-closed reporting when active source or code-policy inputs are unreadable or malformed

It does not own:

- repo-global Rust root placement
- app/package overlap classification
- app-internal hex structure
- Cargo workspace/member policy
- dependency-boundary architecture

Those belong to:

- `RS-ARCH`
- `RS-HEXARCH`
- `RS-CARGO`
- `RS-DEPS`
- shared Rust `placement`
- shared Rust `FamilyMapper`

## Shared Placement And Routing

This family must not decide which Rust roots are live.

It consumes:

- shared root scope from `placement`
- routed roots and file scope from `FamilyMapper::map_rs_code()`

Inside routed roots, the family may then do family-local discovery:

- Rust source-file enumeration
- source AST parsing
- guardrail and Cargo policy input parsing needed for code rules
- per-file and per-item normalization into rule inputs
- input-failure collection for active code-family surfaces

That split is intentional:

- `placement` decides what Rust roots exist
- `FamilyMapper` decides which roots and scoped files reach `code`
- `code` decides source-policy facts inside those routed roots

## Current Status

Current implementation state:

- the family root is a documentation/config container, not a Cargo workspace root
- production and tests live under [crates/runtime/src](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src)
- sibling crates are live and used:
  - [crates/assertions](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/assertions)
  - [test_support](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/test_support)
- the family consumes `RsCodeRoute` in [lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs)
- recent correctness work has focused on shared parser/model fixes captured in [FIXES.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/FIXES.md)
- this README does not claim the family root is currently clean for `RS-ARCH`, `RS-TEST`, or live repo-root `RS-CODE` without a fresh verification pass

The highest-value audit fixes already landed:

- shared test-context detection now feeds the rules that need it instead of path heuristics drifting apart
- same-line `// reason:` parsing is token-aware and exact
- bypass reasons must be nontrivial; weak placeholders still fail as errors
- `cfg_attr` truth is conservative and recursive instead of fail-open
- `#[expect(...)]` is owned with `#[allow(...)]`
- documented local bypasses stay visible instead of disappearing into inventory:
  - item-level `#[allow(...)]` / `#[expect(...)]` with `// reason:` are warnings
  - non-exempt `#[garde(skip)]` with `// reason:` is also a warning
- `garde(skip)` exemptions are explicit rather than suffix-based
- `RS-CODE-33` is the sole firing path for weak public error forms, and legacy `RS-CODE-25` stays non-firing to avoid overlap
- `RS-CODE-23` still allows legitimate `OUT_DIR` includes but no longer blesses upward traversal

Recent inventory completion also landed:

- `RS-CODE-31` forbids reachable public structs with named `pub` fields
- `RS-CODE-33` owns weak public error forms across `String`, `&str`, `anyhow::Error`, and `Box<dyn Error>`
- `RS-CODE-34` caps type/const generic parameter count at 6
- `RS-CODE-35` enforces per-root structural caps
- `RS-CODE-36` caps large string-dispatch sites in non-test code

The next work is still a mix of real repo debt cleanup and adversarial auditing:

- reduce live repo-root `RS-CODE` findings in real project files
- keep attacking implemented rules for false-greens and false-positives
- compare live repo findings against the intended rule inventory

## Live Package Shape

The live shape is a family-owned package group:

```text
apps/guardrail3/crates/app/rs/families/code/
  README.md
  FIXES.md
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        discover.rs
        facts.rs
        inputs.rs
        parse.rs
        rs_code_01_*.rs
        rs_code_01_*_tests/
          mod.rs
        ...
        rs_code_36_*.rs
        rs_code_36_*_tests/
          mod.rs
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_code_01_*.rs
        ...
        rs_code_30_*.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

## Ownership Boundaries

### `crates/runtime`

Owns:

- family orchestration
- routed source-file discovery inside mapped roots
- parse-once source and policy fact collection
- rule execution
- sidecar test scenario setup

Must not own:

- reusable semantic proof helpers
- repo-global root discovery
- production route construction

### `crates/assertions`

Owns:

- rule-local reusable semantic assertions
- proof-bearing assertion helpers called by runtime sidecars

May depend on:

- sibling `runtime` public API
- `test_support`

Must not own:

- mapper or placement wiring
- scenario generation
- runtime private internals

### `test_support`

Owns only:

- generic tempdir/tree builders
- generic fixture copying and file-writing helpers
- generic scoped-run helpers that do not encode rule semantics

Must not own:

- expected `RS-CODE-*` contracts
- reusable result-shape assertions
- direct mapper/placement logic once the family is fully migrated

## Current Documentation Boundaries

This README is intentionally conservative:

- it describes the live package layout and ownership split
- it points at the active fix backlog in [FIXES.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/FIXES.md)
- it does not treat older stabilization-plan wording about a family-root workspace as current truth
