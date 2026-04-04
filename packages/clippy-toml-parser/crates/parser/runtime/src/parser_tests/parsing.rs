use clippy_toml_parser_runtime_assertions::parser as assertions;
use helpers::{parse_fixture, parse_from_tempfile};

use super::helpers;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_thresholds_empty(&cfg);
    assertions::assert_ban_lists_empty(&cfg);
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn thresholds_parse() {
    let cfg = parse_fixture(
        r"
max-struct-bools = 3
max-fn-params-bools = 3
too-many-lines-threshold = 75
too-many-arguments-threshold = 7
excessive-nesting-threshold = 4
cognitive-complexity-threshold = 15
type-complexity-threshold = 75
",
    );

    assertions::assert_thresholds(
        &cfg,
        Some(3),
        Some(3),
        Some(75),
        Some(7),
        Some(4),
        Some(15),
        Some(75),
    );
}

#[test]
fn simple_ban_entries() {
    let cfg = parse_fixture(
        r#"
disallowed-methods = ["std::env::var", "std::process::exit"]
disallowed-types = ["std::collections::HashMap"]
disallowed-macros = ["println!", "dbg!"]
"#,
    );

    assertions::assert_ban_entry(cfg.disallowed_methods.first(), "std::env::var", None);
    assertions::assert_list_len(&cfg.disallowed_methods, 2, "disallowed_methods");
    assertions::assert_list_len(&cfg.disallowed_types, 1, "disallowed_types");
    assertions::assert_list_len(&cfg.disallowed_macros, 2, "disallowed_macros");
}

#[test]
fn detailed_ban_entries_with_reason() {
    let cfg = parse_fixture(
        r#"
disallowed-methods = [
    { path = "std::env::var", reason = "Use config module" },
    "std::process::exit",
]
"#,
    );

    assertions::assert_list_len(&cfg.disallowed_methods, 2, "disallowed_methods");
    assertions::assert_ban_entry(
        cfg.disallowed_methods.first(),
        "std::env::var",
        Some("Use config module"),
    );
    assertions::assert_ban_entry(cfg.disallowed_methods.get(1), "std::process::exit", None);
}

#[test]
fn test_relaxations_parse() {
    let cfg = parse_fixture(
        r"
allow-dbg-in-tests = false
allow-print-in-tests = false
allow-expect-in-tests = true
allow-panic-in-tests = false
allow-unwrap-in-tests = false
",
    );

    assertions::assert_bool_field(cfg.allow_dbg_in_tests, Some(false), "allow_dbg_in_tests");
    assertions::assert_bool_field(cfg.allow_print_in_tests, Some(false), "allow_print_in_tests");
    assertions::assert_bool_field(
        cfg.allow_expect_in_tests,
        Some(true),
        "allow_expect_in_tests",
    );
    assertions::assert_bool_field(cfg.allow_panic_in_tests, Some(false), "allow_panic_in_tests");
    assertions::assert_bool_field(
        cfg.allow_unwrap_in_tests,
        Some(false),
        "allow_unwrap_in_tests",
    );
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse_fixture(
        r#"
max-struct-bools = 3
some-future-clippy-option = "yes"
"#,
    );

    assertions::assert_thresholds(&cfg, Some(3), None, None, None, None, None, None);
    assertions::assert_top_level_string_extra(&cfg, "some-future-clippy-option", "yes");
}

#[test]
fn representative_config_parses() {
    let cfg = parse_fixture(
        r#"
too-many-lines-threshold = 75
cognitive-complexity-threshold = 15
too-many-arguments-threshold = 7
type-complexity-threshold = 75
max-struct-bools = 3
max-fn-params-bools = 3
excessive-nesting-threshold = 4
avoid-breaking-exported-api = false
allow-dbg-in-tests = false
allow-expect-in-tests = true
allow-panic-in-tests = false
allow-print-in-tests = false
allow-unwrap-in-tests = false
disallowed-methods = [
    { path = "std::env::var", reason = "Use config module" },
    { path = "std::process::exit", reason = "Use error propagation" },
]
disallowed-types = [
    { path = "std::collections::HashMap", reason = "Use BTreeMap" },
]
disallowed-macros = [
    { path = "println", reason = "Use tracing" },
]
"#,
    );

    assertions::assert_thresholds(&cfg, Some(3), Some(3), Some(75), Some(7), Some(4), Some(15), Some(75));
    assertions::assert_bool_field(
        cfg.avoid_breaking_exported_api,
        Some(false),
        "avoid_breaking_exported_api",
    );
    assertions::assert_list_len(&cfg.disallowed_methods, 2, "disallowed_methods");
    assertions::assert_list_len(&cfg.disallowed_types, 1, "disallowed_types");
    assertions::assert_list_len(&cfg.disallowed_macros, 1, "disallowed_macros");
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile("max-struct-bools = 4\n");
    assertions::assert_thresholds(&cfg, Some(4), None, None, None, None, None, None);
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
