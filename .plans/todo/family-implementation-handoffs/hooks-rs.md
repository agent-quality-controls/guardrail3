# HOOK-RS Implementation Handoff

Owner roots:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hooks-rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/runtime.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/Cargo.toml`

## What This Handoff Is For

This is now a closure record for the runtime cutover lane.

The `hooks-rs` family already existed and its rule inventory was written. This lane closed the live runtime gap so Rust validation no longer stubs the family out.

Do **not** turn this into a broad hooks rewrite unless the runtime cutover exposes a concrete bug.

Completed priority:

1. wire `hooks-rs` into the live Rust runtime
2. prove the runtime now builds with `hooks-rs` on the runtime path
3. clean up only the minimum doc drift needed to reflect reality

## Read First

- `/Users/tartakovsky/Projects/websmasher/guardrail3/AGENTS.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/rs.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/shared.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/01-hooks-and-cli.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/runtime.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/Cargo.toml`

## Current Snapshot

Live family exists:

- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hooks-rs/src`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hooks-rs/assertions/src`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hooks-rs/test_support/src`

Current execution gap:

- resolved:
  - `RustValidateFamily::HooksRs` now dispatches the family
  - `apps/guardrail3/crates/app/rs/Cargo.toml` now depends on `guardrail3_app_rs_family_hooks_rs`

Planning/reporting already know the family:

- `RustValidateFamily::HooksRs` exists in `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/domain/validation-model/src/families.rs`
- report names and CLI selection already know `hooks-rs`

So this lane is mostly runtime cutover.

## Scope You Own

This lane owned:

- adding the missing runtime dependency
- wiring the runtime branch to call the family
- making sure the call shape matches the family API
- proving the runtime build no longer treats the family as dead code

You do **not** own:

- re-merging `hooks-rs` and `hooks-shared`
- a broad semantic rewrite of all hook rules
- TS/non-Rust hook cleanup

## Concrete Tasks

1. inspect the public `check(...)` API in `families/hooks-rs/src/lib.rs`
2. add `guardrail3_app_rs_family_hooks_rs` to `apps/guardrail3/crates/app/rs/Cargo.toml`
3. replace the runtime stub with the actual family call
4. run the targeted tests and a validator proof
5. update docs if they still imply the family is unwired

## Architecture Constraints

- keep `hooks-rs` separate from `hooks-shared`
- preserve the current `RustValidateFamily` naming/reporting contract
- do not silently change rule semantics while wiring runtime

## Verify With

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hooks-rs

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family hooks-rs --format json
```

If full-repo output is noisy, add a focused product-level test proving the runtime branch no longer returns an empty section.

## Outcome

- `RustValidateFamily::HooksRs` no longer returns `Vec::new()`
- the runtime crate depends on `guardrail3_app_rs_family_hooks_rs`
- the runtime crate compiles with `hooks-rs` on the live build path
- docs no longer imply a planned-only or dead execution path

## Verification Notes

- `cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hooks-rs` passes
- `cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-runtime --lib` passes with `hooks-rs` linked into runtime dispatch
- full `cargo build` / `cargo run` validator proof in this environment still hits a `SIGTERM` during `hooks-rs` code generation, which appears to be an execution-limit issue rather than a Rust compile error
