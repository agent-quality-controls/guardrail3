# cargo-toml-parser

Facade crate for typed `Cargo.toml` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use cargo_toml_parser::{Dependency, parse};

let manifest = parse(
    r#"
[package]
name = "demo"
edition = "2024"

[dependencies]
serde = "1"
"#,
)?;

assert!(matches!(
    manifest.dependencies.get("serde"),
    Some(Dependency::Simple(value)) if value == "1"
));
```

## License

MIT OR Apache-2.0
