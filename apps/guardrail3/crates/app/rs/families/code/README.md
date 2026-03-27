# RS-CODE

Rust source-code policy family.

This family enforces source-level Rust policy inside routed Rust roots. It does not own repo-global Rust root placement, app/package ownership, or hex-architecture boundaries.

## What This Family Owns

`RS-CODE` owns:

- source-file AST policy checks over routed `.rs` files
- lint-suppression and exception-comment policy
- source-structure thresholds and public-surface organization checks
- code-quality and bypass checks such as `include!`, `#[path]`, `panic!`, `unwrap`, and direct `std::fs`
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

This family is mid-migration and not yet in the same stabilized self-hosted tier as:

- `RS-TEST`
- `RS-ARCH`
- `RS-HEXARCH`
- `RS-CARGO`

Current implementation state:

- the family root is now a workspace
- production and tests now live under [crates/runtime/src](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src)
- placeholder sibling crates now exist for:
  - [crates/assertions](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/assertions)
  - [test_support](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/test_support)
- the family already consumes `RsCodeRoute` in [lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs)
- the family passes `RS-ARCH`
- the family test suite is green from the moved runtime crate
- the family does not yet pass `RS-TEST`

At the current checkpoint after the workspace split:

- `RS-TEST-02`: `31` hits
- `RS-TEST-03`: `778` hits
- `RS-TEST-16`: `99` hits

So the next work is:

- stop sidecars from escaping their owned runtime/assertions boundary
- extract proof-bearing semantic assertions out of runtime sidecars
- replace runtime-local `test_support.rs` with the sibling `test_support` crate

## Target Workspace Shape

The target shape is the same self-hosted family layout already used by `test`, `arch`, `hexarch`, and `cargo`:

```text
apps/guardrail3/crates/app/rs/families/code/
  Cargo.toml
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
        rs_code_30_*.rs
        rs_code_30_*_tests/
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
- route construction

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

## Stabilization Sequence

The practical migration order is:

1. add this family README and a current stabilization plan
2. split the family into `crates/runtime`, `crates/assertions`, and `test_support`
3. move the existing `src/*` runtime implementation into `crates/runtime/src/`
4. replace runtime-local `test_support.rs` with the sibling `test_support` crate
5. extract proof-bearing rule assertions out of runtime sidecars
6. make the family pass `RS-TEST`
7. then attack `RS-CODE` itself the same way `RS-TEST` and `RS-HEXARCH` were attacked

## Done Means

`RS-CODE` is only in the stabilized tier when it:

- has the self-hosted workspace shape above
- passes `RS-ARCH`
- passes `RS-TEST`
- has family-local docs aligned with the live code
- no longer hides mapper/placement wiring inside family-local tests
- is ready for a deeper adversarial rule-family audit
