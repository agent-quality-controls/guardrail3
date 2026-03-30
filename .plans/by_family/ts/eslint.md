# TS-ESLINT

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_parser.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_rule_infra.rs`
- plugin/package portions of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/eslint.md` as the detailed family ledger until the cutover is complete

Current state:

- ESLint logic is substantial but still grouped under the old validator layout

Historical/supplemental references:

- `.plans/todo/checks/ts/eslint.md`
- `.plans/by_file/ts/eslint-config-mjs.md`

Next planning focus:

- separate baseline ESLint ownership from TS hexarch boundary policy

