# cliff-toml-parser-types

Internal typed model crate for `cliff-toml-parser`.

This crate owns the `cliff.toml` schema mirror used by the runtime parser.
External callers should use the facade crate and its `types` module instead of
depending on this internal crate directly.
