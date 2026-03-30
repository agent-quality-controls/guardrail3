# RS-DENY

Status: current, implemented, self-hosted.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/deny/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/deny/README.md` for family-local behavior

Current state:

- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- owns cargo-deny config coverage, placement, shadowing, and deny-policy semantics
- by-file deny docs remain research on tool behavior, not the family contract

Historical/supplemental references:

- `.plans/todo/checks/rs/deny.md`
- `.plans/by_file/rs/deny-toml.md`
- `.plans/by_file/tools/cargo-deny.md`

Next planning focus:

- finish the same doc cleanup done for `arch`/`clippy` if stale implementation pointers remain in the old ledger
- add a stronger family-local README/source-of-truth note if deny grows more generator coupling
