# g3rs-toolchain-config-checks TODO

## Deferred Boundary Work

### Structural rules remain app-side

- `RS-TOOLCHAIN-01` root config discovery / existence
- `RS-TOOLCHAIN-04` placement / duplicate-surface checks

Reason:
- these are discovery and structural rules, not pure config checks over
  already-selected parsed files

## Boundary Guard

- Keep the package inputs on parsed files only:
  - parsed `RustToolchainToml`
  - parsed `CargoToml`
- Do not reintroduce reduced Cargo state such as extracted rust-version enums
  or other scoped helper inputs.
