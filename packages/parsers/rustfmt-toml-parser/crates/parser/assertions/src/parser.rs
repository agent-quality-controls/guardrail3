#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use rustfmt_toml_parser_runtime::{RustfmtToml, Value};

pub fn assert_core_fields_empty(cfg: &RustfmtToml) {
    assert_eq!(
        cfg.max_width, None,
        "max_width should be None for empty input"
    );
    assert_eq!(
        cfg.hard_tabs, None,
        "hard_tabs should be None for empty input"
    );
    assert_eq!(cfg.edition, None, "edition should be None for empty input");
    assert_eq!(
        cfg.newline_style, None,
        "newline_style should be None for empty input"
    );
}

pub fn assert_collections_empty(cfg: &RustfmtToml) {
    assert!(
        cfg.ignore.is_empty(),
        "ignore should be empty for empty input"
    );
    assert!(
        cfg.skip_macro_invocations.is_empty(),
        "skip_macro_invocations should be empty"
    );
}

pub fn assert_extra_empty(cfg: &RustfmtToml) {
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

pub fn assert_basic_width_fields(
    cfg: &RustfmtToml,
    max_width: Option<u32>,
    hard_tabs: Option<bool>,
    tab_spaces: Option<u32>,
) {
    assert_eq!(cfg.max_width, max_width, "max_width mismatch");
    assert_eq!(cfg.hard_tabs, hard_tabs, "hard_tabs mismatch");
    assert_eq!(cfg.tab_spaces, tab_spaces, "tab_spaces mismatch");
}

pub fn assert_string_field(actual: Option<&str>, expected: Option<&str>, field_name: &str) {
    assert_eq!(actual, expected, "{field_name} mismatch");
}

pub fn assert_bool_field(actual: Option<bool>, expected: Option<bool>, field_name: &str) {
    assert_eq!(actual, expected, "{field_name} mismatch");
}

pub fn assert_string_list(actual: &[String], expected: &[&str], field_name: &str) {
    let expected_values = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(actual, expected_values, "{field_name} mismatch");
}

pub fn assert_top_level_string_extra(cfg: &RustfmtToml, key: &str, expected: &str) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "top-level extra key should be preserved",
    );
}

pub fn assert_top_level_integer_extra(cfg: &RustfmtToml, key: &str, expected: i64) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_integer),
        Some(expected),
        "top-level extra key should be preserved",
    );
}

pub fn assert_tomls_equal(left: &RustfmtToml, right: &RustfmtToml) {
    assert_eq!(left, right, "roundtrip should produce identical config");
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid rustfmt.toml"),
        "expected error message prefix, got: {msg}",
    );
}
