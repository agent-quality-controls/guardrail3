# cliff-toml-parser-runtime

Internal parser implementation for `cliff-toml-parser`.

This crate owns the typed error surface, filesystem boundary, and TOML parse
entrypoints used by the facade crate. External callers should depend on
`cliff-toml-parser`, not this runtime crate.
