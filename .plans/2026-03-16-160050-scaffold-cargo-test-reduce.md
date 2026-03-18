# Scaffold cargo-test-reduce in testsmasher

**Date:** 2026-03-16 16:00
**Task:** Create the full project scaffold for cargo-test-reduce (library + CLI) inside the testsmasher monorepo, using ts-rust-railway template configs.

## Goal
A compilable, guardrailed Rust workspace with hex arch separation: pure logic in `packages/cargo-test-reduce`, thin CLI in `apps/cargo-test-reduce-cli`. All configs (clippy, deny, rustfmt, toolchain, pre-commit, gitignore) ported from the ts-rust-railway template, adapted for a CLI tool (not a web service).

## Approach

### Structure
```
testsmasher/
  Cargo.toml                          ← workspace root
  clippy.toml                         ← workspace-level bans (base)
  deny.toml                           ← crate bans (adapted — no axum/tokio/sqlx bans)
  rustfmt.toml
  rust-toolchain.toml
  .githooks/pre-commit
  .gitignore
  apps/
    cargo-test-reduce-cli/
      Cargo.toml                      ← bin crate, depends on cargo-test-reduce
      clippy.toml                     ← composition root (LazyLock allowed, process::Command allowed)
      src/
        main.rs
  packages/
    cargo-test-reduce/
      Cargo.toml                      ← lib crate, pure logic
      clippy.toml                     ← strict: no I/O, no global state, no process
      src/
        lib.rs
```

### Key decisions
- **CLI needs process::Command**: it shells out to cargo-llvm-cov. So the CLI clippy.toml allows it, the package clippy.toml bans it.
- **No web framework bans**: this isn't a service, so deny.toml drops axum/tokio/sqlx/reqwest from banned list. Keep the rest (json alternatives, logging alternatives, error handling).
- **Edition 2024, stable toolchain** — same as template.
- **Pre-commit hook**: adapted from template — Rust-only (no TS checks, no migration checks). Includes gitleaks, fmt, clippy, deny, structural health, machete, tests, cargo-dupes.

## Files to create
- `testsmasher/Cargo.toml` — workspace root
- `testsmasher/clippy.toml` — workspace-level bans
- `testsmasher/deny.toml` — crate bans
- `testsmasher/rustfmt.toml`
- `testsmasher/rust-toolchain.toml`
- `testsmasher/.gitignore`
- `testsmasher/.githooks/pre-commit`
- `testsmasher/apps/cargo-test-reduce-cli/Cargo.toml`
- `testsmasher/apps/cargo-test-reduce-cli/clippy.toml`
- `testsmasher/apps/cargo-test-reduce-cli/src/main.rs`
- `testsmasher/packages/cargo-test-reduce/Cargo.toml`
- `testsmasher/packages/cargo-test-reduce/clippy.toml`
- `testsmasher/packages/cargo-test-reduce/src/lib.rs`
