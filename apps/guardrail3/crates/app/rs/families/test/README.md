# RS-TEST

Rust test quality and test architecture family.

This family enforces a target state where Rust tests are:

- structurally owned
- assertion-centered
- black-box at external boundaries
- fail-closed when inputs cannot be trusted
- backed by real mutation and timeout tooling

This family is conditional:

- if an owned crate has no tests, `RS-TEST` is inactive for that crate
- if an owned crate has any tests, structure and quality rules activate for that crate
- if an owned crate shows any mutation-adoption marker, mutation rules activate for that crate

## What This Family Prevents

- tests existing without proving anything
- assertion logic duplicated across many harnesses
- integration tests reaching through private glue
- `src/tests` trash cans and inline test bodies in production files
- fake mutation setups that exist only cosmetically
- silent skips when source or config inputs are unreadable or unparsable

## Owned Roots

`RS-TEST` is a multi-root family.

It evaluates:

- workspace roots
- standalone package roots that are not workspace members

Per owned root it may inspect:

- `Cargo.toml`
- `.cargo/mutants.toml`
- `.config/nextest.toml`
- Rust source files
- Rust test files

It also reads active validation-root hook surfaces for mutation-hook presence.

## Activation Model

Judgment is per owned Rust crate/root.

### Test activation

If an owned crate has any real tests, the non-mutation `RS-TEST` rules activate for that crate.

Detectable test markers include:

- test functions in Rust source
- sidecar harnesses under `src/*_tests/`
- external harnesses under `tests/*.rs`

If an owned crate has no tests, this family does nothing for that crate.

### Mutation activation

Mutation rules activate only when mutation adoption is detectable for that crate/root.

Detectable mutation markers:

- `.cargo/mutants.toml` exists
- `[profile.mutants]` exists in `Cargo.toml`
- active hook surfaces invoke mutation testing

If none of those markers exist, mutation rules do nothing for that crate/root.

If any one of those markers exists, the full mutation setup must be present and sane.

## What This Family Does Not Own

`RS-TEST` does not own generic structural code pressure rules.

These belong to `RS-CODE`, including for test files:

- file length
- top-level `use` count
- similar source-structure caps

The decision is:

- tests are not exempt from structural sprawl rules just because they are tests
- if those caps exist, they should be enforced by `RS-CODE`, not duplicated here

`RS-TEST` owns test architecture, test assertion quality, timeout safety, fail-closed input handling, and mutation adoption/completeness.

## Target Tested-Component Shape

```text
crates/
  x/                                  # one tested component
    runtime/                          # production crate
      Cargo.toml                      # production manifest; may dev-depend on sibling assertions/test_support
      src/
        lib.rs                        # narrow public API; external harnesses must enter here
        foo.rs                        # production module under test
        foo_tests/                    # sidecar harnesses owned by foo.rs only
          mod.rs                      # sidecar entrypoint
          synthetic.rs                # synthetic and local edge-case inputs for foo
          private_access.rs           # cases that genuinely need runtime internals
      tests/
        public_surface.rs             # black-box harness through runtime public API only
        parser_backed.rs              # black-box harness that crosses parser/fixture boundary
    assertions/                       # reusable semantic assertions over runtime public behavior
      Cargo.toml                      # dev-only crate; depends on sibling runtime and optional test_support
      src/
        lib.rs                        # exports assertion modules only
        foo.rs                        # reusable semantic assertions for foo behavior/output

  test_support/                       # optional shared dev-only helper crate
    Cargo.toml                        # generic helpers only
    src/
      lib.rs
      builders.rs                     # typed builders and setup helpers
      fixtures.rs                     # generic fixture loading helpers
      result_helpers.rs               # generic assertion/result helpers, never module-specific semantics
```

## Exact Architecture Enforced

### `x/runtime`

Owns:

- production behavior
- narrow public API
- private helpers
- module-owned sidecar harnesses under `src/*_tests/`
- external black-box harnesses under `tests/*.rs`

Must not own:

- reusable semantic assertion logic
- shared generic test support

### `x/runtime/src/foo_tests/`

Owns:

- tests attached to `foo.rs`
- synthetic scenario generation
- local edge cases
- private-access tests that genuinely need runtime internals

May import:

- the owned production module subtree
- sibling `assertions::foo`
- shared `test_support`
- std and third-party crates

Must not import:

- sibling production modules
- parser, CLI, crawler, adapters directly
- duplicated semantic assertions defined locally
- other test modules as helpers

### `x/assertions/src/foo.rs`

Owns:

- reusable semantic assertions for `foo`
- one canonical home for expected findings, outcomes, thresholds, and output contracts
- proof-bearing exported assertion functions reused by sidecars and external harnesses

May import:

- sibling `runtime` public API only
- shared `test_support`
- std and third-party crates

Must not import:

- runtime private internals
- parser, crawler, CLI, or fixture setup
- scenario generation
- production logic

Must expose:

- at least one proof-bearing exported assertion function once the module exposes helper APIs

### `x/runtime/tests/*.rs`

Own:

- black-box integration tests
- filesystem-backed tests
- parser-backed tests
- public-surface tests

May import:

- `x/runtime` public API
- sibling `x/assertions`
- shared `test_support`
- std and third-party crates

Must not import:

- `super::`
- `crate::`
- runtime `#[cfg(test)]` helpers
- path-included source files
- duplicated semantic assertions that belong in `x/assertions`
- direct assertion macros as the proof site

### `test_support/`

Owns:

- generic builders
- generic fixture helpers
- generic result helpers

Must not own:

- module-specific semantics
- component-specific output contracts
- production logic
- imports or direct calls into sibling `runtime` / `assertions` component crates

## Structural Rules

For a tested component `x`:

- `x/runtime` is the production crate
- `x/assertions` is required once any test harness exists
- every tested production module `foo.rs` has exactly one sidecar harness directory:
  - `runtime/src/foo_tests/`
- every tested production module `foo.rs` has exactly one reusable assertions module:
  - `assertions/src/foo.rs`
- external black-box harnesses live only under:
  - `runtime/tests/*.rs`

Forbidden:

- inline `mod tests { ... }` bodies in production modules
- generic `src/tests/` trash-can directories
- `*_tests.rs` files as the long-term tested-module shape
- grouped family-wide test files
- semantic assertions duplicated outside `assertions/`
- `#[path = ...]` cross-module escape hatches
- broad `pub mod` exposure that turns internal layout into public API

## Rules

### Test structure

#### `RS-TEST-01`

Inline test bodies in `src/` are forbidden.

Prevents:

- production modules turning into mixed implementation-plus-test blobs

Detection:

- parse Rust source under `src/**` with `syn`
- find `ItemMod` with `#[cfg(test)]`
- error only when the module has an inline body (`content.is_some()`)

Allowed:

```rust
#[cfg(test)]
#[path = "foo_tests/mod.rs"]
mod foo_tests;
```

Forbidden:

```rust
#[cfg(test)]
mod foo_tests {
    #[test]
    fn x() {}
}
```

#### `RS-TEST-02`

Ad hoc `#[cfg(test)]` module sprawl is forbidden. The internal-test shape is nearby sidecar harness directories attached to owned production modules.

Prevents:

- arbitrary cfg-test trees with unclear ownership

Detection:

- ban `src/tests/**`
- for each internal sidecar harness at `src/<module>_tests/mod.rs`, require matching production module `src/<module>.rs`
- ban any other internal test-module shape under `src/**`
- ban arbitrary `#[cfg(test)] mod ...;` declarations that do not resolve to the owned sidecar shape

#### `RS-TEST-03`

If internal sidecar harnesses or external harnesses exist, the required `runtime/assertions` ownership split and import boundaries must hold.

Prevents:

- test structure that looks split on disk but still leaks through private or cross-module reach

Detection:

- if either exists:
  - `runtime/src/*_tests/mod.rs`
  - `runtime/tests/*.rs`
  then require sibling `assertions/Cargo.toml`
- for each `runtime/src/<module>_tests/mod.rs`, require:
  - `runtime/src/<module>.rs`
  - `assertions/src/<module>.rs`
- manifest boundary checks:
  - `runtime` may `dev-depend` on `assertions`
  - `runtime` must not normally depend on `assertions`
  - `assertions` may depend on `runtime` and `test_support`
- import boundary checks:
  - `runtime/tests/*.rs` must not import `super::` or `crate::`
  - `runtime/src/<module>_tests/**` must not import sibling production modules
  - `assertions/src/<module>.rs` must import only runtime public API, `test_support`, std, or third-party crates

### Test quality and clarity

#### `RS-TEST-04`

`#[ignore]` requires a documented reason.

Prevents:

- silent test suppression

Detection:

- parse test item attributes with `syn`
- when `#[ignore]` is present, accept exactly one of:
  - `#[ignore = "..."]`
  - same-line `// reason: ...`
  - previous-line `// reason: ...`
- otherwise report the finding

#### `RS-TEST-05`

`#[should_panic]` requires `expected = "..."`

Prevents:

- panic tests that pass on any unrelated panic

Detection:

- parse test item attributes with `syn`
- when `#[should_panic]` is present, require meta containing:
  - `expected = <string literal>`
- plain `#[should_panic]` is a finding

#### `RS-TEST-06`

Tautological literal-vs-literal assertions are forbidden.

Prevents:

- tests that syntactically assert but prove nothing

Detection:

- inspect macro invocations in test bodies
- flag exact AST shapes:
  - `assert_eq!(<lit>, <lit>)`
  - `assert_ne!(<lit>, <lit>)`
- both sides must be `syn::Expr::Lit`

#### `RS-TEST-07`

Tests must contain a real proof site.

Prevents:

- dead-weight tests that only execute code or inspect nothing meaningful

Exact proof sites accepted by this family:

- assertion macros
- calls into proof-bearing functions from the owned `assertions` module/crate

Explicitly rejected:

- heuristic “the function name contains assert/verify/expect”
- “returning `Result` means the test asserted enough”

Detection:

- parse each test function body
- pass only if at least one of these exact proof sites exists:
  - assertion macro from the allowlist:
    - `assert!`
    - `assert_eq!`
    - `assert_ne!`
    - `assert_matches!`
    - `debug_assert!`
    - `debug_assert_eq!`
    - `debug_assert_ne!`
  - call path resolving to a proof-bearing function in the owned `assertions` module/crate for that component
- otherwise report the finding

#### `RS-TEST-08`

Weak wildcard `matches!` assertions are forbidden.

Prevents:

- tests that prove only a variant and not the payload that matters

Detection:

- inspect `assert!(matches!(...))` forms in test bodies
- parse the inner pattern
- report when payload positions use `_` wildcards instead of concrete matching

### Runtime safety and fail-closed behavior

#### `RS-TEST-09`

If `tokio` is present, `.config/nextest.toml` must define timeouts.

Prevents:

- hanging async test suites

Activation:

- activate if either:
  - `tokio` is a dependency of the owned crate
  - any test uses `#[tokio::test]`

Detection:

- require `.config/nextest.toml`
- parse TOML
- require exact timeout keys:
  - `slow-timeout`
  - `leak-timeout`

#### `RS-TEST-10`

Unreadable or unparsable required inputs must fail closed.

Prevents:

- silent skips when Rust source, `Cargo.toml`, `.cargo/mutants.toml`, `.config/nextest.toml`, or required policy inputs cannot be trusted

Detection:

- track which files are required by active `RS-TEST` rules for the owned crate
- if any required file cannot be read or parsed, emit this finding instead of silently skipping dependent rules
- inactive surfaces do not count:
  - no mutation marker means mutation files are not required
  - no async-test activation means nextest config is not required

### Mutation testing

#### `RS-TEST-11`

If mutation adoption is detected, `cargo-mutants` must be installed on `PATH`.

Prevents:

- mutation testing being declared but not runnable

Activation:

- mutation rules activate if any mutation marker exists:
  - `.cargo/mutants.toml`
  - `[profile.mutants]` in `Cargo.toml`
  - active hook surfaces invoke `cargo mutants`

Detection:

- exact tool probe on `PATH`

#### `RS-TEST-12`

If mutation adoption is detected, `.cargo/mutants.toml` must exist.

Prevents:

- mutation tooling with no explicit project configuration

Detection:

- exact file existence check at the owned root

#### `RS-TEST-13`

If mutation adoption is detected, `Cargo.toml` must define `[profile.mutants]`.

Prevents:

- mutation runs using an unowned or accidental build profile

Detection:

- parse owned-root `Cargo.toml`
- require `[profile.mutants]`

#### `RS-TEST-14`

If mutation adoption is detected, active shared and Rust hook surfaces must contain the mutation-testing step.

Prevents:

- mutation checks existing only in documentation or manual workflows

Detection:

- parse active hook surfaces using executable-line matching
- require a command step invoking `cargo mutants`
- raw substring matching is not sufficient

#### `RS-TEST-15`

If mutation adoption is detected, mutation config content must not make mutation testing fake or useless.

Prevents:

- exclude-everything configs
- timeout settings that make mutation scores meaningless

Detection:

- parse `.cargo/mutants.toml`
- flag exact dangerous states such as:
  - `exclude_re = [\".*\"]`
  - `timeout_multiplier < 1.0`

#### `RS-TEST-16`

Assertions modules must actually prove something.

Prevents:

- hollow `assertions` layers that only forward into runtime or generic helpers

Detection:

- inspect exported functions in `assertions/src/**/*.rs`
- if a file exposes helper functions, require at least one proof-bearing exported function
- proof-bearing means exactly one of:
  - the function body contains an allowlisted assertion macro
  - the function calls another owned assertions function already known to be proof-bearing
- aggregator files with no exported functions are ignored

#### `RS-TEST-17`

External harnesses must prove through owned assertions only.

Prevents:

- reusable public-surface proof logic being duplicated across `runtime/tests/*.rs`

Detection:

- inspect each `runtime/tests/*.rs` test function
- report when the external harness contains direct assertion macros
- pair with `RS-TEST-07`, which still requires an actual proof site through owned assertions

#### `RS-TEST-18`

`test_support` must stay generic.

Prevents:

- shared helper crates quietly turning into semantic assertion or production-behavior owners

Detection:

- inspect `test_support/src/**/*.rs`
- report imports of sibling local `runtime` crates
- report imports of sibling local `assertions` crates
- report direct calls into those sibling local component crates

## Assertion Flow

For a tested production module `foo.rs`:

1. `runtime/src/foo_tests/` builds local scenarios and synthetic inputs
2. `assertions/src/foo.rs` owns reusable semantic assertions for `foo`
3. `runtime/tests/*.rs` exercises public behavior and reuses those same assertions
4. `test_support/` helps with generic setup only

The contract is:

- sidecars own scenario construction
- assertions own semantic expectations
- assertions own reusable public-surface proof
- external harnesses stay black-box and prove through assertions
- `test_support` stays generic and component-agnostic

## Detectable Structural Triggers

This family relies on structure that already exists on disk.

Mechanically checkable triggers:

- `crates/<name>/runtime/Cargo.toml` exists
- `crates/<name>/runtime/src/*_tests/mod.rs` exists
- `crates/<name>/runtime/tests/*.rs` exists
- `crates/<name>/assertions/Cargo.toml` exists
- `crates/<name>/assertions/src/<module>.rs` exists
- exported proof-bearing assertion functions inside `assertions/src/**/*.rs`
- dependency direction between `runtime`, `assertions`, and `test_support`
- import boundaries inside sidecars, assertions modules, and external harnesses

This family does not rely on vague triggers like:

- “these helpers feel generic enough”
- “this suite is duplicated enough”
- “this test feels like integration”

## Target Outcome

The target state is:

- every tested module has one owned sidecar harness location
- every tested module has one owned semantic assertions location
- external tests stay black-box
- assertions crates actually contain proof-bearing exports
- public-surface proof lives in assertions instead of external harnesses
- `test_support` stays generic instead of binding to sibling component crates
- test structure cannot leak across module/component boundaries
- test files are still subject to structural code pressure through `RS-CODE`
- mutation and timeout tooling are real
- weak or content-free tests are rejected
- unreadable inputs fail closed instead of silently degrading validation
