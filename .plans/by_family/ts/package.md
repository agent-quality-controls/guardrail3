# TS-PACKAGE

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/package_check.rs`
- package/tool dependency ownership in `apps/guardrail3/crates/app/ts/validate/package_deps.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/package.md` as the detailed family ledger until the cutover is complete

Current state:

- package policy exists, but shares some tool/package surfaces with sibling families

Historical/supplemental references:

- `.plans/todo/checks/ts/package.md`

Next planning focus:

- clarify `package.json` key ownership across package, eslint, fmt, spelling, typecov, size, css, and tests

