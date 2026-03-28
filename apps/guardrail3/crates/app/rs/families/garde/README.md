# RS-GARDE

Rust garde boundary validation family.

This family is conditional per owned Rust root:

- `RS-GARDE-01` checks whether the root is actually using `garde`
- if `garde` is absent for that root, the ban and source-enforcement rules stay inactive
- if `garde` is present, the family enforces clippy-ban completeness plus AST-side boundary rules

## Owned Roots

`RS-GARDE` is a multi-root family.

It owns:

- Rust workspace roots
- standalone package roots that are not members of a workspace

Per owned root the runtime resolves:

- garde enablement from routed root policy
- the covering `clippy.toml` / `.clippy.toml`
- Rust source files belonging to that root

The family must not invent its own repo-root-only discovery model outside `RsGardeRoute`.

## Workspace Shape

```text
families/garde/
  Cargo.toml
  README.md
  crates/
    runtime/
      src/
        lib.rs
        discover.rs
        facts.rs
        garde_support.rs
        inputs.rs
        parse.rs
        rs_garde_01_*.rs
        ...
    assertions/
      src/
        lib.rs
        rs_garde_01_*.rs
        ...
  test_support/
    src/
      lib.rs
```

## Responsibilities

`runtime` owns:

- routed family orchestration
- per-root fact collection
- pure rule execution
- rule-specific sidecar tests
- family-local test fixtures that are specific to garde routing or garde policy fixtures

`assertions` owns:

- reusable result assertions per `RS-GARDE-*` rule

`test_support` owns:

- generic temporary-tree and `ProjectTree` helpers only

## What This Family Depends On

`RS-GARDE` depends on the `RS-CLIPPY` contract for covering-config resolution and canonical ban surfaces.

The active family contract is:

- [`.plans/todo/checks/rs/garde.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/garde.md)
- [`.plans/todo/checks/rs/clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md)
