use guardrail3_ts_toml_parser_runtime_assertions::parser as assertions;
use helpers::{from_path_missing, parse_error, parse_fixture, parse_from_tempfile};

use super::helpers;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_core_fields_empty(&cfg);
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn checks_table_with_eslint_false_parses() {
    let cfg = parse_fixture(
        r"
[checks]
eslint = false
",
    );

    assertions::assert_eslint_check(cfg.checks.as_ref(), Some(false));
}

#[test]
fn checks_table_with_multiple_disabled_families_parses() {
    let cfg = parse_fixture(
        r"
[checks]
eslint = false
style = false
",
    );

    assertions::assert_eslint_check(cfg.checks.as_ref(), Some(false));
    assertions::assert_style_check(cfg.checks.as_ref(), Some(false));
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r"
[checks]
eslint = false
",
    );

    assertions::assert_eslint_check(cfg.checks.as_ref(), Some(false));
}

#[test]
fn from_path_missing_returns_io_error() {
    let err = from_path_missing();
    let msg = err.to_string();
    assert!(
        msg.contains("could not read guardrail3-ts.toml"),
        "expected io error prefix, got: {msg}",
    );
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = parse_error("this is not [[[valid toml");
    assertions::assert_parse_error(err);
}

#[test]
fn unknown_check_keys_land_in_extra() {
    let cfg = parse_fixture(
        r#"
[checks]
eslint = false
future_check = "preserve-me"
"#,
    );

    assertions::assert_eslint_check(cfg.checks.as_ref(), Some(false));
    assertions::assert_check_extra_string(cfg.checks.as_ref(), "future_check", "preserve-me");
}
