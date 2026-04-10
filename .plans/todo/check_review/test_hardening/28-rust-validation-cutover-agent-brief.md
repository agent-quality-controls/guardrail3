# Rust Validation Cutover Agent Brief

You own the Rust validation runtime cutover.

Your job is to make `guardrail3 rs validate` run the new Rust family checkers under `crates/app/rs/checks/**` and stop using the legacy Rust validator runtime.

## Read first

Read in this order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/checks/2026-03-24-rust-validation-cutover.md`
4. `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`

## Primary runtime surfaces

- `apps/guardrail3/crates/main.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/cli.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/validate.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs`
- `apps/guardrail3/crates/domain/config/types.rs`
- `apps/guardrail3/crates/domain/report/mod.rs`
- `apps/guardrail3/crates/domain/modules/guide.rs`

## New runtime family surfaces

- `apps/guardrail3/crates/app/rs/checks/rs/`
- `apps/guardrail3/crates/app/rs/checks/hooks/`

Important family entrypoints:

- `apps/guardrail3/crates/app/rs/checks/rs/fmt/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/deny/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/code/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/deps/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/garde/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/test/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/release/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/hooks/shared/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/hooks/rs/mod.rs`

## Legacy runtime surfaces to remove from the Rust validate path

- `apps/guardrail3/crates/app/rs/validate/`
- `apps/guardrail3/crates/app/hooks/validate.rs`

## What the cutover must do

1. `guardrail3 rs validate` must run only the new Rust family modules.
2. The public Rust validate path is rooted in `apps/guardrail3/crates/main.rs`, not just the helper in `cli/validate.rs`.
3. Rust validation selection must become family-based with repeatable `--family`.
4. Old grouped Rust flags must be removed:
   - `--code`
   - `--architecture`
   - `--garde`
   - `--tests`
   - `--release`
5. Rust config must become family-based under `[rust.checks]`.
6. Old grouped Rust config keys must be removed:
   - `architecture`
   - `tests`
   - `hooks`
7. Hook validation must live under Rust families:
   - `hooks-rs`
   - `hooks-shared`
8. `hooks-rs` implies `hooks-shared`.
9. Rust report output must be one section per family.
10. Rust validate output must emit only `RS-*` and `HOOK-*` findings.
11. Rust help/init/guide/test surfaces must stop advertising old grouped flags and old `R*` IDs.

## Family set

The Rust validation family set is:

- `fmt`
- `toolchain`
- `clippy`
- `deny`
- `cargo`
- `code`
- `hexarch`
- `deps`
- `garde`
- `test`
- `release`
- `hooks-shared`
- `hooks-rs`

The retired package-only layered family is not part of runtime validation.

## Important semantics

### Scope behavior

Do not treat whole families as scoped or unscoped.

Correct rule:

- source-file analysis surfaces may honor:
  - `--staged`
  - `--dirty`
  - `--commits`
  - `--files`
- root/config/tool/policy/architecture rules must still run in full

This matters especially for:

- `code`
- `garde`
- `test`

If needed, change those family APIs so scoped source analysis is possible without skipping root-owned rules.

### Hook dependency

`hooks-rs` is not a complete mode by itself.

Selecting `hooks-rs` must also run `hooks-shared`.

### CLI/config split

Rust and TypeScript must not share one grouped validate-args model anymore.

The Rust-only `--family` surface must not leak onto TypeScript commands.

### Config failure behavior

Rust validate must fail closed on bad Rust family-selection config:

- malformed `guardrail3.toml`
- unknown `[rust.checks]` keys
- removed grouped Rust keys under `[rust.checks]`

Do not silently drop to defaults.

## Acceptance criteria

Done means:

1. `guardrail3 rs validate` no longer routes through the legacy Rust validator runtime.
2. `guardrail3 rs validate . --family hexarch` reports the workspace-boundary issue on this repo through `RS-HEXARCH-*`, not `R-ARCH-*`.
3. Rust report sections are one-to-one with families.
4. Rust CLI help and generated guide text no longer advertise grouped Rust validate flags or old rule IDs.
5. `rs init` emits family-based `[rust.checks]`.
6. Rust validate tests and snapshots are updated to the new family model.
7. `rs hooks-validate` is removed as a separate Rust validation path.

## Suggested implementation order

1. Add Rust family-selection types and Rust-only validate args.
2. Build the new Rust validation runner over `app/rs/checks/**`.
3. Switch `main.rs` Rust validate routing to that runner.
4. Implement dependency closure for `hooks-rs => hooks-shared`.
5. Update config parsing and fail-closed behavior.
6. Update report section naming.
7. Update help/init/guide surfaces.
8. Update Rust validate tests and golden snapshots.
9. Remove legacy Rust runtime paths from the public validate flow.

## Do not

- do not preserve grouped Rust validate flags
- do not keep a compatibility delegation through `app/rs/validate`
- do not leave old `R*` IDs in Rust validate output
- do not let bad Rust config silently collapse to defaults
- do not leak Rust `--family` semantics onto TypeScript validate commands
