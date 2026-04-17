#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use std::io::Write;

use release_plz_toml_parser_runtime::types::ReleasePlzToml;

pub fn parse_fixture(input: &str) -> ReleasePlzToml {
    release_plz_toml_parser_runtime::parse(input).expect("should parse valid release-plz.toml")
}

pub fn parse_from_tempfile(input: &str) -> ReleasePlzToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("release-plz config should be written");
    release_plz_toml_parser_runtime::from_path(file.path()).expect("file should parse")
}

/// Assert a boolean `Option` field matches an expected value.
pub fn assert_bool_field(actual: Option<bool>, expected: Option<bool>, field_name: &str) {
    assert_eq!(actual, expected, "{field_name} mismatch");
}

/// Assert a slice has the expected length.
pub fn assert_list_len<T>(items: &[T], expected: usize, field_name: &str) {
    assert_eq!(items.len(), expected, "{field_name} count mismatch");
}

/// Assert that a parse error contains the expected prefix.
pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid release-plz.toml"),
        "expected error message prefix, got: {msg}",
    );
}

/// Parse release-plz TOML content through the runtime parser.
///
/// # Errors
///
/// Returns the parser error when the input is not valid release-plz TOML.
pub fn parse_error(input: &str) -> Result<ReleasePlzToml, release_plz_toml_parser_runtime::Error> {
    release_plz_toml_parser_runtime::parse(input)
}

pub fn assert_empty_toml(cfg: &ReleasePlzToml) {
    assert!(cfg.workspace.is_none(), "workspace should be absent");
    assert!(cfg.package.is_empty(), "package list should be empty");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

pub fn assert_unknown_keys_preserved(cfg: &ReleasePlzToml) {
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
