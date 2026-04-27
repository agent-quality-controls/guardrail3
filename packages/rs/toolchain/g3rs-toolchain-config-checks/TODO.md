# g3rs-toolchain-config-checks TODO

## Deferred Boundary Work

### Structural rules remain app-side

- `g3rs-toolchain/root-toolchain-config-exists` root config discovery / existence
- `g3rs-toolchain/no-duplicate-toolchain-config` placement / duplicate-surface checks

Reason:
- these are discovery and structural rules, not pure config checks over
  already-selected parsed files

## Boundary Guard

- Keep the package inputs on parsed files only:
  - parsed `RustToolchainToml`
  - parsed `CargoToml`
- Do not reintroduce reduced Cargo state such as extracted rust-version enums
  or other scoped helper inputs.
