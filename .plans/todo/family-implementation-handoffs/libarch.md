# RS-LIBARCH Implementation Handoff

Owner roots:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/libarch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/libarch` (to be created)
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/runtime.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/domain/validation-model/src/families.rs`

## What This Handoff Is For

This is now a closure record for the family implementation lane.

`RS-LIBARCH` is live. The family exists in the standard Rust family layout and is wired into the runtime/model surface.

Completed priority:

1. create the family
2. implement the planned rule inventory
3. wire the family into the Rust validation model and runtime
4. prove the family end to end

## Read First

- `/Users/tartakovsky/Projects/websmasher/guardrail3/AGENTS.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/libarch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/arch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/hexarch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/deps.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code.md`

Useful structural specimens:

- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/deps`

## Current Snapshot

Current plan state:

- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/libarch.md` now points at live code
- `RS-LIBARCH-01..11` are implemented

Current runtime/model state:

- `RustValidateFamily` contains `Libarch`
- `apps/guardrail3/crates/app/rs/runtime.rs` dispatches the family
- `apps/guardrail3/crates/app/rs/Cargo.toml` depends on `guardrail3_app_rs_family_libarch`
- the family is no longer a planning-only ownership hole

## Planned Rules To Implement

- `RS-LIBARCH-01` escalation required
- `RS-LIBARCH-02` layered root is workspace + facade package
- `RS-LIBARCH-03` `crates/` exists
- `RS-LIBARCH-04` exact layered crate set
- `RS-LIBARCH-05` workspace members match layered crate dirs
- `RS-LIBARCH-06` no extra workspace members outside layered boundary
- `RS-LIBARCH-07` `core` must not depend on `api`
- `RS-LIBARCH-08` `core` must not depend on `infra`
- `RS-LIBARCH-09` `api` must not depend on `infra`
- `RS-LIBARCH-10` `infra` must not become public package surface
- `RS-LIBARCH-11` root facade exports from `api`

## Scope You Own

This lane owned:

- creating `families/libarch/`
- implementing the family in the current multi-crate family pattern
- wiring the family into validation-model, runtime, selection/reporting/config where needed
- updating `libarch.md` once the family was live

You do **not** own:

- collapsing `libarch` into `arch`, `hexarch`, `code`, or `deps`
- expanding the plan beyond layered-library escalation and boundary enforcement

## Delivered End State

Create a live family shaped like:

```text
families/libarch/
  README.md
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_libarch_01_*.rs
        ...
        rs_libarch_11_*.rs
        rs_libarch_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_libarch_*.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

## Suggested Execution Order

1. freeze the exact rule contracts from `libarch.md`
2. add `Libarch` to:
   - validation model
   - report naming
   - config toggles if needed
   - family selection
   - runtime dispatch
3. scaffold `families/libarch/`
4. implement family facts/inputs first
5. implement the shape/escalation rules:
   - `01..06`
6. implement the dependency/facade rules:
   - `07..11`
7. add exact sidecar tests for each rule
8. update `libarch.md`

## Architecture Constraints

- use shared placement/routing patterns
- keep one rule per production file
- keep rule-specific sidecar test directories
- avoid inventing a repo-root-only shortcut
- fail closed on malformed Cargo/workspace/source inputs when the rule depends on them

## Verify With

At minimum:

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-libarch --lib
```

And once wired:

```bash
cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family libarch --format json
```

## Outcome

- `families/libarch/` exists and builds
- `RS-LIBARCH-01..11` are implemented
- the family is wired into live Rust validation
- `libarch.md` no longer describes the family as planned-only
