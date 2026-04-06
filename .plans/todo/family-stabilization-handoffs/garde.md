# RS-GARDE Stabilization + Attack Handoff

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/garde`

## What This Handoff Is For

This is **not** a repo-cleanup task.

The goal is to make the `RS-GARDE` family itself:

- structurally self-hosted like `test`, `arch`, `cargo`, `hexarch`, and `code`
- clean under `RS-ARCH`
- clean under `RS-TEST`
- clean under `RS-GARDE`
- adversarially reviewed so its rules are trustworthy

Do **not** spend time cleaning repo-wide garde policy drift outside the family unless a finding proves to be a detector bug.

The priority is:

1. stabilize the family structure
2. make the family self-enforcing
3. attack the rules for false positives / false negatives / fail-closed gaps
4. fix the rules

## Read First

Architecture:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/garde.md`

Cross-family contract you must understand:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md`

Stabilized family specimens:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`

Current target family:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/garde/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/garde/src`

## Current Snapshot

As of handoff time:

- no family README yet
- still a single-crate family
- current source tree includes:
  - `discover.rs`
  - `facts.rs`
  - `garde_support.rs`
  - `inputs.rs`
  - `parse.rs`
  - `test_support.rs`
  - rule files `RS-GARDE-CONFIG-01..13`
  - rule-specific sidecar dirs already exist for all live rules
- size is medium:
  - about `90` Rust files
  - about `6048` LOC

Last known family-health snapshot before the current outer-workspace break:

- `RS-TEST` on the family root: about `29` errors
- self-family had at least `1` remaining family-root issue

Current global blocker:

- top-level Cargo commands from `apps/guardrail3/Cargo.toml` are currently poisoned by the in-flight `deny` workspace split
- the error looks like:
  - `multiple workspace roots found in the same workspace`
  - `.../families/deny`
  - `.../apps/guardrail3`

So if top-level validation fails for unrelated reasons, do **not** “fix” deny or the outer workspace just to get `garde` green. Another agent owns that lane.

## Shared Rust Architecture You Must Respect

Do not reintroduce family-local root discovery outside the routed surface.

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

For `garde`, this matters because it is a conditional, multi-root family:

- it owns per-owned-root garde gating
- it resolves the covering `clippy.toml` for that owned root
- it parses source files belonging to that owned root

It must **not** collapse to repo-root-only behavior and it must **not** invent its own root universe outside `RsGardeRoute`.

## Expected End State

The family root should end up shaped like:

```text
families/garde/
  Cargo.toml
  README.md
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        discover.rs
        facts.rs
        garde_support.rs
        inputs.rs
        parse.rs
        rs_garde_01_*.rs
        ...
        rs_garde_13_*.rs
        rs_garde_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_garde_01_*.rs
        ...
  test_support/
    Cargo.toml
    src/
      lib.rs
```

Notes:

- this family already has broad rule coverage; the missing part is structural self-hosting and adversarial trust
- runtime sidecars should not keep reusable semantic proof
- assertions should own reusable result checks
- `test_support` should hold generic fixtures/helpers only, not rule semantics

## What To Do

### Phase 1: Structural stabilization

1. Add `README.md` for the family.
2. Convert `families/garde/Cargo.toml` into a workspace if it is still a single crate.
3. Move production code into `crates/runtime/src/`.
4. Add sibling `crates/assertions`.
5. Add sibling `test_support` and move generic helpers out of runtime-local `test_support.rs`.
6. Update workspace wiring so `guardrail3-app-rs-family-garde` points at `crates/runtime`.
7. Make the family pass:
   - `RS-ARCH`
   - `RS-TEST`
   - `RS-GARDE`

### Phase 2: Attack the rules

Attack `RS-GARDE` itself after structural stabilization.

You are looking for:

- false positives
- false negatives
- fail-closed gaps
- route/scope leaks
- wrong garde gating
- wrong covering-clippy resolution
- source parse / AST bypass holes
- disagreement with `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/garde.md`

You are **not** looking to reduce wider-repo garde findings unless the detector is wrong.

### Phase 3: Fix the rules, not the repo

For every concrete detector bug:

1. add or update a rule-specific regression
2. patch the rule / parser / fact collector
3. rerun family tests
4. rerun family self-validation

If a wider-repo finding is legitimate debt, leave it alone.

## Attack Method

Use this approach:

1. Read `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/garde.md`.
2. Compare each implemented rule against that contract.
3. Read `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md` because `garde` explicitly depends on `clippy`’s covering-config contract.
4. Stabilize the family first, then attack semantics.
5. Focus attack energy on the real complexity:
   - conditional garde gating
   - root-policy resolution
   - covering `clippy.toml` lookup per owned root
   - multi-root source ownership
   - manual `Deserialize` bypasses
   - `sqlx::query_as!` inventory
   - field-level garde constraints
   - nested `#[garde(dive)]`
   - `ctx` usage without explicit `#[garde(context(...))]`
   - malformed source/policy inputs and fail-closed behavior

## High-Value Attack Targets

Attack these first:

- `RS-GARDE-CONFIG-02/03/04/06`
  - correct clippy-ban ownership and per-root covering-config lookup
  - false greens when garde is enabled but a local covering config is incomplete
  - false positives when garde is disabled
- `RS-GARDE-AST-01/07/08`
  - derive / manual-impl bypasses
  - primitive-only false positives
  - enum variant-shape edge cases
- `RS-GARDE-AST-05/12/13`
  - field-level semantic enforcement
  - nested validated types
  - context-driven validators
  - local helper/macro shapes that might evade the AST logic
- `RS-GARDE-10`
  - malformed source and policy inputs must surface, not silently skip

## Verify With

When the outer workspace is healthy again:

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-garde --lib

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/garde --family arch --inventory --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/garde --family test --inventory --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/garde --family garde --inventory --format json
```

Expected final state:

- family tests pass
- `RS-ARCH`: `0 errors, 0 warnings, 0 info`
- `RS-TEST`: `0 errors, 0 warnings, 0 info`
- `RS-GARDE`: `0 errors, 0 warnings, 0 info`

If top-level validation is still blocked by the unrelated `deny` workspace split, document that explicitly and verify as locally as possible without editing deny-owned files.

## Output Expected From The Worker

The worker should leave behind:

- the stabilized family structure
- a family README that matches reality
- green family tests
- green `RS-ARCH` / `RS-TEST` / self-family validation on the family root once the unrelated workspace blocker is gone
- any rule fixes backed by concrete regressions

If the worker finds a policy ambiguity instead of a bug, they should document it explicitly instead of guessing.
