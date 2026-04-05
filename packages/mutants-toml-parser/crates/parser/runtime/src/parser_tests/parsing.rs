use super::helpers::{parse_fixture, parse_from_tempfile};
use crate::{Sharding, TestTool};
use mutants_toml_parser_runtime_assertions::parser as assertions;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_lists_empty(&cfg);
    assertions::assert_basic_fields(&cfg, None, None);
    assertions::assert_test_tool(cfg.test_tool, None);
    assertions::assert_sharding(cfg.sharding, None);
}

#[test]
fn real_config_fields_parse() {
    let cfg = parse_fixture(
        r#"
timeout_multiplier = 3.0
minimum_test_timeout = 20.0
build_timeout_multiplier = 2.0
copy_target = true
gitignore = false
test_tool = "nextest"
sharding = "round-robin"
exclude_re = ["^tests/", "^benches/"]
examine_globs = ["src/**/*.rs"]
test_package = ["app", "lib"]
"#,
    );

    assertions::assert_basic_fields(&cfg, Some(3.0), Some(20.0));
    assertions::assert_bool_field(cfg.copy_target, Some(true), "copy_target");
    assertions::assert_bool_field(cfg.gitignore, Some(false), "gitignore");
    assertions::assert_test_tool(cfg.test_tool, Some(TestTool::Nextest));
    assertions::assert_sharding(cfg.sharding, Some(Sharding::RoundRobin));
    assertions::assert_string_list(&cfg.exclude_re, &["^tests/", "^benches/"], "exclude_re");
    assertions::assert_string_list(&cfg.examine_globs, &["src/**/*.rs"], "examine_globs");
    assertions::assert_string_list(&cfg.test_package, &["app", "lib"], "test_package");
}

#[test]
fn feature_control_fields() {
    let cfg = parse_fixture(
        r#"
all_features = true
no_default_features = false
features = ["serde", "derive"]
test_workspace = true
"#,
    );

    assertions::assert_bool_field(cfg.all_features, Some(true), "all_features");
    assertions::assert_bool_field(
        cfg.no_default_features,
        Some(false),
        "no_default_features",
    );
    assertions::assert_string_list(&cfg.features, &["serde", "derive"], "features");
    assertions::assert_bool_field(cfg.test_workspace, Some(true), "test_workspace");
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r#"
timeout_multiplier = 1.5
test_tool = "cargo"
"#,
    );

    assertions::assert_basic_fields(&cfg, Some(1.5), None);
    assertions::assert_test_tool(cfg.test_tool, Some(TestTool::Cargo));
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}

#[test]
fn unknown_keys_are_rejected() {
    let err = super::super::parse(
        r"
timeout_multiplier = 2.0
some_future_option = true
",
    )
    .expect_err("unknown keys should be rejected");

    assertions::assert_parse_error(err);
}

#[test]
fn unsupported_emit_diffs_is_rejected() {
    let err = super::super::parse(
        r"
emit_diffs = true
",
    )
    .expect_err("emit_diffs is intentionally skipped by upstream config");

    assertions::assert_parse_error(err);
}
