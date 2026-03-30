# TS-LIBARCH

Status: planned family contract, no cohesive family runtime yet.

Implementation roots:

- library/package concerns are currently scattered across:
  - `apps/guardrail3/crates/app/ts/validate/package_check.rs`
  - `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - app-type handling in `apps/guardrail3/crates/app/ts/validate/mod.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/libarch.md` as the detailed family ledger until the cutover is complete

Current state:

- library/package architecture is still a design lane, not a distinct runtime family

Historical/supplemental references:

- `.plans/todo/checks/ts/libarch.md`

Next planning focus:

- define concrete library root detection and package-boundary architecture inputs

