# g3rs-cargo-config-checks TODO

## Boundary Contract

- The package input is one parsed file only:
  - `Cargo.toml`
- The package decides whether workspace rules, package rules, or both apply
  from the parsed file content itself.

## Rule Split

Package-owned single-file config rules:

- `RS-CARGO-CONFIG-01`
- `RS-CARGO-CONFIG-02`
- `RS-CARGO-CONFIG-03`
- `RS-CARGO-CONFIG-04`
- `RS-CARGO-CONFIG-05`
- `RS-CARGO-CONFIG-06`

App-owned rules for now:

- `RS-CARGO-03`
- `RS-CARGO-04`
- `RS-CARGO-06`
- `RS-CARGO-09`
- `RS-CARGO-10`
- `RS-CARGO-12`
- `RS-CARGO-13`
- `RS-CARGO-14`
- `RS-CARGO-15`

Reason:
- `RS-CARGO-04` depends on workspace/member relationship context
- `RS-CARGO-06` and `RS-CARGO-09` are cross-file comparisons
- `RS-CARGO-10` and `RS-CARGO-14` are structural/input-failure rules
- `RS-CARGO-03`, `RS-CARGO-12`, `RS-CARGO-13`, and `RS-CARGO-15` still depend
  on app-owned policy data that should only move if the package takes the full
  parsed policy file

## Boundary Guard

- Do not reintroduce policy/profile/waiver subset helper types here.
- If policy-sensitive cargo rules move later, the package must take the full
  parsed policy file, not reduced policy projections.
