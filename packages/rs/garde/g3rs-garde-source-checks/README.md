# g3rs-garde-source-checks

Extracted garde source checks.

This package owns the garde rules that operate on governed Rust source files:

- `RS-GARDE-SOURCE-01`
- `RS-GARDE-SOURCE-02`
- `RS-GARDE-SOURCE-03`
- `RS-GARDE-SOURCE-04`
- `RS-GARDE-SOURCE-05`
- `RS-GARDE-SOURCE-06`
- `RS-GARDE-SOURCE-07`
- `RS-GARDE-SOURCE-10`

The package reads and analyzes governed source files from the explicit file
list in its input contract and consumes typed Rust-policy waiver state from
`guardrail3-rs.toml`.
