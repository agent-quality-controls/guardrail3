# rustfmt-toml-parser

Facade crate for typed `rustfmt.toml` / `.rustfmt.toml` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use rustfmt_toml_parser::parse;
use rustfmt_toml_parser::types::Edition;

let cfg = parse(
    r#"
max_width = 100
edition = "2021"
"#,
)?;

assert_eq!(cfg.max_width, Some(100));
assert_eq!(cfg.edition, Some(Edition::Edition2021));
```

## License

MIT OR Apache-2.0
