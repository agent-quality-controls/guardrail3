# Hooks-Rs Runtime Agent Brief

You own the remaining `hooks-rs` operational work.

This is not primarily a family-writing task. The `hooks-rs` family exists, its rule inventory is written, and the plan marks the rules implemented. The live gap is that the runtime still does not execute the family.

## Read First

1. `AGENTS.md`
2. `.plans/todo/checks/hooks/rs.md`
3. `.plans/todo/checks/hooks/shared.md`
4. `.plans/todo/check_review/01-hooks-and-cli.md`
5. `.plans/todo/check_review/test_hardening/23-hooks-rs-agent-brief.md`
6. `apps/guardrail3/crates/app/rs/runtime.rs`
7. `apps/guardrail3/crates/app/rs/Cargo.toml`

## Primary Code

- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/`
- `apps/guardrail3/crates/app/rs/families/hooks-rs/assertions/src/`
- `apps/guardrail3/crates/app/rs/families/hooks-rs/test_support/src/`
- `apps/guardrail3/crates/app/rs/runtime.rs`
- `apps/guardrail3/crates/app/rs/Cargo.toml`

## Family Status At Handoff

Current local reality:

- the family crate exists under `families/hooks-rs/`
- the plan marks `HOOK-RS-*` rules implemented
- family selection and enum wiring know about `HooksRs`
- the runtime still stubs the family out with `RustValidateFamily::HooksRs => Vec::new()`
- `app/rs/Cargo.toml` does not yet depend on the `hooks-rs` family crate

This is the one concrete reason `hooks-rs` should still be treated as not fully done.

## Mission

Make `hooks-rs` live.

Required outcomes:

1. wire `guardrail3_app_rs_family_hooks_rs` into `apps/guardrail3/crates/app/rs/Cargo.toml`
2. replace the runtime stub in `apps/guardrail3/crates/app/rs/runtime.rs` with the actual family call
3. verify the runtime call shape is correct for the family API
4. run the relevant tests or validation commands that prove the family is no longer dead code
5. update any docs that still imply the family is planned-only or unwired

## Constraints

- Do not merge `hooks-rs` into `hooks-shared`
- Do not silently change hook policy while doing the runtime wiring
- If family-local test hardening gaps surface, note them, but the primary mission is operational cutover

## Highest-Value Targets

1. runtime wiring
2. dependency wiring in `app/rs/Cargo.toml`
3. end-to-end validation proof that `HooksRs` results now flow into reports
4. documentation drift cleanup if the runtime behavior changes the done-call

## Suggested Execution Order

1. inspect the `hooks-rs` crate public `check(...)` API
2. add the missing dependency to `app/rs/Cargo.toml`
3. wire the runtime branch
4. run targeted tests and one validator invocation that exercises `--family hooks-rs`
5. update any stale plan/README notes if needed

## Done Means

This lane is not done until:

- `RustValidateFamily::HooksRs` no longer returns `Vec::new()`
- the runtime builds against the live `hooks-rs` crate
- the relevant test/validation proof is green
- the repo no longer has a known dead `hooks-rs` execution path
