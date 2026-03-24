# Rust Generator — `cargo`

## Generated artifacts

- `<rust_policy_root>/Cargo.toml` patched in place

## Ownership mode

- semantic-patch

## Root selection

`cargo` is a Rust policy-root family.

The generator patches `Cargo.toml` only at Rust policy roots:
- workspace roots
- standalone Rust package roots that are not members of any workspace

It must never patch:
- workspace member manifests for policy-root sections
- non-Rust roots
- inner hex structural roots that are not workspace/package roots
- arbitrary ancestor manifests that do not own the local Rust policy

This contract must hold in mixed repositories containing:
- a root packages workspace
- nested app workspaces
- standalone Rust roots
- non-Rust apps beside Rust roots

## Required generator contract

For workspace roots, the generator owns:
- `[workspace.lints.rust]`
- `[workspace.lints.clippy]`
- explicit workspace `resolver`
- the policy-root edition and `rust-version` / MSRV surfaces owned by `RS-CARGO`

For standalone package roots, the generator owns:
- `[lints.rust]`
- `[lints.clippy]`
- the policy-root edition and `rust-version` / MSRV surfaces owned by `RS-CARGO`

The generator must:
- write the full canonical lint baseline required by `RS-CARGO`
- preserve unrelated manifest content outside the owned sections
- leave non-owned keys and tables semantically unchanged
- stabilize the owned sections on repeated runs
- never invent workspace-only sections on standalone package roots

## Checker target

- `.plans/todo/checks/rs/cargo.md`
- checker family: `RS-CARGO`

The generated result must satisfy:
- `RS-CARGO-01`
- `RS-CARGO-02`
- `RS-CARGO-03`
- `RS-CARGO-05`
- `RS-CARGO-07`
- `RS-CARGO-08`
- `RS-CARGO-11`
- `RS-CARGO-12`
- `RS-CARGO-13`
- `RS-CARGO-15`

## Parity contract

1. `generate -> validate`
- patch every owned policy-root `Cargo.toml`
- `RS-CARGO` passes for the full repository shape

2. `generate twice`
- a second patch run is semantically stable
- non-owned content is preserved
- owned sections normalize to the same result

3. negative mutation
- weakening or deleting one generated cargo-owned surface produces the exact `RS-CARGO-*` finding for that surface

4. root exactness
- workspace roots and standalone package roots are patched exactly once
- workspace members are checked through their owning workspace and are not treated as separate cargo policy roots
