#![allow(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "parser tests use direct exact-shape assertions for concise contract proofs"
)]

use release_plz_toml_parser_runtime_assertions::parser as assertions;

use super::helpers;
use helpers::{parse_fixture, parse_from_tempfile};

#[test]
fn full_config_parses_all_fields() {
    let cfg = parse_fixture(
        r#"
[workspace]
changelog_config = "cliff.toml"
git_release_enable = true
release_always = false

[[package]]
name = "my-crate"
"#,
    );

    let ws = cfg
        .workspace
        .as_ref()
        .expect("workspace section should be present");
    assert_eq!(
        ws.changelog_config.as_deref(),
        Some("cliff.toml"),
        "changelog_config should match",
    );
    assertions::assert_bool_field(ws.git_release_enable, Some(true), "git_release_enable");
    assertions::assert_bool_field(ws.release_always, Some(false), "release_always");

    assertions::assert_list_len(&cfg.package, 1, "package");
    assert_eq!(
        cfg.package.first().expect("first package should exist").name.as_deref(),
        Some("my-crate"),
        "package name should match",
    );
}

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assert!(cfg.workspace.is_none(), "workspace should be None for empty input");
    assert!(cfg.package.is_empty(), "package list should be empty for empty input");
    assert!(cfg.extra.is_empty(), "extra should be empty for empty input");
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse_fixture(
        r#"
some_future_option = "yes"

[workspace]
changelog_config = "cliff.toml"
some_workspace_future_key = 42

[[package]]
name = "my-crate"
some_package_future_key = true
"#,
    );

    assert!(
        cfg.extra.contains_key("some_future_option"),
        "top-level unknown key should land in extra",
    );

    let ws = cfg
        .workspace
        .as_ref()
        .expect("workspace section should be present");
    assert!(
        ws.extra.contains_key("some_workspace_future_key"),
        "workspace unknown key should land in workspace extra",
    );

    let pkg = cfg.package.first().expect("first package should exist");
    assert!(
        pkg.extra.contains_key("some_package_future_key"),
        "package unknown key should land in package extra",
    );
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r#"
[workspace]
git_release_enable = true
"#,
    );

    let ws = cfg
        .workspace
        .as_ref()
        .expect("workspace section should be present after from_path");
    assertions::assert_bool_field(ws.git_release_enable, Some(true), "git_release_enable");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
