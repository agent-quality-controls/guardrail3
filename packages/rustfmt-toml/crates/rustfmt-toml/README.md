# rustfmt-toml

Typed parser for `rustfmt.toml` / `.rustfmt.toml` configuration files.

All known rustfmt options are represented as `Option<T>` fields so that absent keys parse as `None`. Unknown keys (e.g. nightly-only options) are captured in a catch-all `extra` map.

## Usage

```rust
use rustfmt_toml::RustfmtConfig;

// Parse from a string
let cfg = RustfmtConfig::from_str("max_width = 100\n")?;
assert_eq!(cfg.max_width, Some(100));

// Parse from a file path
let cfg = RustfmtConfig::from_path("rustfmt.toml")?;
```

## License

MIT OR Apache-2.0
