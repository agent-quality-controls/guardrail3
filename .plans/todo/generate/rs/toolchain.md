# Rust Generator — `toolchain`

## Generated artifacts

- `<validation_root>/rust-toolchain.toml`

## Ownership mode

- exact-owned

## Root selection

`toolchain` is a validation-root family.

The generator owns exactly one toolchain file:
- `rust-toolchain.toml` at the validation root

It must never generate:
- legacy `rust-toolchain`
- nested `rust-toolchain.toml`
- nested `rust-toolchain`
- per-app, per-workspace, or per-package toolchain files

## Required generator contract

- the validation root contains exactly one generator-owned `rust-toolchain.toml`
- the generated file declares the canonical repository toolchain contract
- the generated file includes the required component set for the checker contract
- generation never creates sibling legacy ambiguity or nested toolchain drift

## Checker target

- `.plans/todo/checks/rs/toolchain.md`
- checker family: `RS-TOOLCHAIN`

The generated result must satisfy:
- `RS-TOOLCHAIN-01`
- `RS-TOOLCHAIN-CONFIG-01`
- `RS-TOOLCHAIN-04`

It also participates in:
- `RS-TOOLCHAIN-CONFIG-02`

## Parity contract

1. `generate -> validate`
- generate root `rust-toolchain.toml`
- `RS-TOOLCHAIN` passes

2. `generate twice`
- second generation is byte-identical

3. negative mutation
- mutating channel or components produces the exact `RS-TOOLCHAIN-*` finding

4. scope exactness
- generator never creates legacy or nested toolchain files
