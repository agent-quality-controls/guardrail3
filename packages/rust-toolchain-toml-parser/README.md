# rust-toolchain-toml-parser

Facade crate for typed `rust-toolchain.toml` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use rust_toolchain_toml_parser::parse;

let cfg = parse(
    r#"
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
"#,
)?;

assert_eq!(
    cfg.toolchain().and_then(|section| section.channel()),
    Some("stable")
);
```

## License

MIT OR Apache-2.0
