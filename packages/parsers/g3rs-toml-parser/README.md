# g3rs-toml-parser

Facade crate for typed `guardrail3-rs.toml` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use g3rs_toml_parser::parse;
use g3rs_toml_parser::types::RustProfile;

let cfg = parse(
    r#"
profile = "service"
allowed_deps = ["serde"]
"#,
)?;

assert_eq!(cfg.profile, Some(RustProfile::Service));
assert_eq!(cfg.allowed_deps, vec!["serde".to_owned()]);
```

## License

MIT OR Apache-2.0
