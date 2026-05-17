# cargo-config-toml-parser

Typed parser for `.cargo/config.toml` and legacy `.cargo/config`.

This package publishes the facade crate `cargo-config-toml-parser`. The facade
re-exports the public parser API from the internal runtime crate and the typed
schema from the internal types crate.

## What it parses

- top-level Cargo config keys such as `paths`, `include`, and `alias`
- known nested sections such as `build`, `env`, `http`, `net`, `registry`, and `target`
- unknown keys through `extra` maps so the model stays forward-compatible

## Usage

```rust
use cargo_config_toml_parser::{parse, types::EnvValue};

let cfg = parse(
    r#"
[env]
CLIPPY_CONF_DIR = "."
"#,
)?;

assert!(matches!(
    cfg.env.get("CLIPPY_CONF_DIR"),
    Some(EnvValue::Simple(value)) if value == "."
));
```

## Internal layout

- `crates/runtime` owns parsing and file loading
- `crates/types` owns the typed Cargo config schema

## License

MIT OR Apache-2.0
