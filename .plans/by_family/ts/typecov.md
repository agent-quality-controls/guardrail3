# TS-TYPECOV

Status: current family contract, partial legacy implementation only.

Implementation roots:

- type-coverage-related parts of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- type-coverage-related parts of `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/typecov.md` as the detailed family ledger until the cutover is complete

Current state:

- type-coverage enforcement exists only as mixed tool/package logic

Historical/supplemental references:

- `.plans/todo/checks/ts/typecov.md`

Next planning focus:

- define the exact threshold/config ownership split against `ts/tsconfig`

