# deny-toml-parser

Facade crate for typed `deny.toml` parsing.

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
