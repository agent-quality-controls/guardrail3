use guardrail3_rs_toml_parser_runtime_assertions::parser as assertions;
use helpers::{parse_fixture, parse_fixture_file, parse_from_tempfile};

use super::helpers;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_core_fields_empty(&cfg);
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn root_profile_and_allowlist_parse() {
    let cfg = parse_fixture(
        r#"
profile = "service"
allowed_deps = ["serde", "toml"]
"#,
    );

    assertions::assert_profile(&cfg, Some(crate::RustProfile::Service));
    assertions::assert_string_list(&cfg.allowed_deps, &["serde", "toml"], "allowed_deps");
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn realistic_config_parses_known_fields() {
    let cfg = parse_fixture(
        r#"
version = "1"
profile = "library"
excluded_paths = ["legacy", "tests/fixtures"]
allowed_deps = ["serde", "toml"]
future-key = "keep-me"

[checks]
fmt = true
clippy = true
hexarch = false
future_check = "preserve-me"

[[waivers]]
rule = "RS-CARGO-12"
file = "Cargo.toml"
selector = "clippy:redundant_pub_crate"
reason = "temporary allow during migration"
reviewer = "guardrail-team"
"#,
    );

    assertions::assert_version(&cfg, Some("1"));
    assertions::assert_profile(&cfg, Some(crate::RustProfile::Library));
    assertions::assert_string_list(
        &cfg.excluded_paths,
        &["legacy", "tests/fixtures"],
        "excluded_paths",
    );
    assertions::assert_string_list(&cfg.allowed_deps, &["serde", "toml"], "allowed_deps");
    assertions::assert_check_value(cfg.checks.as_ref(), "fmt", Some(true));
    assertions::assert_check_value(cfg.checks.as_ref(), "clippy", Some(true));
    assertions::assert_check_value(cfg.checks.as_ref(), "hexarch", Some(false));
    assertions::assert_check_extra_string(cfg.checks.as_ref(), "future_check", "preserve-me");
    assertions::assert_waiver(
        cfg.waivers.first(),
        "RS-CARGO-12",
        "Cargo.toml",
        "clippy:redundant_pub_crate",
        "temporary allow during migration",
    );
    assertions::assert_waiver_extra_string(cfg.waivers.first(), "reviewer", "guardrail-team");
    assertions::assert_top_level_string_extra(&cfg, "future-key", "keep-me");
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse_fixture(
        r#"
profile = "service"
future-key = "value"

[checks]
fmt = true
nested = { still = "here" }
"#,
    );

    assertions::assert_profile(&cfg, Some(crate::RustProfile::Service));
    assertions::assert_top_level_string_extra(&cfg, "future-key", "value");
    assertions::assert_check_extra_table(cfg.checks.as_ref(), "nested");
}

#[test]
fn config_roundtrips() {
    let cfg = parse_fixture(
        r#"
profile = "service"
allowed_deps = ["serde"]

[checks]
garde = true

[[waivers]]
rule = "RS-GARDE-AST-04"
file = "src/adapters/db.rs"
selector = "sqlx::query_as@L42"
reason = "legacy row type"
"#,
    );

    let serialized = toml::to_string(&cfg).expect("serialization should succeed");
    let cfg2 = parse_fixture(&serialized);
    assertions::assert_tomls_equal(&cfg, &cfg2);
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r#"
profile = "service"
"#,
    );

    assertions::assert_profile(&cfg, Some(crate::RustProfile::Service));
}

#[test]
fn realistic_workspace_fixture_file_parses() {
    let cfg = parse_fixture_file("workspace_service.toml");

    assertions::assert_version(&cfg, Some("1"));
    assertions::assert_profile(&cfg, Some(crate::RustProfile::Service));
    assertions::assert_string_list(
        &cfg.excluded_paths,
        &["legacy", "tests/fixtures"],
        "excluded_paths",
    );
    assertions::assert_string_list(&cfg.allowed_deps, &["serde", "toml"], "allowed_deps");
    assertions::assert_check_value(cfg.checks.as_ref(), "fmt", Some(true));
    assertions::assert_check_value(cfg.checks.as_ref(), "hooks_rs", Some(true));
    assert_eq!(cfg.waivers.len(), 3, "fixture should contain three waivers");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should error"));
}
