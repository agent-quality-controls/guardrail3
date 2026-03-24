# Rust Generator — `deny`

## Generated artifacts

- `<rust_policy_root>/deny.toml`

The generator does not own:
- `.deny.toml`
- `.cargo/deny.toml`

## Ownership mode

- exact-owned

## Root selection

`deny` is a Rust policy-root family.

The generator owns one `deny.toml` at every Rust policy root:
- validation root when it is itself a Rust workspace root or a standalone Rust package root
- each nested Rust workspace root
- each standalone Rust package root that is not a member of any workspace

The generator must never create `deny.toml` at:
- workspace member roots
- inner hex structural roots that are not workspace/package roots
- arbitrary ancestor or intermediate directories
- sibling alternative deny filenames

This contract must hold in mixed repositories containing:
- a root packages workspace
- multiple nested app workspaces
- standalone Rust roots
- non-Rust apps beside Rust roots

## Required generator contract

- every owned Rust policy root gets exactly one generator-owned `deny.toml`
- each generated file encodes the exact local deny baseline for that root
- service and library roots receive the correct local ban/license/source surface
- generated content includes all required top-level sections owned by `RS-DENY`
- root-local deny override data is validated, deduplicated, and merged into the correct section exactly once
- generator never creates forbidden nested shadow configs or lower-precedence sibling configs

## Checker target

- `.plans/todo/checks/rs/deny.md`
- checker family: `RS-DENY`

The generated result must satisfy the config-side `RS-DENY` contract for:
- coverage
- placement
- local-root parity
- ban baseline
- feature-ban baseline
- license policy
- source policy

## Parity contract

1. `generate -> validate`
- generate `deny.toml` at every owned Rust policy root
- `RS-DENY` passes for the full repository shape

2. `generate twice`
- second generation is byte-identical for unchanged inputs

3. negative mutation
- mutating one generated deny/license/source surface produces the exact `RS-DENY-*` finding for that surface

4. root exactness
- root packages workspace, nested app workspaces, and standalone package roots each receive exactly the files they own
- workspace members and inner hex structural roots receive none
