# RS-CODE Family Stabilization Plan

This plan is about stabilizing the `RS-CODE` family itself as a self-hosted Rust family.

It is not the old rule-hardening lane.
The rule inventory and prior adversarial rule notes still live in:

- [code.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code.md)
- [.plans/todo/check_review/test_hardening/02-code.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/test_hardening/02-code.md)

This document is the current migration plan for getting the family into the same tier as:

- `RS-TEST`
- `RS-ARCH`
- `RS-HEXARCH`
- `RS-CARGO`

## Current Snapshot

As of the current checkpoint:

- [code/Cargo.toml](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/Cargo.toml) is now a family workspace root
- the existing runtime source tree has been moved under [crates/runtime/src](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src)
- placeholder sibling crates exist at:
  - [crates/assertions](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/assertions)
  - [test_support](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/test_support)
- the family already consumes `RsCodeRoute` in [lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs)
- the family passes `RS-ARCH`
- the family tests pass:
  - `179 passed`
- the family does not pass `RS-TEST`
  - `RS-TEST-02`: `31`
  - `RS-TEST-03`: `778`
  - `RS-TEST-16`: `99`
- there was no family README before this checkpoint

Approximate size:

- `3356` production LOC
- `8041` test LOC
- `179` Rust files under [src](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/src)
- `143` of those under rule-specific `*_tests/` directories

So the family is already semantically alive and heavily tested, but its self-hosting split is only partial: the workspace shape exists, while assertion ownership and generic test-support boundaries do not.

## Why `RS-CODE` Next

Compared to the remaining unstabilized Rust families:

- `code` has `61` `RS-TEST` errors
- `release` has `59`
- `garde` has `29`
- `deps` has `23`

`RS-CODE` is the best next target because:

- it has the broadest repo-wide leverage on everyday source quality
- it already uses shared routing, so the architecture gap is mostly structural
- once stabilized, it becomes the next family that can pressure cleaner source shape across the repo

## Target End State

The family should end in the same self-hosted shape as the other stabilized families:

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
        ...
        rs_code_30_*.rs
        rs_code_01_*_tests/
          mod.rs
        ...
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

Required end-state properties:

- passes `RS-ARCH`
- passes `RS-TEST`
- family-local README matches reality
- runtime owns orchestration only
- assertions own reusable semantic proof helpers
- test support stays generic
- route/placement wiring is external except for narrow family test entrypoints

## Main Risks

### 1. Size, not semantics

The biggest cost is file movement and boundary cleanup, not discovering brand-new rule semantics.

The family already has:

- routed entrypoint
- per-rule files
- per-rule sidecar test directories
- a large passing local suite

So the main danger is migration churn across many files, not missing rule inventory.

### 2. Runtime-local test support still leaks architecture wiring

[test_support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/test_support.rs) currently imports:

- `FamilyMapper`
- `placement`
- `CheckResult`

That is exactly the kind of pre-self-hosting debt removed from `test`, `arch`, `cargo`, and `hexarch`.

### 3. Assertions layer exists only as a placeholder

The family now has a sibling assertions crate, but it is only scaffolding. Runtime sidecars still own result-shape proof.

That means the migration is no longer about file movement. It is about extracting reusable semantic proof out of runtime sidecars.

## Migration Phases

## Phase 1 — Documentation And Contract Lock

1. Add [code/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/README.md).
2. Keep this plan file as the current structural source of truth.
3. Do not update rule inventory wording unless the rule contract itself changes.

## Phase 2 — Workspace Split

Status:

- complete

Completed work:

1. Converted [code/Cargo.toml](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/Cargo.toml) into a family workspace root.
2. Created:
   - `crates/runtime/Cargo.toml`
   - `crates/assertions/Cargo.toml`
   - `test_support/Cargo.toml`
3. Updated the main Rust workspace wiring so the family package points at `crates/runtime`.

## Phase 3 — Runtime Move

Status:

- complete

Completed work:

1. Moved current runtime files from `src/` into `crates/runtime/src/`.
2. Kept the current rule-file naming.
3. Kept the current rule-specific `*_tests/` directories under runtime.
4. Restored passing unit coverage after the move.

## Phase 4 — Generic Test Support Extraction

Current state:

- not started
- runtime-local `test_support.rs` is still active
- sibling `test_support` crate exists but is still empty scaffolding

## Phase 5 — Assertions Split

Current state:

- not started
- sibling assertions crate exists but is still empty scaffolding
- runtime sidecars still own semantic result-shape proof

Target work:

1. Add one assertions module per production rule.
2. Start with the most reusable, result-shape-heavy rules:
   - suppression rules
   - public-surface rules
   - fail-closed rules
3. Move proof-bearing assertion helpers out of runtime sidecars and into `crates/assertions`.
4. Make sidecars prove through owned assertions rather than inline result-shape checking.

## Phase 6 — RS-TEST Closure

1. Run:
   - `rs validate .../families/code --family test`
   - `rs validate .../families/code --family arch`
   - `cargo test -p guardrail3-app-rs-family-code --lib`
2. Close all structural `RS-TEST-02/03` failures.
3. If stricter `RS-TEST-07/16/18` findings appear after the split, fix those before any rule-family attack pass.

## Phase 7 — Family Attack Pass

Only after structural stabilization:

1. adversarially attack `RS-CODE` itself
2. compare README, rule inventory, and implementation
3. look for false greens, false positives, and hidden scope widening

That is separate work from the workspace split.

## Immediate Execution Order

The next concrete coding order should be:

1. workspace split scaffolding
2. move runtime files without semantic changes
3. restore green family tests
4. extract generic test support
5. add assertions crate and migrate proof helpers
6. make `RS-CODE` pass `RS-TEST`
7. only then start attacking rule semantics again

## Definition Of Done

`RS-CODE` is stabilized when:

- [code/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/README.md) matches the live implementation
- the family has the workspace shape above
- `guardrail3-app-rs-family-code` points at `crates/runtime`
- family tests pass
- `RS-ARCH` on the family root is clean
- `RS-TEST` on the family root is clean
- the family is ready for a deeper adversarial rule-family review
