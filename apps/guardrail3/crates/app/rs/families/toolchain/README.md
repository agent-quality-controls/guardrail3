# RS-TOOLCHAIN

Rust toolchain contract family.

This family is workspace-local. It owns legal workspaces plus all toolchain
files relevant to those workspaces.

For each owned root, the contract is:

- one local `rust-toolchain.toml`
- optional local legacy `rust-toolchain`
- one local `Cargo.toml` source for MSRV comparison
- no drifting ancestor toolchain that can shadow the local workspace contract

Nested non-member packages inside an owned workspace subtree inherit the
workspace toolchain by rustup walk-up. They are not required to carry their own
local `rust-toolchain.toml`, and they are not allowed to define one.

## What This Family Enforces

- `RS-TOOLCHAIN-01`: `rust-toolchain.toml` exists at each owned policy root
- `RS-TOOLCHAIN-02`: channel and component policy
- `RS-TOOLCHAIN-03`: pinned stable toolchain vs owned-root `Cargo.toml`
  `rust-version`
- `RS-TOOLCHAIN-04`: legacy `rust-toolchain` migration and same-root shadowing
- `RS-TOOLCHAIN-05`: ancestor shadow drift across legal workspace policy roots
- `RS-TOOLCHAIN-06`: descendant workspace-shadowing toolchain files
- `RS-TOOLCHAIN-07`: toolchain files outside every governed workspace root

### Current Rule Behavior

#### `RS-TOOLCHAIN-01`

- inventories when the owned-root `rust-toolchain.toml` exists
- errors when the owned-root modern file is missing, even if a local legacy
  `rust-toolchain` exists
- a parent/repo-root toolchain file does not satisfy a governed workspace root

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
- warns when pinned toolchain is older than the owned-root `Cargo.toml`
  `rust-version`
- inventories when pinned toolchain satisfies declared MSRV
- inventories when `rust-version` is absent
- errors when owned-root `Cargo.toml` is missing
- errors when owned-root `Cargo.toml` is malformed
- errors when `rust-version` exists but is not a string
- errors when `rust-version` string is not a valid version

#### `RS-TOOLCHAIN-04`

- warns when legacy `rust-toolchain` exists
- errors when both legacy and modern files coexist, because rustup prefers the
  legacy file and shadows the modern contract

#### `RS-TOOLCHAIN-05`

- warns when an ancestor legacy `rust-toolchain` can shadow a local
  `rust-toolchain.toml`
- warns when an ancestor `rust-toolchain.toml` is malformed
- warns when an ancestor `rust-toolchain.toml` differs from the local routed
  policy-root contract
- emits nothing when the nearest ancestor toolchain semantically matches the
  local routed-root contract

#### `RS-TOOLCHAIN-06`

- errors when a governed workspace subtree contains a descendant
  `rust-toolchain.toml`
- errors when a governed workspace subtree contains a descendant legacy
  `rust-toolchain`
- treats any nested toolchain beneath a governed workspace root as a policy
  escape hatch that destabilizes the workspace contract

#### `RS-TOOLCHAIN-07`

- errors when `rust-toolchain.toml` or `rust-toolchain` exists outside every
  governed workspace root
- respects shared Rust exclusions such as `target/`, `tests/fixtures/`,
  `tests/snapshots/`, and `.claude/worktrees/`
- catches repo-root toolchains in repos where the repo root is not itself a
  governed Rust workspace

## Layout

```text
toolchain/
  README.md
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
- a family-level full-tree fixture harness proves:
  - `ProjectTree` sees injected toolchain files
  - owned workspace roots inventory cleanly
  - nested descendant toolchains become `RS-TOOLCHAIN-06`
  - out-of-workspace toolchains become `RS-TOOLCHAIN-07`

## Current State

As of the latest attack-hardening pass:

- the family is routed through placement/family-mapper instead of assuming the
  validation root is the policy root
- rule-side coverage includes malformed active inputs and suffix-bypass attacks
- same-directory legacy-shadow cases now suppress false modern-file inventory in
  `RS-TOOLCHAIN-02` and `RS-TOOLCHAIN-03`
- ancestor walk-up drift is enforced explicitly so repo-root toolchains cannot
  silently diverge from governed workspace toolchain contracts
- descendant toolchain files anywhere beneath a governed workspace root are now
  rejected as workspace-policy shadowing
- out-of-workspace toolchain files are now rejected, while `.claude/worktrees`
  and the other shared Rust exclusions stay out of scope
- full-tree golden-fixture tests prove the walker and the family agree on
  toolchain-file visibility and ownership
- direct family package tests pass:

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-toolchain --lib
```

The runtime and assertions crates live inside the top-level
`apps/guardrail3/Cargo.toml` workspace rather than a family-local workspace
manifest.
