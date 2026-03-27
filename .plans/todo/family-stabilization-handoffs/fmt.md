# RS-FMT Stabilization + Attack Handoff

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/fmt`

## What This Handoff Is For

This is **not** a repo-cleanup task.

The goal is to make the `RS-FMT` family itself:

- structurally self-hosted like `test`, `arch`, `cargo`, `hexarch`, and `code`
- clean under `RS-ARCH`
- clean under `RS-TEST`
- clean under `RS-FMT`
- adversarially reviewed so its rules are trustworthy

Do **not** spend time fixing repo-wide formatting policy drift outside the family unless a finding proves to be a detector bug.

## Read First

Architecture:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/fmt.md`

Specimens for stabilized family shape:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`

Current target family:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/fmt/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/fmt/src`

## Current Snapshot

As of handoff time:

- no family README yet
- `RS-ARCH` on the family root: `0 errors, 0 warnings, 0 info`
- `RS-TEST` on the family root: `16 errors, 0 warnings, 0 info`
- `RS-FMT` on the family root: `1 error, 0 warnings, 0 info`
- unit tests pass:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-fmt --lib`
- size is small:
  - about `19` Rust files
  - about `1027` LOC

This is also an easy family and should be a safe cold-start worker task.

## Shared Rust Architecture You Must Respect

Do not reintroduce family-local root discovery.

The intended flow is:

```text
ProjectTree
  -> placement
  -> family_selection
  -> FamilyMapper
  -> typed family route
  -> family runtime/orchestrator
  -> typed rule inputs
  -> pure rule functions
```

Meaning:

- `placement` decides what Rust roots exist
- `family_selection` decides which families run
- `FamilyMapper` routes scope into typed family inputs
- the family runtime may do family-local parsing/discovery **inside routed inputs only**

For `fmt`, the family is intentionally repo-root oriented. That does **not** mean it gets to rediscover arbitrary scope on its own; it still needs to respect the shared architecture and only own formatting-policy discovery inside the routed repo context.

## Expected End State

The family root should end up shaped like:

```text
families/fmt/
  Cargo.toml
  README.md
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_fmt_01_*.rs
        ...
        rs_fmt_08_*.rs
        rs_fmt_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_fmt_01_*.rs
        ...
  test_support/
    Cargo.toml
    src/
      lib.rs
```

Notes:

- `fmt` may need little or no shared test support, but if helpers are shared they belong in `test_support`, not runtime-local glue
- assertions should own reusable semantic result checks
- runtime sidecars should not keep reusable result-shape proof

## What To Do

### Phase 1: Structural stabilization

1. Add `README.md` for the family.
2. Convert `families/fmt/Cargo.toml` into a workspace if it is still a single crate.
3. Move production code into `crates/runtime/src/`.
4. Add sibling `crates/assertions`.
5. Add sibling `test_support` if needed.
6. Update workspace wiring so `guardrail3-app-rs-family-fmt` points at `crates/runtime`.
7. Make the family pass:
   - `RS-ARCH`
   - `RS-TEST`
   - `RS-FMT`

### Phase 2: Attack the rules

Attack `RS-FMT` itself after structural stabilization.

You are looking for:

- false positives
- false negatives
- fail-closed gaps
- root-vs-nested config ambiguity bugs
- rule behavior that disagrees with `.plans/todo/checks/rs/fmt.md`

You are **not** looking to reduce repo-wide formatting findings unless the rule is wrong.

### Phase 3: Fix the rules, not the repo

For every concrete detector bug:

1. add or update a rule-specific regression
2. patch the rule / parser / fact collector
3. rerun family tests
4. rerun family self-validation

If a finding in the wider repo is legitimate debt, leave it alone.

## Attack Method

Use this approach:

1. Read the plan in `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/fmt.md`.
2. Compare each implemented rule against the documented contract.
3. Run the family on itself.
4. Create adversarial fixtures for:
   - missing root `rustfmt.toml`
   - malformed root `rustfmt.toml`
   - nested override `rustfmt.toml` and `.rustfmt.toml`
   - nightly-only keys on stable toolchain
   - edition mismatch against root `Cargo.toml`
   - dual file conflict (`rustfmt.toml` + `.rustfmt.toml`)
5. Decide for each observed result:
   - real detector bug
   - acceptable contract behavior
   - legitimate family debt

The success criterion is not “fewer findings”.
It is “when `RS-FMT` fires, the result is actually trustworthy”.

## Verify With

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-fmt --lib

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/fmt --family arch --inventory --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/fmt --family test --inventory --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/fmt --family fmt --inventory --format json
```

Expected final state:

- family tests pass
- `RS-ARCH`: `0 errors, 0 warnings, 0 info`
- `RS-TEST`: `0 errors, 0 warnings, 0 info`
- `RS-FMT`: `0 errors, 0 warnings, 0 info`

## Output Expected From The Worker

The worker should leave behind:

- the stabilized family structure
- a family README that matches reality
- green family tests
- green `RS-ARCH` / `RS-TEST` / self-family validation on the family root
- any rule fixes backed by concrete regressions

If the worker finds a policy ambiguity instead of a bug, they should document it explicitly instead of guessing.
