# cargo-toml-parser

Typed parser for `Cargo.toml`.

This package publishes the facade crate `cargo-toml-parser`. The facade
re-exports the public parser API from the internal runtime crate and the typed
schema from the internal types crate.

## What it parses

- top-level Cargo manifest keys such as `package`, `dependencies`, and `profile`
- known nested sections such as `workspace`, `target`, `lints`, and `patch`
- unknown keys through `extra` maps so the model stays forward-compatible

## Usage

```rust
use cargo_toml_parser::{parse, types::Dependency};

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

## Internal layout

- `crates/runtime` owns parsing and file loading
- `crates/types` owns the typed Cargo.toml schema
- `crates/assertions` owns shared proof helpers for runtime sidecar tests

## License

MIT OR Apache-2.0
