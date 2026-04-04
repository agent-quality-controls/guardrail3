use helpers::{parse_fixture, parse_from_tempfile};
use rustfmt_toml_parser_runtime_assertions::parser as assertions;

use super::helpers;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_core_fields_empty(&cfg);
    assertions::assert_collections_empty(&cfg);
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn realistic_config_parses_typed_fields() {
    let cfg = parse_fixture(
        r#"
max_width = 100
hard_tabs = false
tab_spaces = 4
edition = "2021"
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
merge_derives = true
"#,
    );

    assertions::assert_basic_width_fields(&cfg, Some(100), Some(false), Some(4));
    assertions::assert_string_field(cfg.edition.as_deref(), Some("2021"), "edition");
    assertions::assert_string_field(
        cfg.newline_style.as_deref(),
        Some("Unix"),
        "newline_style",
    );
    assertions::assert_string_field(
        cfg.use_small_heuristics.as_deref(),
        Some("Default"),
        "use_small_heuristics",
    );
    assertions::assert_bool_field(cfg.reorder_imports, Some(true), "reorder_imports");
    assertions::assert_bool_field(cfg.merge_derives, Some(true), "merge_derives");
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse_fixture(
        r#"
max_width = 100
some_future_nightly_option = "yes"
another_unknown = 42
"#,
    );

    assertions::assert_basic_width_fields(&cfg, Some(100), None, None);
    assertions::assert_top_level_string_extra(&cfg, "some_future_nightly_option", "yes");
    assertions::assert_top_level_integer_extra(&cfg, "another_unknown", 42);
}

#[test]
fn flat_entries_roundtrip() {
    let cfg = parse_fixture(
        r#"
max_width = 120
ignore = ["generated.rs", "vendor/"]
skip_macro_invocations = ["bitflags"]
disable_all_formatting = false
"#,
    );

    assertions::assert_basic_width_fields(&cfg, Some(120), None, None);
    assertions::assert_string_list(
        &cfg.ignore,
        &["generated.rs", "vendor/"],
        "ignore",
    );
    assertions::assert_string_list(
        &cfg.skip_macro_invocations,
        &["bitflags"],
        "skip_macro_invocations",
    );
    assertions::assert_bool_field(
        cfg.disable_all_formatting,
        Some(false),
        "disable_all_formatting",
    );

    let serialized = toml::to_string(&cfg).expect("serialization should succeed");
    let cfg2 = parse_fixture(&serialized);
    assertions::assert_tomls_equal(&cfg, &cfg2);
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r#"
max_width = 99
edition = "2021"
"#,
    );

    assertions::assert_basic_width_fields(&cfg, Some(99), None, None);
    assertions::assert_string_field(cfg.edition.as_deref(), Some("2021"), "edition");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
