#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use mutants_toml_parser_runtime::{MutantsToml, Sharding, TestTool};

pub fn assert_lists_empty(cfg: &MutantsToml) {
    assert!(cfg.exclude_re.is_empty(), "exclude_re should be empty");
    assert!(cfg.examine_re.is_empty(), "examine_re should be empty");
}

pub fn assert_basic_fields(
    cfg: &MutantsToml,
    timeout_multiplier: Option<f64>,
    minimum_test_timeout: Option<f64>,
) {
    assert_eq!(
        cfg.timeout_multiplier,
        timeout_multiplier,
        "timeout_multiplier mismatch",
    );
    assert_eq!(
        cfg.minimum_test_timeout,
        minimum_test_timeout,
        "minimum_test_timeout mismatch",
    );
}

pub fn assert_string_list(actual: &[String], expected: &[&str], field_name: &str) {
    let expected_values = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(actual, expected_values, "{field_name} mismatch");
}

pub fn assert_bool_field(actual: Option<bool>, expected: Option<bool>, field_name: &str) {
    assert_eq!(actual, expected, "{field_name} mismatch");
}

pub fn assert_test_tool(actual: Option<TestTool>, expected: Option<TestTool>) {
    assert_eq!(actual, expected, "test_tool mismatch");
}

pub fn assert_sharding(actual: Option<Sharding>, expected: Option<Sharding>) {
    assert_eq!(actual, expected, "sharding mismatch");
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid mutants.toml"),
        "expected error message prefix, got: {msg}",
    );
}
