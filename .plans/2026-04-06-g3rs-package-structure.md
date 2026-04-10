# G3RS Package Structure Plan

## Purpose

Freeze the concrete package layout for the new `g3rs` stack before building
`g3rs-workspace-crawl`, the `g3rs-*-ingestion` line, and the later minimal
Rust-only app.

This plan is deliberately based on the packages that already exist and compile,
especially:

- `packages/g3rs-garde-source-checks`
- `packages/g3rs-fmt-config-checks`

Those are the current structural specimens. The new packages should conform to
that line unless there is a specific reason to diverge.

## Existing Package Pattern To Preserve

The extracted `g3rs-*` packages are currently built as small internal
workspaces with the same outer shape:

```text
packages/<package-name>/
  Cargo.toml
  Cargo.lock
  README.md
  TODO.md
  src/lib.rs
  crates/
    assertions/
    runtime/
    types/
```

That pattern is already real in:

- `packages/g3rs-garde-source-checks`
- `packages/g3rs-fmt-config-checks`
- the other renamed `g3rs-*-config-checks` packages

The important structural properties are:

1. a public facade crate at the package root
2. a `types` crate with the public contract types
3. a `runtime` crate with the executable logic
4. an `assertions` crate for test helpers / result assertions
5. package-local `README.md` and `TODO.md`
6. package-local `Cargo.lock`
7. sidecar rule tests close to the runtime units

## Facade Convention

Every new `g3rs-*` package should have a root facade crate like this:

```text
packages/<name>/
  src/lib.rs
```

Responsibilities:

- re-export package logic under a feature gate
- re-export package public types under the same feature gate
- expose no internal implementation modules directly

Pattern:

```rust
#[cfg(feature = "api")]
pub use <runtime-crate>::<entrypoint>;
#[cfg(feature = "api")]
pub use <types-crate>::{...public types...};
```

Facade `Cargo.toml` pattern:

- root package owns:
  - `name`
  - metadata
  - workspace members
  - lint configuration
- dependencies on internal crates are optional
- `default = ["all"]`
- `all = ["api"]`
- `api = [dep:runtime, dep:types]`

This is the current live pattern in:

- `packages/g3rs-garde-source-checks/Cargo.toml`
- `packages/g3rs-fmt-config-checks/Cargo.toml`

## Types Crate Convention

Every package gets:

```text
crates/types/
  Cargo.toml
  README.md
  src/lib.rs
```

Responsibilities:

- define the public contract types only
- depend on parser/model crates as needed
- avoid runtime logic
- derive `Debug` / `Clone` where useful for tests and orchestration

The types crate is where stage boundaries are made explicit.

Examples:

- checks packages:
  - `G3RsFmtConfigChecksInput`
  - `G3RsGardeSourceChecksInput`
- future crawl package:
  - `G3RsWorkspaceCrawl`
  - `G3RsWorkspacePath`
  - `G3RsWorkspaceEntry`
- future ingestion package:
  - `G3RsFmtConfigIngestionInput`
  - `G3RsFmtConfigIngestionFailure`

### Types rule

Cross-package coupling should happen through `types` only.

That means:

- `g3rs-*-ingestion` depends on `g3rs-workspace-crawl` types
- `g3rs-*-ingestion` depends on `g3rs-*-checks` types
- checks packages do not depend back on crawl or ingestion

## Runtime Crate Convention

Every package gets:

```text
crates/runtime/
  Cargo.toml
  README.md
  src/lib.rs
```

The runtime crate owns the executable logic for that package and re-exports
that logic under a `checks`, `crawl`, or `ingest` feature.

Pattern:

```rust
mod run;

#[cfg(feature = "<logic-feature>")]
pub use run::<entrypoint>;
```

Allowed supporting modules:

- `support.rs`
- `inputs.rs`
- `parse/`
- `query/`
- `ignore.rs`
- `test_support.rs`

depending on the package role.

### Rule/test directory pattern for check packages

The existing extracted checks packages already use:

```text
crates/runtime/src/
  rs_<family>_<surface>_<nn>_<slug>/
    mod.rs
    rule.rs
    rule_tests/
      mod.rs
      ...
```

That should remain the standard for checks packages.

### Unit/test pattern for crawl and ingestion packages

Crawl and ingestion are not rule-ID packages, so they should not fake a rule
directory structure. Their runtime crates should instead group units by
responsibility:

```text
crates/runtime/src/
  lib.rs
  run.rs
  crawl.rs / ingest.rs
  support.rs
  <topic>.rs
  <topic>_tests/
    mod.rs
    ...
```

Examples:

- `g3rs-workspace-crawl`
  - `crawl.rs`
  - `ignore.rs`
  - `entries.rs`
  - `queries.rs`
  - `crawl_tests/`
  - `ignore_tests/`

- `g3rs-fmt-config-ingestion`
  - `ingest.rs`
  - `select.rs`
  - `parse.rs`
  - `support.rs`
  - `ingest_tests/`
  - `parse_tests/`

The key distinction is:

- checks packages keep one-rule-per-directory
- crawl/ingestion packages organize around operational units

## Assertions Crate Convention

Every package should keep an `assertions` crate:

```text
crates/assertions/
  Cargo.toml
  README.md
  src/lib.rs
  src/common.rs
  ...
```

Why keep it even for crawl/ingestion:

- it gives tests a shared result/assertion surface
- it keeps package-local test ergonomics consistent
- it avoids repeated ad hoc helper functions inside many test files

For checks packages, the assertions crate can stay one file per rule/surface.

For crawl/ingestion packages, the assertions crate should be unit-oriented:

- `workspace_entries.rs`
- `ignore_state.rs`
- `fmt_config_ingestion.rs`
- `garde_ast_ingestion.rs`

## Package-Specific Structures

### 1. `g3rs-workspace-crawl`

This is the only shared pre-check package.

Recommended shape:

```text
packages/g3rs-workspace-crawl/
  Cargo.toml
  Cargo.lock
  README.md
  TODO.md
  src/lib.rs
  crates/
    assertions/
      Cargo.toml
      README.md
      src/
        common.rs
        lib.rs
        workspace_entries.rs
        workspace_queries.rs
    runtime/
      Cargo.toml
      README.md
      src/
        lib.rs
        run.rs
        crawl.rs
        entries.rs
        ignore.rs
        queries.rs
        support.rs
        crawl_tests/
          mod.rs
          ...
        ignore_tests/
          mod.rs
          ...
        queries_tests/
          mod.rs
          ...
    types/
      Cargo.toml
      README.md
      src/
        lib.rs
```

Facade export:

- `crawl(...)`
- `G3RsWorkspaceCrawl`
- supporting path/entry types

Runtime logic feature:

- `crawl`

### 2. `g3rs-<family>-<surface>-ingestion`

All ingestion packages should follow one common pattern:

```text
packages/g3rs-<family>-<surface>-ingestion/
  Cargo.toml
  Cargo.lock
  README.md
  TODO.md
  src/lib.rs
  crates/
    assertions/
      Cargo.toml
      README.md
      src/
        common.rs
        lib.rs
        <family>_<surface>_ingestion.rs
    runtime/
      Cargo.toml
      README.md
      src/
        lib.rs
        run.rs
        ingest.rs
        select.rs
        parse.rs
        support.rs
        ingest_tests/
          mod.rs
          ...
        parse_tests/
          mod.rs
          ...
    types/
      Cargo.toml
      README.md
      src/
        lib.rs
```

Responsibilities by module:

- `select.rs`
  - choose files from `G3RsWorkspaceCrawl`
- `parse.rs`
  - parse selected files into typed models
- `ingest.rs`
  - assemble final checks input
- `support.rs`
  - local helpers that do not belong in public types
- `run.rs`
  - public runtime entrypoint

Facade export:

- `ingest(...)`
- ingestion input types
- ingestion failure types

Runtime logic feature:

- `ingest`

### 3. `g3rs-<family>-<surface>-checks`

These already exist and should stay aligned with the current extracted pattern:

```text
packages/g3rs-<family>-<surface>-checks/
  Cargo.toml
  Cargo.lock
  README.md
  TODO.md
  src/lib.rs
  crates/
    assertions/
    runtime/
    types/
```

Checks-specific rule:

- keep rule-directory runtime layout
- keep sidecar `rule_tests/`
- keep `run.rs` as the runtime entrypoint
- keep package facade exporting `check(...)`

The current baseline for this is:

- `packages/g3rs-garde-source-checks`
- `packages/g3rs-fmt-config-checks`

## Feature Naming Convention

The feature names should track the package role:

- crawl package:
  - `all`
  - `crawl`

- ingestion packages:
  - `all`
  - `ingest`

- checks packages:
  - `all`
  - `checks`

Facade feature:

- `api`

So the practical pattern becomes:

- facade:
  - `default = ["all"]`
  - `all = ["api"]`
  - `api = [dep:runtime, dep:types]`

- internal runtime crate:
  - `default = ["all"]`
  - `all = ["crawl" | "ingest" | "checks"]`
  - `<logic-feature> = []`

## Naming Convention

Package names:

- `g3rs-workspace-crawl`
- `g3rs-fmt-config-ingestion`
- `g3rs-fmt-config-checks`
- `g3rs-garde-ast-ingestion`
- `g3rs-garde-source-checks`

Public type names:

- `G3RsWorkspaceCrawl`
- `G3RsWorkspacePath`
- `G3RsFmtConfigIngestionInput`
- `G3RsFmtConfigIngestionFailure`
- `G3RsFmtConfigChecksInput`

Entrypoints:

- `crawl(...)`
- `ingest(...)`
- `check(...)`

## What Not To Do

Do not:

- invent a different package shape for crawl or ingestion
- skip `README.md` / `TODO.md`
- let ingestion packages call checks packages directly
- let checks packages depend on crawl or ingestion
- collapse runtime and types together “just for the first one”
- fake rule-style directories inside crawl/ingestion packages
- rebuild the old repo-wide legality/topology stack inside crawl

## Build Order

Build in this order:

1. `g3rs-workspace-crawl`
2. `g3rs-cargo-config-ingestion`
3. `g3rs-fmt-config-ingestion`
4. `g3rs-garde-ast-ingestion`
5. minimal `g3rs` app/orchestrator

Why this order:

- `workspace-crawl` freezes the shared filesystem semantics first
- `cargo-config-ingestion` proves the simplest config-ingestion shape
- `fmt-config-ingestion` proves a multi-file config ingestion package
- `garde-ast-ingestion` proves source-file ingestion against the same crawl
- the new app should come only after these package boundaries are stable

## Immediate Next Step

Use this package-structure plan together with
`.plans/2026-04-06-g3rs-workspace-crawl-and-ingestion.md` and scaffold
`packages/g3rs-workspace-crawl` in the same style as
`packages/g3rs-garde-source-checks`, but with `crawl`-oriented runtime modules
instead of rule directories.
