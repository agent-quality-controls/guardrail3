use cargo_toml_parser_runtime_assertions::parser as assertions;

use super::helpers::{parse_fixture, parse_from_tempfile};

#[test]
fn empty_string_yields_empty_manifest() {
    let manifest = parse_fixture("");
    assertions::assert_manifest_empty(&manifest);
}

#[test]
fn package_workspace_lints_and_dependencies_parse() {
    let manifest = parse_fixture(
        r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
future-key = "keep-me"

[workspace]
members = ["crates/*"]
exclude = ["legacy"]
resolver = "2"

[workspace.package]
edition = "2024"
rust-version = "1.85"

[workspace.lints.rust]
unsafe_code = "forbid"

[lints]
workspace = true

[lints.clippy]
unwrap_used = "deny"
all = { level = "deny", priority = -1 }

[dependencies]
serde = "1"
toml = { version = "0.8", features = ["parse"] }

[target.'cfg(unix)'.dependencies]
libc = "0.2"
"#,
    );

    let package = manifest.package.as_ref().expect("package should exist");
    assert_eq!(package.name.as_deref(), Some("demo"));
    assert_eq!(package.edition.as_deref(), Some("2024"));
    assert_eq!(
        package.extra.get("future-key").and_then(toml::Value::as_str),
        Some("keep-me"),
    );

    let workspace = manifest.workspace.as_ref().expect("workspace should exist");
    assert_eq!(workspace.members, vec!["crates/*".to_owned()]);
    assert_eq!(workspace.exclude, vec!["legacy".to_owned()]);
    assertions::assert_lint_level(
        workspace
            .lints
            .as_ref()
            .and_then(|lints| lints.rust.get("unsafe_code")),
        "forbid",
        "workspace.lints.rust.unsafe_code",
    );
    assert_eq!(
        manifest.lints.as_ref().and_then(|lints| lints.workspace),
        Some(true),
    );
    assertions::assert_lint_level(
        manifest
            .lints
            .as_ref()
            .and_then(|lints| lints.clippy.get("unwrap_used")),
        "deny",
        "lints.clippy.unwrap_used",
    );
    assertions::assert_simple_dep(
        manifest.dependencies.get("serde"),
        "1",
        "dependencies.serde",
    );
    assertions::assert_detailed_dep_version(
        manifest.dependencies.get("toml"),
        "0.8",
        "dependencies.toml",
    );
    assertions::assert_simple_dep(
        manifest
            .target
            .get("cfg(unix)")
            .and_then(|target| target.dependencies.get("libc")),
        "0.2",
        "target.cfg(unix).dependencies.libc",
    );
}

#[test]
fn from_path_reads_and_parses_file() {
    let manifest = parse_from_tempfile(
        r#"
[package]
name = "demo"
edition = "2024"
"#,
    );

    assert_eq!(
        manifest.package.as_ref().and_then(|package| package.name.as_deref()),
        Some("demo"),
    );
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = crate::parse("this is not [[[valid toml").expect_err("invalid Cargo.toml should fail");
    assertions::assert_parse_error(err);
}
