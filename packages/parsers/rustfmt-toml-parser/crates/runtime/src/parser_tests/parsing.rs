use rustfmt_toml_parser_runtime_assertions::parser as assertions;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = assertions::parse_fixture("");

    assertions::assert_core_fields_empty(&cfg);
    assertions::assert_collections_empty(&cfg);
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn realistic_config_parses_typed_fields() {
    let cfg = assertions::parse_fixture(
        r#"
max_width = 100
hard_tabs = false
tab_spaces = 4
edition = "2021"
newline_style = "Unix"
use_small_heuristics = "Default"
blank_lines_lower_bound = 1
blank_lines_upper_bound = 2
color = "Always"
overflow_delimited_expr = true
emit_mode = "Stdout"
make_backup = false
print_misformatted_file_names = true
reorder_imports = true
merge_derives = true
"#,
    );

    assertions::assert_basic_width_fields(&cfg, Some(100), Some(false), Some(4));
    assertions::assert_edition(&cfg, Some(assertions::Edition::Edition2021));
    assertions::assert_newline_style(&cfg, Some(assertions::NewlineStyle::Unix));
    assertions::assert_use_small_heuristics(&cfg, Some(assertions::Heuristics::Default));
    assertions::assert_blank_line_bounds(&cfg, Some(1), Some(2));
    assertions::assert_color(&cfg, Some(assertions::Color::Always));
    assertions::assert_bool_field(
        cfg.overflow_delimited_expr,
        Some(true),
        "overflow_delimited_expr",
    );
    assertions::assert_emit_mode(&cfg, Some(assertions::EmitMode::Stdout));
    assertions::assert_bool_field(cfg.make_backup, Some(false), "make_backup");
    assertions::assert_bool_field(
        cfg.print_misformatted_file_names,
        Some(true),
        "print_misformatted_file_names",
    );
    assertions::assert_bool_field(cfg.reorder_imports, Some(true), "reorder_imports");
    assertions::assert_bool_field(cfg.merge_derives, Some(true), "merge_derives");
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = assertions::parse_fixture(
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
    let cfg = assertions::parse_fixture(
        r#"
max_width = 120
ignore = ["generated.rs", "vendor/"]
skip_macro_invocations = ["bitflags"]
disable_all_formatting = false
"#,
    );

    assertions::assert_basic_width_fields(&cfg, Some(120), None, None);
    assertions::assert_string_list(&cfg.ignore, &["generated.rs", "vendor/"], "ignore");
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
    let cfg2 = assertions::parse_fixture(&serialized);
    assertions::assert_tomls_equal(&cfg, &cfg2);
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = assertions::parse_from_tempfile(
        r#"
max_width = 99
edition = "2021"
"#,
    );

    assertions::assert_basic_width_fields(&cfg, Some(99), None, None);
    assertions::assert_edition(&cfg, Some(assertions::Edition::Edition2021));
}

#[test]
fn invalid_enum_value_is_rejected() {
    let err = assertions::parse_error(
        r#"
newline_style = "Posix"
"#,
    );

    assertions::assert_parse_error(err.expect_err("invalid enum value should produce an error"));
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = assertions::parse_error("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
