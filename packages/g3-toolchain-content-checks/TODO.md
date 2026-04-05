# g3-toolchain-content-checks TODO

## Known Issues

### Compile drift after `cargo-toml-parser` input shape changed

- The package currently fails to compile against the changed `cargo-toml-parser` types.
- `RS-TOOLCHAIN-03` still calls `.as_deref()` on `Option<InheritableValue<String>>`.

Relevant file:

- `packages/g3-toolchain-content-checks/crates/runtime/src/rs_toolchain_03_msrv_consistency/rule.rs`

Observed failure:

- `cargo test --manifest-path packages/g3-toolchain-content-checks/Cargo.toml --workspace -- --list`

Follow-up:

- Update `rust-version` extraction to handle `InheritableValue<String>` explicitly.
- Re-run package tests and the app `toolchain` family tests after the fix.

### Keep the package boundary on parsed file inputs

- The package should continue to receive parsed `RustToolchainToml` and parsed `CargoToml`.
- Fix the parser-API drift without reintroducing derived Cargo state as package input.

Follow-up:

- Verify the `RS-TOOLCHAIN-03` fix preserves the “full parsed files in, content checks only” boundary.
