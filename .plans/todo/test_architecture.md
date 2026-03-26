# Test Architecture

```text
crates/
  x/                                  # one owned tested component
    runtime/                          # production crate for x behavior
      Cargo.toml                      # prod crate manifest; dev-depends on sibling assertions and test_support
      src/
        lib.rs                        # narrow public API only; external tests must go through this
        foo.rs                        # one production module
        foo_tests/                    # sidecar harnesses for foo.rs only; owns scenario generation, not shared semantics
          mod.rs                      # module entry for foo sidecar tests
          synthetic.rs                # typed/local input cases for foo
          edge_cases.rs               # private-access or local edge cases for foo
      tests/
        public_surface.rs             # black-box integration tests for x runtime public API only
        parser_backed.rs              # integration tests that cross a parser/fixture boundary but stay black-box
    assertions/                       # dev-only reusable semantic assertions over x/runtime public behavior
      Cargo.toml                      # depends on sibling runtime and optionally test_support; never the other way in prod
      src/
        lib.rs                        # exports assertion modules only
        foo.rs                        # reusable semantic assertions for foo output/behavior

  test_support/                       # optional shared generic test helpers
    Cargo.toml                        # dev-only helper crate
    src/
      lib.rs                          # shared helper exports
      builders.rs                     # typed builders for inputs/fixtures
      fixtures.rs                     # generic fixture loading helpers
      result_helpers.rs               # generic result/assertion helpers, never module-specific semantics
```

## `x/runtime/src/foo_tests/`

Use for:
- harnesses attached to `foo.rs`
- synthetic/local scenario generation
- private-access tests that genuinely need runtime internals

Require:
- directory sidecar shape: `foo.rs` -> `foo_tests/`
- `mod.rs` entrypoint
- tests may call `x/assertions::foo::*`

Allow:
- `super::foo`
- sibling assertions crate `x/assertions::foo`
- `test_support::*`
- std and third-party crates

Ban:
- sibling production modules
- `crate::...` outside the `foo` subtree
- parser/CLI/crawler/adapters directly
- reusable semantic assertions defined locally here
- calling other test functions

## `x/assertions/src/foo.rs`

Use for:
- reusable semantic assertions about `foo` public behavior
- one source of truth for expected outputs/results/severity/threshold behavior

Require:
- one assertion module per tested production module: `foo.rs`
- depends only on public behavior of sibling `runtime`

Allow:
- sibling `runtime` public API
- `test_support::*`
- std and third-party crates

Ban:
- private runtime internals
- parser/crawler/CLI setup
- fixture loading logic
- scenario generation
- production logic

## `x/runtime/tests/*.rs`

Use for:
- black-box integration tests
- parser-backed tests
- filesystem-backed tests
- public-surface tests only

Require:
- go through `x/runtime` public API
- reuse sibling `x/assertions` for module semantics when applicable

Allow:
- sibling `runtime`
- sibling `assertions`
- `test_support`
- std and third-party crates

Ban:
- `super::`
- crate-private helpers
- runtime `#[cfg(test)]` glue
- duplicated semantic assertions that already belong in `x_assertions`

## `test_support/`

Use for:
- generic builders
- generic fixtures helpers
- generic result helpers

Allow:
- typed builders
- generic snippets
- generic convenience helpers

Ban:
- module-specific semantics
- crate-specific rule/output contracts
- production logic

## Crate-Level Rules

For a tested production crate `x`:

Require:
- owned component directory `x/`
- sibling crates:
  - `x/runtime`
  - `x/assertions`
- sidecar directories for tested modules in `x/runtime`
- narrow public API in `x/runtime`

Allow:
- internal sidecar tests for private-access cases
- external integration tests for black-box/public behavior
- optional shared `test_support/`

Ban:
- generic `src/tests/` trash-can directories
- `*_tests.rs` files as the default tested-module shape
- inline `mod tests` in production modules
- duplicated semantic assertions across sidecars and integration tests
- `#[path = ...]` cross-module escape hatches
- broad `pub mod` exposure that turns module layout into public API

## Enforceability

The architecture must be expressed in terms of things that are mechanically checkable.

The following are mechanically enforceable:

- existence and placement of component directories and crates
- existence and placement of sidecar test directories
- presence or absence of external `tests/*.rs`
- dependency direction between `runtime`, `assertions`, and `test_support`
- import graph restrictions
- visibility restrictions such as banning broad `pub mod` exposure and `#[path = ...]` escapes

The following are **not** mechanically inferable without heuristics and therefore must not be used as mandatory triggers:

- “this helper is generic enough”
- “these builders are duplicated enough”
- “this test feels black-box”

So required/allowed decisions must be based on:

- filesystem placement
- Cargo crate boundaries
- imports
- dependency direction

## When Each Part Becomes Required

### `x/runtime`

Required when:
- component `x` exists at all

Reason:
- this is the production crate

### `x/assertions`

Required when:
- `x/runtime/tests/*.rs` exists
- or any module sidecar exists under `x/runtime/src/*_tests/mod.rs`

Reason:
- semantic assertions must have one canonical home
- these triggers are mechanically checkable from the tree itself

Allowed to be minimal at first:
- `src/lib.rs`
- only the assertion modules actually needed so far

### `x/runtime/src/foo_tests/`

Required when:
- `foo_tests/` exists and must conform

Reason:
- every tested production module must have one owned sidecar harness location

### `x/runtime/tests/*.rs`

Allowed when:
- such files exist under `x/runtime/tests/`

Required when:
- such tests already exist

Mechanical definition of an allowed external harness:
- lives under `x/runtime/tests/*.rs`
- imports only:
  - the public API of `x/runtime`
  - `x/assertions`
  - `test_support`
  - std and third-party crates
- does not import:
  - `super::`
  - `crate::`
  - `#[cfg(test)]` helpers from runtime
  - path-included source files

### `test_support`

Allowed when:
- explicitly present as a workspace dev-only crate

Not required for:
- module-specific semantics
- component-specific output contracts

No automatic “required when duplicated” trigger is defined here because that is not mechanically reliable.

## Specific Assertions

These are the concrete assertions a guardrail can check.

### TA-01 Component shape

If `crates/<name>/runtime/Cargo.toml` exists, this is a governed runtime crate.

If either of these also exist:
- `crates/<name>/runtime/tests/*.rs`
- `crates/<name>/runtime/src/*_tests/mod.rs`

require:
- `crates/<name>/assertions/Cargo.toml`

### TA-02 Runtime dependency direction

For `crates/<name>/runtime/Cargo.toml`:
- allow `dev-dependencies` on:
  - sibling `assertions`
  - `test_support`
- ban normal dependencies on sibling `assertions`

### TA-03 Assertions dependency direction

For `crates/<name>/assertions/Cargo.toml`:
- require dependency on sibling `runtime`
- allow dependency on `test_support`
- ban dependency on adapters or unrelated component runtimes unless explicitly configured

### TA-04 Tested module sidecar

For each discovered tested module `<m>` where:
- `crates/<name>/runtime/src/<m>_tests/mod.rs` exists

- require:
  - `crates/<name>/runtime/src/<m>.rs`
  - `crates/<name>/runtime/src/<m>_tests/mod.rs`
  - `crates/<name>/assertions/src/<m>.rs`
- ban:
  - `crates/<name>/runtime/src/<m>_tests.rs`
  - inline `mod tests` inside `<m>.rs`

### TA-05 Sidecar import boundary

For files under `crates/<name>/runtime/src/<m>_tests/**`, allow imports only from:
- `super::<m>`
- sibling assertions crate
- `test_support`
- std and third-party crates

Ban imports from:
- sibling production modules
- `crate::` outside the `<m>` subtree
- adapters
- path-included source files

### TA-06 Assertions import boundary

For files under `crates/<name>/assertions/src/**`, allow imports only from:
- sibling runtime public API
- `test_support`
- std and third-party crates

Ban imports from:
- runtime private modules
- parser/crawler/CLI setup modules
- fixture-loading modules

### TA-07 External harness boundary

If files exist under `crates/<name>/runtime/tests/*.rs`, validate them as external harnesses.

For those files, ban imports from:
- `super::`
- `crate::`
- runtime `#[cfg(test)]` helpers
- path-included source files

### TA-08 Escape hatches

Ban:
- `#[path = ...]` in runtime and assertions crates
- broad `pub mod` trees when a narrower `pub use` surface is configured as required
