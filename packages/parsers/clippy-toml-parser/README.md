# clippy-toml-parser

Facade crate for typed `clippy.toml` / `.clippy.toml` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use clippy_toml_parser::parse;

let cfg = parse(
    r#"
max-struct-bools = 3
allow-expect-in-tests = true
"#,
)?;

assert_eq!(cfg.max_struct_bools, Some(3));
assert_eq!(cfg.allow_expect_in_tests, Some(true));
```
