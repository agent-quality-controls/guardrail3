# cliff-toml-parser

Facade crate for typed `cliff.toml` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use cliff_toml_parser::parse;

let cfg = parse(
    r#"
[git]
conventional_commits = true

[changelog]
header = "# Changelog"
"#,
)?;

assert_eq!(cfg.git.unwrap().conventional_commits, Some(true));
assert_eq!(cfg.changelog.unwrap().header.as_deref(), Some("# Changelog"));
```
