use super::helpers::{parse_fixture, parse_from_tempfile};
use mutants_toml_parser_runtime_assertions::parser as assertions;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_lists_empty(&cfg);
    assertions::assert_basic_fields(&cfg, None, None, None);
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn realistic_config_parses_typed_fields() {
    let cfg = parse_fixture(
        r#"
timeout_multiplier = 3.0
minimum_test_timeout = "20s"
test_tool = "nextest"
exclude_re = ["^tests/", "^benches/"]
examine_globs = ["src/**/*.rs"]
"#,
    );

    assertions::assert_basic_fields(&cfg, Some(3.0), Some("20s"), Some("nextest"));
    assertions::assert_string_list(&cfg.exclude_re, &["^tests/", "^benches/"], "exclude_re");
    assertions::assert_string_list(&cfg.examine_globs, &["src/**/*.rs"], "examine_globs");
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse_fixture(
        r"
timeout_multiplier = 2.0
some_future_option = true
",
    );

    assertions::assert_basic_fields(&cfg, Some(2.0), None, None);
    assertions::assert_top_level_bool_extra(&cfg, "some_future_option", true);
}

#[test]
fn feature_control_fields() {
    let cfg = parse_fixture(
        r#"
all_features = true
no_default_features = false
features = ["serde", "derive"]
"#,
    );

    assertions::assert_bool_field(cfg.all_features, Some(true), "all_features");
    assertions::assert_bool_field(
        cfg.no_default_features,
        Some(false),
        "no_default_features",
    );
    assertions::assert_string_list(&cfg.features, &["serde", "derive"], "features");
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r#"
test_tool = "cargo"
timeout_multiplier = 1.5
"#,
    );

    assertions::assert_basic_fields(&cfg, Some(1.5), None, Some("cargo"));
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
