# cargo-toml-parser-types

Typed Cargo manifest schema for the `cargo-toml-parser` package.

This crate mirrors Cargo's manifest surface closely, including typed sections
for known keys and `toml::Value`-backed `extra` maps for forward-compatible
unknown keys. It exists so the runtime parser and the public facade share one
explicit data contract.

External consumers should depend on the facade crate `cargo-toml-parser`
instead of importing this crate directly.
