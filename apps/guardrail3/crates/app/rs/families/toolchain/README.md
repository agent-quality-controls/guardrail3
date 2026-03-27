# RS-TOOLCHAIN

Repository-root Rust toolchain contract family.

This family is intentionally root-level. It does not discover per-workspace or
per-package toolchain files. The owned contract is:

- one root `rust-toolchain.toml`
- optional legacy root `rust-toolchain`
- one root `Cargo.toml` source for MSRV comparison

## What This Family Enforces

- `RS-TOOLCHAIN-01`: `rust-toolchain.toml` exists at the family/repository root
- `RS-TOOLCHAIN-02`: channel and component policy
- `RS-TOOLCHAIN-03`: pinned stable toolchain vs `Cargo.toml` `rust-version`
- `RS-TOOLCHAIN-04`: legacy `rust-toolchain` migration and coexistence warning

## Layout

```text
toolchain/
  Cargo.toml                # family workspace root
  README.md
  rust-toolchain.toml       # self-hosted family toolchain contract
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        discover.rs
        facts.rs
        inputs.rs
        rs_toolchain_01_exists.rs
        rs_toolchain_01_exists_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_toolchain_01_exists.rs
        ...
```

## Testing Shape

- one production rule per file
- one owned `*_tests/mod.rs` sidecar directory per rule
- rule-side tests prove local behavior
- `assertions/` owns reusable result assertions so sidecars do not duplicate
  semantic checks

## Self-Hosting Notes

The family carries its own root `rust-toolchain.toml` so validating the family
directory exercises the same root-level contract it enforces elsewhere.
