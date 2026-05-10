#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use std::io::Write;

pub use rustfmt_toml_parser_runtime::types::Color;
pub use rustfmt_toml_parser_runtime::types::Edition;
pub use rustfmt_toml_parser_runtime::types::EmitMode;
pub use rustfmt_toml_parser_runtime::types::Heuristics;
pub use rustfmt_toml_parser_runtime::types::NewlineStyle;

use rustfmt_toml_parser_runtime::Value;
use rustfmt_toml_parser_runtime::types::RustfmtToml;

#[must_use]
pub fn parse_fixture(input: &str) -> RustfmtToml {
    rustfmt_toml_parser_runtime::parse(input).expect("should parse valid rustfmt.toml")
}

#[must_use]
pub fn parse_from_tempfile(input: &str) -> RustfmtToml {
    let mut file = tempfile::NamedTempFile::new().expect("tempfile should be created");
    file.write_all(input.as_bytes())
        .expect("rustfmt config should be written");
    rustfmt_toml_parser_runtime::from_path(file.path()).expect("file should parse")
}

/// Parse rustfmt TOML content through the runtime parser.
///
/// # Errors
///
/// Returns the parser error when the input is not valid rustfmt TOML.
pub fn parse_error(input: &str) -> Result<RustfmtToml, rustfmt_toml_parser_runtime::Error> {
    rustfmt_toml_parser_runtime::parse(input)
}

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

pub fn assert_edition(cfg: &RustfmtToml, expected: Option<Edition>) {
    assert_eq!(cfg.edition, expected, "edition mismatch");
}

pub fn assert_newline_style(cfg: &RustfmtToml, expected: Option<NewlineStyle>) {
    assert_eq!(cfg.newline_style, expected, "newline_style mismatch");
}

pub fn assert_use_small_heuristics(cfg: &RustfmtToml, expected: Option<Heuristics>) {
    assert_eq!(
        cfg.use_small_heuristics, expected,
        "use_small_heuristics mismatch",
    );
}

pub fn assert_blank_line_bounds(cfg: &RustfmtToml, lower: Option<u32>, upper: Option<u32>) {
    assert_eq!(
        cfg.blank_lines_lower_bound, lower,
        "blank_lines_lower_bound mismatch"
    );
    assert_eq!(
        cfg.blank_lines_upper_bound, upper,
        "blank_lines_upper_bound mismatch"
    );
}

pub fn assert_color(cfg: &RustfmtToml, expected: Option<Color>) {
    assert_eq!(cfg.color, expected, "color mismatch");
}

pub fn assert_emit_mode(cfg: &RustfmtToml, expected: Option<EmitMode>) {
    assert_eq!(cfg.emit_mode, expected, "emit_mode mismatch");
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
