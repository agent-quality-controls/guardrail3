# RS-DENY

Rust `cargo-deny` policy family.

This family owns allowed deny config placement plus the generated `deny.toml` contract: graph settings, ban baselines, feature bans, license allowlists, advisory policy, source restrictions, and inventory-style visibility for local exceptions.

## What This Family Owns

`RS-DENY` owns:

- allowed root placement for `deny.toml`, `.deny.toml`, and `.cargo/deny.toml`
- local shadowing detection for nested deny configs
- generated service/library deny baseline parity
- graph, bans, feature-ban, license, advisory, and source policy checks
- inventory rules for stricter local policy and documented local exceptions
- self-hosted runtime/assertions/test-support workspace structure

It does not own:

- Cargo workspace structure
- general Rust source architecture
- formatting or toolchain policy

Those belong to:

- `RS-ARCH`
- `RS-CARGO`
- `RS-CODE`
- `RS-FMT`
- `RS-TOOLCHAIN`

## Current Workspace Shape

```text
apps/guardrail3/crates/app/rs/families/deny/
  Cargo.toml
  README.md
  deny.toml
  rustfmt.toml
  rust-toolchain.toml
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_deny_01_*.rs
        ...
        rs_deny_30_*.rs
        rs_deny_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_deny_01_*.rs
        ...
        rs_deny_30_*.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

## Current Status

This family is now in the same stabilized shape as `RS-FMT`:

- workspace root plus `crates/runtime`, `crates/assertions`, and `test_support`
- one production rule file per `RS-DENY-*`
- one rule-specific sidecar directory per rule
- owned assertions modules for reusable result-shape proofs
- clean `RS-TEST` validation for the internal sidecar/assertions boundary
- self-hosted family-root `deny.toml`, `rustfmt.toml`, and `rust-toolchain.toml`

Next work on `deny` should stay focused on detector drift and adversarial coverage, not broadening the policy surface.
