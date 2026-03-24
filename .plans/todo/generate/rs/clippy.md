# Rust Generator — `clippy`

## Generated artifacts

- `<rust_policy_root>/clippy.toml`

No `.clippy.toml` files are generator-owned.

## Ownership mode

- exact-owned

## Root selection

`clippy` is a Rust policy-root family.

The generator owns one `clippy.toml` at every Rust policy root:
- validation root when it is itself a Rust workspace root or a standalone Rust package root
- each nested Rust workspace root
- each standalone Rust package root that is not a member of any workspace

The generator must never create `clippy.toml` at:
- workspace member roots
- inner hex structural roots that are not workspace/package roots
- arbitrary ancestor or intermediate directories
- sibling `.clippy.toml` locations

This contract must hold in mixed repositories containing:
- a root packages workspace
- multiple nested app workspaces
- standalone Rust roots
- non-Rust apps beside Rust roots

## Required generator contract

- every owned Rust policy root gets exactly one generator-owned `clippy.toml`
- each generated file encodes the exact local clippy policy for that root
- profile-sensitive generation is exact per root:
  - service profile
  - library profile
  - pure-layer service roots where local policy requires pure-layer additions
- garde-sensitive generation is exact per root
- generated content includes the full guardrail-managed clippy baseline for that root
- root-local clippy override data is validated, deduplicated, and merged exactly once
- generator never creates lower-precedence sibling files or forbidden nested local shadow configs

## Checker target

- `.plans/todo/checks/rs/clippy.md`
- checker family: `RS-CLIPPY`

The generated result must satisfy the config-side `RS-CLIPPY` contract for:
- coverage
- placement
- local-root parity
- threshold exactness
- setting exactness
- method/type/macro baseline completeness

## Parity contract

1. `generate -> validate`
- generate `clippy.toml` at every owned Rust policy root
- `RS-CLIPPY` passes for the full repository shape

2. `generate twice`
- second generation is byte-identical for unchanged inputs

3. negative mutation
- mutating one generated threshold, setting, or ban entry produces the exact `RS-CLIPPY-*` finding for that surface

4. root exactness
- root packages workspace, nested app workspaces, and standalone package roots each receive exactly the files they own
- workspace members and inner hex structural roots receive none
