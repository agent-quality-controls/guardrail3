#![allow(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "parser tests use direct exact-shape assertions for concise contract proofs"
)]

use release_plz_toml_parser_runtime_assertions::parser as assertions;

#[test]
fn full_config_parses_all_fields() {
    let cfg = assertions::parse_fixture(
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
        cfg.package
            .first()
            .expect("first package should exist")
            .name
            .as_deref(),
        Some("my-crate"),
        "package name should match",
    );
}

#[test]
fn empty_string_yields_empty_config() {
    let cfg = assertions::parse_fixture("");

    assertions::assert_empty_toml(&cfg);
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = assertions::parse_fixture(
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

    assertions::assert_unknown_keys_preserved(&cfg);
}

#[test]
fn from_path_reads_and_parses_file() {
    use std::io::Write as _;

    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(
        br"
[workspace]
git_release_enable = true
",
    )
    .expect("release-plz config should be written");
    let cfg = crate::parser::from_path(file.path()).expect("file should parse");

    let ws = cfg
        .workspace
        .as_ref()
        .expect("workspace section should be present after from_path");
    assertions::assert_bool_field(ws.git_release_enable, Some(true), "git_release_enable");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = assertions::parse_error("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
