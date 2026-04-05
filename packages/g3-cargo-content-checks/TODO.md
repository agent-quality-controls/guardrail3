# g3-cargo-content-checks TODO

## Boundary contract

The package input is one parsed file only:

- `Cargo.toml`

The app still owns:

- file discovery
- workspace/member routing
- missing-member detection
- parse-failure routing
- cross-file comparisons between workspace and member manifests
- parsing and providing full `guardrail3-rs.toml` if policy-file-sensitive cargo
  rules are ever moved later

## Rule split

Package-owned content rules that fit a single parsed `Cargo.toml`:

- `RS-CARGO-01`
- `RS-CARGO-02`
- `RS-CARGO-05`
- `RS-CARGO-07`
- `RS-CARGO-08`
- `RS-CARGO-11`

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
- `RS-CARGO-03`, `RS-CARGO-12`, `RS-CARGO-13`, and `RS-CARGO-15` depend on
  policy data that must come from the full parsed `guardrail3-rs.toml` file,
  not subset/profile/waiver helper types

## Next step

After the package boundary is stable:

1. delete all invented cargo subset/policy helper types from the package
2. extract only the single-file cargo rules first
3. leave cross-file and policy-file-sensitive rules in the app until they can
   take full parsed-file inputs cleanly
