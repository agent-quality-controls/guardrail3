# RS-TOOLCHAIN

Status: current, implemented, workspace-local family.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/toolchain/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/toolchain/README.md` for family-local behavior

Current state:

- workspace-local family over legal Rust workspaces plus toolchain-owned files
- implemented in `crates/runtime` and `crates/assertions`
- descendant toolchain files beneath a governed workspace root are explicit
  `RS-TOOLCHAIN-06` violations
- toolchain files outside any governed workspace root are explicit
  `RS-TOOLCHAIN-07` violations
- tree-wide stray-toolchain scans respect shared Rust exclusions such as
  `target/`, `tests/fixtures/`, `tests/snapshots/`, and `.claude/worktrees/`
- `guardrail3` repo reality: the governed workspace root is
  `apps/guardrail3`, so the repo-root `rust-toolchain.toml` is an illegal
  placement rather than an authority

Scope model:

- workspace-local family
- subtree validation should narrow to the owning legal workspace while keeping
  toolchain files outside or beneath workspaces visible

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/discover.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- prove nested-path runs resolve the correct owning legal workspace after
  the whole-project walker change
- prove shared discovery sees every `rust-toolchain.toml` / `rust-toolchain`
  in the live tree and the family partitions them correctly into:
  - owned workspace-root contracts
  - forbidden descendant overrides
  - forbidden out-of-workspace placements

Known current risk:

- rustup walk-up precedence means ancestor toolchain files can silently shadow a
  routed local policy root if the family stops checking for drift

Done means:

- subtree tests prove only the owning legal workspace roots are active
- workspace roots without toolchains fail closed
- descendant toolchains fail closed
- out-of-workspace toolchains fail closed
- ancestor/repo-root toolchain drift against a governed local root is enforced
- no direct tree-wide toolchain scan bypasses shared Rust exclusion rules

Historical/supplemental references:

- `.plans/todo/checks/rs/toolchain.md`
- `.plans/by_file/rs/rust-toolchain-toml.md`

Next planning focus:

- keep docs/tests aligned with workspace-root-only ownership
- keep toolchain/MSRV overlap with `RS-CARGO` explicit rather than letting the two families drift
- keep rustup walk-up precedence explicit, including same-directory legacy-file shadowing
- keep descendant workspace-subtree toolchain overrides and out-of-workspace
  toolchain files forbidden unless the family contract is deliberately widened
  in the future
