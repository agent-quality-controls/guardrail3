# TS-CODE

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/source_scan.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_comment_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_code_analysis.rs`
- `apps/guardrail3/crates/app/ts/validate/ast_helpers.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/code.md` as the detailed family ledger until the cutover is complete

Current state:

- source scanning exists, but still lives in the old grouped TS validator
- no dedicated family workspace/README yet

Historical/supplemental references:

- `.plans/todo/checks/ts/code.md`

Next planning focus:

- separate generic TS source rules from architecture, tests, i18n, and content concerns

