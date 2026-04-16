# cargo-config-toml-parser-runtime

Runtime crate for the `cargo-config-toml-parser` package.

This crate owns:

- the typed parse entrypoints `parse` and `from_path`
- the public parser error type
- the sidecar tests for the parser module

It is part of the package's internal split. External consumers should depend on
the facade crate `cargo-config-toml-parser` instead of importing this crate
directly.
