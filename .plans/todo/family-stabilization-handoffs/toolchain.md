# RS-TOOLCHAIN Stabilization + Attack Handoff

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/toolchain`

## What This Handoff Is For

This is **not** a repo-cleanup task.

The goal is to make the `RS-TOOLCHAIN` family itself:

- structurally self-hosted like `test`, `arch`, `cargo`, `hexarch`, and `code`
- clean under `RS-ARCH`
- clean under `RS-TEST`
- clean under `RS-TOOLCHAIN`
- adversarially reviewed so its rules are trustworthy

Do **not** spend time reducing repo-wide `RS-TOOLCHAIN` findings in unrelated code.
The priority is:

1. stabilize the family
2. make the family self-enforcing
3. attack the rules for false positives / false negatives
4. fix the rules

## Read First

Architecture:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/toolchain.md`

Specimens for stabilized family shape:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`

Current target family:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/toolchain/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/toolchain/src`

## Current Snapshot

As of handoff time:

- no family README yet
- `RS-ARCH` on the family root: `0 errors, 0 warnings, 0 info`
- `RS-TEST` on the family root: `8 errors, 0 warnings, 0 info`
- `RS-TOOLCHAIN` on the family root: `1 error, 0 warnings, 0 info`
- unit tests pass:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-toolchain --lib`
- size is small:
  - about `12` Rust files
  - about `831` LOC

This is an easy family and should be a safe cold-start worker task.

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

Do not put root-scope discovery logic into the family.

## Expected End State

The family root should end up shaped like:

```text
families/toolchain/
  Cargo.toml
  README.md
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_toolchain_01_*.rs
        ...
        rs_toolchain_04_*.rs
        rs_toolchain_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_toolchain_01_*.rs
        ...
  test_support/
    Cargo.toml
    src/
      lib.rs
```

Notes:

- `test_support` may be very small or almost empty, but keep the same pattern if the family needs shared fixture helpers
- runtime sidecars should not own reusable semantic result assertions
- assertions should own reusable proof-bearing checks

## What To Do

### Phase 1: Structural stabilization

1. Add `README.md` for the family.
2. Convert `families/toolchain/Cargo.toml` into a workspace if it is still a single crate.
3. Move production code into `crates/runtime/src/`.
4. Add sibling `crates/assertions`.
5. Add sibling `test_support` if needed for generic fixture helpers.
6. Update workspace wiring so `guardrail3-app-rs-family-toolchain` points at `crates/runtime`.
7. Make the family pass:
   - `RS-ARCH`
   - `RS-TEST`
   - `RS-TOOLCHAIN`

### Phase 2: Attack the rules

Attack `RS-TOOLCHAIN` itself after structural stabilization.

You are looking for:

- false positives
- false negatives
- fail-closed gaps
- rules that silently skip malformed active inputs
- rule behavior that disagrees with `.plans/todo/checks/rs/toolchain.md`

You are **not** looking to reduce repo debt outside the family.

### Phase 3: Fix the rules, not the repo

For every concrete detector bug:

1. add or update a rule-specific sidecar regression
2. patch the rule / parser / fact collector
3. rerun family tests
4. rerun family self-validation

If a finding in the wider repo is actually legitimate debt, leave it alone.

## Attack Method

Use this approach:

1. Read the plan in `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/toolchain.md`.
2. Compare each implemented rule against that contract.
3. Run the family on itself.
4. Create adversarial fixtures for:
   - malformed `rust-toolchain.toml`
   - malformed root `Cargo.toml`
   - both `rust-toolchain` and `rust-toolchain.toml`
   - pinned stable vs MSRV mismatch
   - nightly-only settings or ambiguous channel forms if relevant
5. Decide for each observed result:
   - real detector bug
   - acceptable contract behavior
   - legitimate family debt

The success criterion is not “fewer findings”.
It is “findings now mean something trustworthy”.

## Verify With

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-toolchain --lib

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/toolchain --family arch --inventory --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/toolchain --family test --inventory --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/toolchain --family toolchain --inventory --format json
```

Expected final state:

- family tests pass
- `RS-ARCH`: `0 errors, 0 warnings, 0 info`
- `RS-TEST`: `0 errors, 0 warnings, 0 info`
- `RS-TOOLCHAIN`: `0 errors, 0 warnings, 0 info`

## Output Expected From The Worker

The worker should leave behind:

- the stabilized family structure
- a family README that matches reality
- green family tests
- green `RS-ARCH` / `RS-TEST` / self-family validation on the family root
- any rule fixes backed by concrete regressions

If the worker finds a policy ambiguity instead of a bug, they should document it explicitly instead of guessing.
