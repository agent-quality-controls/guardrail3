# RS-TEST — Rust test quality checker (18 rules)

**Input:** owned-root `Cargo.toml` + optional `.cargo/mutants.toml` + optional `.config/nextest.toml` + Rust source/test files + active hook surfaces
**Parser:** TOML + `syn` AST + executable-line shell parsing
**Current code:** `apps/guardrail3/crates/app/rs/families/test/**`

## Source of truth

The accepted family contract is:

- `apps/guardrail3/crates/app/rs/families/test/README.md`

That README owns:

- the 18-rule inventory
- rule numbering
- the per-root activation model
- exact detection logic
- the target test architecture

This file mirrors the live target state; it should not drift back toward the removed 19-rule family.

## Implementation mapping contract

- exactly one `RS-TEST-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `lib.rs` orchestrates only
- `discover.rs`, `facts.rs`, `inputs.rs`, `parse.rs`, and `test_support.rs` remain support only

Forbidden:

- flat `*_tests.rs` sidecars as the long-term rule-test shape
- grouped family test files
- grouped production files
- helper files that hide multiple rule predicates behind one interface

## Activation model

Judgment is per owned Rust root:

- every directory containing `Cargo.toml`

### Non-mutation activation

If an owned root has any tests, non-mutation `RS-TEST` rules activate for that root.

Detectable test markers:

- test functions in Rust source
- sidecar harnesses under `src/*_tests/`
- external harnesses under `tests/*.rs`

If an owned root has no tests, the non-mutation family stays inactive for that root.

### Mutation activation

Mutation rules activate only when any mutation-adoption marker exists:

- `.cargo/mutants.toml`
- `[profile.mutants]` in `Cargo.toml`
- active hook surfaces invoke `cargo mutants`

If any marker exists, the full mutation setup must be present and sane.

## Accepted rule inventory

| ID | Severity | What | Status |
|---|---|---|---|
| RS-TEST-01 | Error | Inline `#[cfg(test)] mod ... { ... }` bodies in `src/` are forbidden. | Implemented |
| RS-TEST-02 | Error | Ad hoc internal test-module sprawl is forbidden; owned sidecar shape only. | Implemented |
| RS-TEST-03 | Error | `runtime/assertions` split and import boundaries are enforced when harnesses exist. | Implemented |
| RS-TEST-04 | Warn | `#[ignore]` requires a documented reason. | Implemented |
| RS-TEST-05 | Warn | `#[should_panic]` requires `expected = "..."`. | Implemented |
| RS-TEST-06 | Warn | Literal-vs-literal tautological assertions are forbidden. | Implemented |
| RS-TEST-07 | Warn | Tests must contain a real proof site. | Implemented |
| RS-TEST-08 | Warn | Weak wildcard `matches!` assertions are forbidden. | Implemented |
| RS-TEST-09 | Warn/Info | Async-test roots require nextest slow/leak timeout settings. | Implemented |
| RS-TEST-10 | Error | Required inputs fail closed when unreadable or unparsable. | Implemented |
| RS-TEST-11 | Warn/Info | Mutation adoption requires `cargo-mutants` on `PATH`. | Implemented |
| RS-TEST-12 | Warn/Info | Mutation adoption requires `.cargo/mutants.toml`. | Implemented |
| RS-TEST-13 | Warn/Info | Mutation adoption requires `[profile.mutants]`. | Implemented |
| RS-TEST-14 | Warn/Info | Mutation adoption requires an executable `cargo mutants` step in active hooks. | Implemented |
| RS-TEST-15 | Warn/Info | Mutation config must avoid fake/useless settings. | Implemented |
| RS-TEST-16 | Error | Assertions modules must contain proof-bearing exported functions once they expose helpers. | Implemented |
| RS-TEST-17 | Error | External harnesses must prove through owned assertions instead of direct assertion macros. | Implemented |
| RS-TEST-18 | Error | `test_support` must stay generic and must not import/call sibling runtime/assertions crates. | Implemented |

## Explicitly removed from this family

These are no longer part of `RS-TEST`:

- tests-exist requirement
- coverage inventory / test-count ratio
- integration-tests-exist requirement
- test-function naming rule
- cfg-test-module naming rule
- test-file-length rule

Structural pressure such as file length and `use` count belongs to `RS-CODE`, including for test files.

## Exact detection reminders

- `RS-TEST-07` must not treat `Result` returns as proof.
- `RS-TEST-07` must not use name-based heuristics like `assert` / `verify` / `expect`.
- `RS-TEST-07` must only count owned assertions calls when the target function is proof-bearing.
- `RS-TEST-02` and `RS-TEST-03` must use filesystem, manifest, and import-boundary checks.
- Gotcha: if discovery sees sidecar or external harness files but cannot map them to a discovered `runtime/assertions` component, `RS-TEST-03` must report that as an error instead of silently skipping the root.
- `RS-TEST-14` must use executable-line matching on active hook surfaces, not raw substring scans.
- `RS-TEST-10` must ignore inactive surfaces:
  - no async activation means nextest config is not required
  - no mutation activation means mutation config is not required
- `RS-TEST-16` must ignore pure aggregator files with no exported helper functions.
- `RS-TEST-17` and `RS-TEST-18` must stay scoped to owned component/runtime/assertions packages, not every local crate in the repo.
- `RS-TEST-18` must reject route-construction logic in `test_support`; mapper/placement wiring is not generic support.
- `RS-TEST-03` must also reject route-construction infrastructure imports or `FamilyMapper`-built routed inputs from `assertions`; reusable proof helpers are not allowed to own mapper/placement orchestration either.

## Current migration state

The live family crate has been rewritten onto the accepted 18-rule inventory and activation model.

Still in progress:

- refactor the family’s own assertions crate away from thin wrapper helpers so it can satisfy `RS-TEST-16`
- decide whether the current runtime-sidecar self-tests should keep their direct local assertions or migrate more proof into owned assertions helpers
- expand attack-vector coverage around proof-bearing assertion detection and `test_support` boundary attacks
