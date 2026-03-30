# RS-CODE Implementation Handoff

Owner root:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code`

## What This Handoff Is For

This is now a closure record for the last inventory-completion lane.

`RS-CODE` already existed and most of the family was live. This lane closed the remaining universal-rule inventory and the family-local overlap ambiguity around weak public error forms.

Do **not** spend this lane cleaning repo-wide `RS-CODE` findings unless a result proves the detector is wrong.

Completed priority:

1. implement the planned `RS-CODE-*` rules
2. keep them inside the current family architecture
3. add exact regressions proving the rules
4. close adjacent family-local bugs only when exposed by that work

## Read First

- `/Users/tartakovsky/Projects/websmasher/guardrail3/AGENTS.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code-family-stabilization.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/FIXES.md`

## Current Snapshot

Live code root:

- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src`

Family status:

- `RS-CODE-01..30` are implemented except `RS-CODE-28`, which was merged into `RS-CODE-27`
- `RS-CODE-31..36` are implemented except `RS-CODE-28`, which remains merged into `RS-CODE-27`
- `RS-CODE-33` is now the sole firing path for weak public error forms; `RS-CODE-25` stays non-firing to avoid overlap

The detailed rule ledger remains in:

- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code.md`

The family already has the expected workspace split:

- `crates/runtime`
- `crates/assertions`
- `test_support`

So this is not a family-structure job.

## Scope You Own

This lane owned:

- implementing the still-planned `RS-CODE-*` rules
- any minimal fact/input extensions required for those rules
- exact regressions for those rules
- plan/doc cleanup necessary to reflect the completed inventory

You do **not** own:

- `RS-ARCH` routing
- `RS-HEXARCH` architectural layering
- `RS-DEPS` dependency policy
- repo-wide cleanup of existing findings

## Rules Closed In This Lane

### `RS-CODE-31`

- public named-field `pub struct` is forbidden

### `RS-CODE-33`

- public functions must not return obviously weak public error forms:
  - `Result<_, String>`
  - `Result<_, &str>`
  - `Result<_, anyhow::Error>`
  - `Result<_, Box<dyn Error>>`

### `RS-CODE-34`

- more than 6 type/const generic parameters on `struct`, `enum`, `trait`, or `fn`
- lifetimes do not count

### `RS-CODE-35`

- per-crate structural caps:
  - module depth > 6
  - sibling subdirectories > 12
  - sibling `.rs` files > 20 in one Rust source directory

### `RS-CODE-36`

- one string-dispatch site has more than 10 string-literal branches
- applies to `match` and `if/else if` chains over the same expression
- test files exempt

## Architecture Constraints

Stay inside the current family architecture:

- facts/discovery in `crates/runtime/src/facts.rs`, `discover.rs`, `parse/`, or a small new helper if needed
- typed local rule inputs in `inputs.rs`
- one production file per rule
- one rule-specific `*_tests/` directory per rule

Do not:

- bundle the planned rules into one grouped production file
- move policy ownership into another family
- silently narrow the plan contract to make implementation easier

## Execution Shape

1. extend shared parser/types only where multiple rules actually reuse the shape
2. land one production file plus one sidecar test directory per new rule
3. keep `RS-CODE-33` as the sole live weak-public-error firing path
4. update the detailed ledger and family-local docs to remove the planned-only state

## Verify With

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-code --lib

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json
```

If full-repo validation noise is too broad, at minimum prove the family test crate is green and the new rules fire correctly in targeted family fixtures.

## Outcome

- `RS-CODE-31`, `33`, `34`, `35`, and `36` are implemented
- each has a production file and rule-specific sidecar tests
- family tests pass
- `code.md` no longer marks those rules planned
- weak-public-error ownership is explicit instead of overlapping between `RS-CODE-25` and `RS-CODE-33`
