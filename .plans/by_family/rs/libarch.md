# RS-LIBARCH

Status: current, implemented, self-hosted, newly live.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/libarch/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/libarch/README.md` for family-local behavior
- `.plans/todo/checks/rs/libarch.md` as the detailed rule ledger and design history

Current state:

- self-hosted with:
  - `crates/runtime`
  - `crates/assertions`
  - `test_support`
- runtime/model/config/reporting selection already know `libarch`
- the family now owns layered-library escalation, layered workspace shape, layer dependency direction, and root facade export policy for package-owned library roots
- the old detailed design ledger remains useful as history, but it is no longer describing a hypothetical family

Historical/supplemental references:

- `.plans/todo/checks/rs/libarch.md`
- `.plans/todo/family-implementation-handoffs/libarch.md`
- `arch` and `hexarch` docs where package/app ownership boundaries are already described

Next planning focus:

- keep package architecture separate from generic Cargo policy and from repo-global `arch`
- pressure fail-closed and dependency-direction edges as the package zone evolves
