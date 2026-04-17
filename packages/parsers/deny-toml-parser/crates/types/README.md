# deny-toml-parser-types

Internal typed model crate for `deny-toml-parser`.

It mirrors the `deny.toml` schema and is re-exported through the facade's
`types` module. External callers should use `deny_toml_parser::types`.
