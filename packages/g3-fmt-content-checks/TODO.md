# g3-fmt-content-checks TODO

## Known Issues

### Compile drift after `cargo-toml-parser` input shape changed

- The package currently fails to compile against the changed `cargo-toml-parser` types.
- `inputs.rs` still calls `.as_deref()` on `Option<InheritableValue<String>>`, which no longer works.

Relevant file:

- `packages/g3-fmt-content-checks/crates/runtime/src/inputs.rs`

Observed failure:

- `cargo test --manifest-path packages/g3-fmt-content-checks/Cargo.toml --workspace -- --list`

Follow-up:

- Update edition extraction to handle `InheritableValue<String>` explicitly.
- Re-run package tests and the app `fmt` family tests after the fix.

### Keep the package boundary on full parsed files

- The package should continue to receive full parsed files, not derived state enums or ad hoc scalar facts.
- After adapting to the parser change, verify the runtime still follows the agreed boundary:
  - parsed `RustfmtToml`
  - parsed `CargoToml`
  - parsed `RustToolchainToml`

Follow-up:

- Check that any fix for `InheritableValue` does not reintroduce scoped derived-state shortcuts.
