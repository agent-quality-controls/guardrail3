# mutants-toml-parser

Facade crate for typed `.cargo/mutants.toml` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use mutants_toml_parser::parse;

let cfg = parse(
    r#"
timeout_multiplier = 3.0
test_tool = "nextest"
"#,
)?;

assert_eq!(cfg.timeout_multiplier, Some(3.0));
assert_eq!(cfg.test_tool.as_deref(), Some("nextest"));
```

## License

MIT OR Apache-2.0
