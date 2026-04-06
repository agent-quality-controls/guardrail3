# g3rs-fmt-config-checks TODO

## Deferred Boundary Work

### `RS-FMT-07` remains app-side

- `RS-FMT-07` depends on documented waiver / escape-hatch policy, not only on
  parsed `rustfmt.toml` content.
- Keep it in the app until waiver ownership is intentionally moved using the
  full parsed policy file, not derived waiver subsets.

### Structural and placement rules remain app-side

- `RS-FMT-01` root config existence
- `RS-FMT-05` per-crate override placement
- `RS-FMT-08` dual-file conflict

Reason:
- these are discovery / placement / structural rules, not pure config checks

## Boundary Guard

- Keep this package on full parsed files only:
  - parsed `RustfmtToml`
  - parsed `CargoToml`
  - parsed `RustToolchainToml`
- Do not regress to derived edition/channel state enums or scalar facts just to
  make rule plumbing smaller.
