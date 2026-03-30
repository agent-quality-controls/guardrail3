# RS-TOOLCHAIN

Status: current, implemented, self-hosted, routed policy-root family.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/toolchain/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/toolchain/README.md` for family-local behavior

Current state:

- routed family over Rust policy roots
- self-hosted with `crates/runtime` and `crates/assertions`
- owned roots are workspace roots plus standalone package roots that are not
  claimed as workspace members
- `guardrail3` repo reality: the governed workspace root is
  `apps/guardrail3`, not the repo root

Historical/supplemental references:

- `.plans/todo/checks/rs/toolchain.md`
- `.plans/by_file/rs/rust-toolchain-toml.md`

Next planning focus:

- keep docs/tests aligned with routed policy-root ownership
- keep toolchain/MSRV overlap with `RS-CARGO` explicit rather than letting the two families drift
