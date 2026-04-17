# deny-toml-parser

Typed parser facade for `deny.toml`.

The root crate exposes only the parsing entrypoints:
- `parse`
- `from_path`
- `Error`

Schema types live under `deny_toml_parser::types`.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use deny_toml_parser::parse;

let cfg = parse(
    r#"
[graph]
all-features = true
"#,
)?;

assert_eq!(cfg.graph.as_ref().and_then(|graph| graph.all_features), Some(true));
```
