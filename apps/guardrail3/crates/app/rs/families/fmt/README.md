# RS-FMT

Rust formatting-policy family.

This family owns the repository-root `rustfmt.toml` contract plus any nested override files that would shadow it. It is intentionally repo-global: `fmt` does not route per app or per package, and it does not rediscover Rust roots on its own.

## What This Family Owns

`RS-FMT` owns:

- required root `rustfmt.toml` or `.rustfmt.toml`
- baseline rustfmt key/value policy
- extra-setting inventory
- nightly-only rustfmt keys on stable toolchains
- nested `rustfmt.toml` and `.rustfmt.toml` override detection
- root Cargo/rustfmt edition consistency
- `ignore` escape-hatch visibility
- same-directory `rustfmt.toml` plus `.rustfmt.toml` conflicts
- fail-closed reporting when root `Cargo.toml` or `rust-toolchain.toml` is required and malformed

It does not own:

- repo-global Rust root placement
- per-root Cargo workspace policy
- source-level Rust quality

Those belong to:

- `RS-ARCH`
- `RS-CARGO`
- `RS-CODE`

## Current Workspace Shape

```text
apps/guardrail3/crates/app/rs/families/fmt/
  Cargo.toml
  README.md
  rustfmt.toml
  rust-toolchain.toml
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_fmt_01_*.rs
        ...
        rs_fmt_08_*.rs
        rs_fmt_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_fmt_01_*.rs
        ...
        rs_fmt_08_*.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

## Shared Architecture Boundary

`fmt` remains a repo-root family, but it still respects the shared Rust validation boundary:

- `ProjectTree` is the shared snapshot
- `fmt` does family-local config parsing only inside that repo snapshot
- rules stay pure and consume typed inputs
- no family-local root placement or mapper replacement is allowed

## Current Status

This family is now self-hosted in the same stabilized shape as the other migrated Rust families:

- workspace root plus `crates/runtime`, `crates/assertions`, and `test_support`
- one production rule file per `RS-FMT-*`
- one rule-specific sidecar test directory per rule
- family-local assertions crate for reusable result-shape checks
- fail-closed coverage for required root Cargo/toolchain inputs used by `RS-FMT-04` and `RS-FMT-06`

The next work on `fmt` should stay in the attack-review lane: compare live behavior against `.plans/todo/checks/rs/fmt.md`, add regressions for any concrete detector drift, and avoid treating repo-wide formatting debt as a detector bug.
