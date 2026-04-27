# g3rs-garde-source-checks

Extracted garde source checks.

This package owns the garde rules that operate on governed Rust source files:

- `g3rs-garde/struct-derive-validate`
- `g3rs-garde/manual-deserialize-impl`
- `g3rs-garde/enum-derive-validate`
- `g3rs-garde/query-as-inventory`
- `g3rs-garde/field-level-constraints`
- `g3rs-garde/nested-validation-dive`
- `g3rs-garde/context-validation-surface`
- `g3rs-garde/input-failures`

The package reads and analyzes governed source files from the explicit file
list in its input contract and consumes typed Rust-policy waiver state from
`guardrail3-rs.toml`.
