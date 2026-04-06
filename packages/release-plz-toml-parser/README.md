# release-plz-toml-parser

Facade crate for typed `release-plz.toml` parsing.

The public API is exposed from this root crate. Internal parser and model crates
live under `crates/`.

## Usage

```rust
use release_plz_toml_parser::parse;

let cfg = parse(
    r#"
[workspace]
changelog_config = "cliff.toml"
git_release_enable = true

[[package]]
name = "my-crate"
"#,
)?;

assert_eq!(cfg.workspace.unwrap().changelog_config.as_deref(), Some("cliff.toml"));
assert_eq!(cfg.package.len(), 1);
```
