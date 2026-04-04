# cargo-config-toml-parser

Facade crate for typed `.cargo/config.toml` / `.cargo/config` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use cargo_config_toml_parser::{EnvValue, parse};

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

## License

MIT OR Apache-2.0
