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
copy_target = true
test_tool = "nextest"
"#,
)?;

assert_eq!(cfg.timeout_multiplier, Some(3.0));
assert_eq!(cfg.copy_target, Some(true));
assert_eq!(cfg.test_tool, Some(mutants_toml_parser::TestTool::Nextest));
```

## License

MIT OR Apache-2.0
