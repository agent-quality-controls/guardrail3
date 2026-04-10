# Parallel Cargo And Agent Builds

This repo should not run multiple Cargo commands against the same `target/debug` at the same time.

The failure mode is:

- one process holds `target/debug/.cargo-lock`
- other Cargo commands print `Blocking waiting for file lock on build directory`
- interrupted turns can leave stale Cargo processes running in the background
- agents then pile up behind the same lock and everybody slows down

## Core rules

1. Only one process may use the shared default target dir at a time.
2. If work is parallel, each compile-running agent must use the stable family target dir for the folder it owns.
3. Read-only agents should not run Cargo at all.
4. Do not create a fresh random target dir per command. Reuse the same family target dir for the whole session so incremental artifacts stay warm.
5. Prefer narrow verification before broad verification.

## Recommended layout

Use a small fixed pool of family target dirs and keep them stable:

```bash
target/verify-shared
target/fmt
target/toolchain
target/cargo
target/code
target/hexarch
target/deps
target/garde
target/test
target/release
target/clippy
target/deny
target/hooks-rs
target/hooks-shared
target/arch-package
```

Canonical family map:

- `target/verify-shared`
  - one verifier only
  - used for the main branch of truth when a human wants the closest thing to the normal build
- `target/fmt`
  - owned by `apps/guardrail3/crates/app/rs/checks/rs/fmt/`
- `target/toolchain`
  - owned by `apps/guardrail3/crates/app/rs/checks/rs/toolchain/`
- `target/cargo`
  - owned by `apps/guardrail3/crates/app/rs/checks/rs/cargo/`
- `target/code`
  - owned by `apps/guardrail3/crates/app/rs/checks/rs/code/`
- `target/hexarch`
  - owned by work under `apps/guardrail3/crates/app/rs/checks/rs/hexarch/`
- `target/deps`
  - owned by `apps/guardrail3/crates/app/rs/checks/rs/deps/`
- `target/garde`
  - owned by `apps/guardrail3/crates/app/rs/checks/rs/garde/`
- `target/test`
  - owned by `apps/guardrail3/crates/app/rs/checks/rs/test/`
- `target/release`
  - owned by work under `apps/guardrail3/crates/app/rs/checks/rs/release/`
- `target/clippy`
  - owned by work under `apps/guardrail3/crates/app/rs/checks/rs/clippy/`
- `target/deny`
  - owned by `apps/guardrail3/crates/app/rs/checks/rs/deny/`
- `target/hooks-rs`
  - owned by work under `apps/guardrail3/crates/app/rs/checks/hooks/rs/`
- `target/hooks-shared`
  - owned by `apps/guardrail3/crates/app/rs/checks/hooks/shared/`
- `target/arch-package`
  - reserved for package-scoped `rs/arch` work

Rule: the target dir name should match the family folder name, not the person or the session.
If the plan names a family before the folder exists yet, reserve the target name from the plan name itself.

## Why stable per-agent target dirs matter

Separate target dirs avoid lock contention, but they do **not** share compiled artifacts with each other.

That means:

- `CARGO_TARGET_DIR=target/tmp-123 cargo check ...`
- `CARGO_TARGET_DIR=target/tmp-456 cargo check ...`

will rebuild dependencies twice.

To keep builds fast:

- give each family one stable target dir
- keep reusing it across commands
- avoid one-off temp target dirs unless isolation matters more than speed

## Fast verification order

Use the cheapest command that answers the question.

Preferred order:

1. `cargo check --lib`
2. `cargo check --tests`
3. `cargo test --no-run`
4. focused test runs such as `cargo test rs_hexarch_07 -- --nocapture`
5. broad suite runs only when necessary

More narrowing:

- use `--lib` when you only changed library code
- use `--test <name>` or a test filter when you only need one area
- use `--no-run` to prove compileability before paying runtime cost

## Commands

Main verifier on a stable isolated target:

```bash
CARGO_TARGET_DIR=target/verify-shared cargo check --manifest-path apps/guardrail3/Cargo.toml --lib
```

Hexarch focused compile:

```bash
CARGO_TARGET_DIR=target/hexarch cargo test --manifest-path apps/guardrail3/Cargo.toml --lib rs_hexarch_07 --no-run
```

Hexarch focused run:

```bash
CARGO_TARGET_DIR=target/hexarch cargo test --manifest-path apps/guardrail3/Cargo.toml --lib rs_hexarch_07 -- --nocapture
```

Parallel family example:

```bash
CARGO_TARGET_DIR=target/code cargo check --manifest-path apps/guardrail3/Cargo.toml --lib
CARGO_TARGET_DIR=target/hexarch cargo test --manifest-path apps/guardrail3/Cargo.toml --lib rs_hexarch_10 --no-run
```

## Cleanup when things get stuck

Find the lock holders:

```bash
lsof target/debug/.cargo-lock
```

Find all Cargo processes:

```bash
pgrep -af cargo
```

Kill only the stale processes you own:

```bash
kill <pid>
```

Do not blindly kill every Cargo process if another active agent is still using one intentionally.

## Practical team policy

Use this policy in parallel sessions:

1. One agent owns shared verification, or nobody uses shared `target/debug` at all.
2. Every compile-running family gets one stable `CARGO_TARGET_DIR` named after its folder.
3. Analysis-only agents do not run Cargo.
4. If a turn is interrupted, check for stale Cargo processes before launching more.
5. Do not run broad `cargo test` in parallel unless there is a strong reason.

## About full rebuilds

If you isolate every command into a brand-new target dir, you will get full rebuilds every time.

To avoid that:

- reuse the same family target dir
- keep the number of target dirs small
- do not rotate target dirs unless the build cache is poisoned

This gives you:

- no lock fights between agents
- incremental rebuilds inside each agent’s own cache

## Optional future optimization

This machine does not currently have `sccache` installed.

If faster parallel Rust builds become important, install and configure `sccache`. It can reduce repeated recompilation across separate target dirs by caching compiler outputs.

Without `sccache`, the best available strategy is:

- stable per-family target dirs
- narrow Cargo commands
- one shared verifier or no shared verifier
