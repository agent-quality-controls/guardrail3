# RS-TOOLCHAIN

Status: current, implemented, self-hosted, repository-root family.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/toolchain/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/toolchain/README.md` for family-local behavior

Current state:

- root-level family for `rust-toolchain.toml` / `rust-toolchain`
- self-hosted with `crates/runtime` and `crates/assertions`
- deliberately not a per-workspace/per-package discovery family unless a future architecture decision changes that

Historical/supplemental references:

- `.plans/todo/checks/rs/toolchain.md`
- `.plans/by_file/rs/rust-toolchain-toml.md`

Next planning focus:

- clean stale old-path references in the older ledger
- keep toolchain/MSRV overlap with `RS-CARGO` explicit rather than letting the two families drift
