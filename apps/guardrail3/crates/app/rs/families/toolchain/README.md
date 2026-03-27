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

### Current Rule Behavior

#### `RS-TOOLCHAIN-01`

- inventories when root `rust-toolchain.toml` exists
- errors when the modern file is missing, even if legacy `rust-toolchain` exists

#### `RS-TOOLCHAIN-02`

Channel policy:

- plain `stable` is accepted inventory
- `stable-<host>` is accepted inventory
- pinned stable versions such as `1.85.0` are accepted inventory
- pinned stable versions with host suffixes such as
  `1.85.0-x86_64-unknown-linux-gnu` are accepted inventory
- `nightly`, pinned-nightly, and version-like nightly suffix forms are errors
- `beta`, pinned-beta, and version-like beta suffix forms are errors
- unsupported string channels are errors
- missing `channel` is a warning
- non-string `channel` is an error

Components policy:

- `clippy` and `rustfmt` are required
- each present component is inventoried
- each missing required component is a warning
- non-array or mixed-type `components` values are errors

Input integrity:

- missing `[toolchain]` table is an error
- non-table `[toolchain]` shape is an error
- malformed `rust-toolchain.toml` is an error

#### `RS-TOOLCHAIN-03`

- activates only for pinned stable toolchain forms
- warns when pinned toolchain is older than `Cargo.toml` `rust-version`
- inventories when pinned toolchain satisfies declared MSRV
- inventories when `rust-version` is absent
- errors when root `Cargo.toml` is missing
- errors when root `Cargo.toml` is malformed
- errors when `rust-version` exists but is not a string
- errors when `rust-version` string is not a valid version

#### `RS-TOOLCHAIN-04`

- warns when legacy `rust-toolchain` exists
- also warns when both legacy and modern files coexist

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
- a small family-level `ProjectTree` smoke harness covers discovery and
  cross-rule interaction, not just direct rule helpers

## Self-Hosting Notes

The family carries its own root `rust-toolchain.toml` so validating the family
directory exercises the same root-level contract it enforces elsewhere.

Under `--inventory`, a clean self-hosted family still reports positive info
inventory for:

- `RS-TOOLCHAIN-01` modern file presence
- `RS-TOOLCHAIN-02` accepted channel
- `RS-TOOLCHAIN-02` required components present

So the meaningful “green” target is:

- `0 errors`
- `0 warnings`

not literal zero inventory output.

## Current State

As of the latest attack-hardening pass:

- the family is in the stabilized workspace shape
- rule-side coverage includes malformed active inputs and suffix-bypass attacks
- direct family workspace tests pass:

```bash
cargo test --manifest-path apps/guardrail3/crates/app/rs/families/toolchain/Cargo.toml --workspace --lib
```

This local family workspace also carries its own `workspace.lints` so it can be
tested directly even when unrelated top-level workspace issues exist elsewhere
in the repo.
