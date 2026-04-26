use guardrail3_rs_toml_parser_runtime_assertions::parser as assertions;
use helpers::{parse_error, parse_fixture, parse_fixture_file, parse_from_tempfile};

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

    assertions::assert_profile(&cfg, Some(crate::types::RustProfile::Service));
    assertions::assert_string_list(&cfg.allowed_deps, &["serde", "toml"], "allowed_deps");
    assertions::assert_extra_empty(&cfg);
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "single parser test proves all top-level parser fields map to typed output"
)]
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

[ts]
future_ts_key = "preserve-ts"

[ts.astro]
profile = "strict-local-content"
content_routes = ["src/pages/**/*.astro"]
non_content_routes = ["src/pages/404.astro"]
endpoints = ["src/pages/**/*.ts"]
content_root = "src/content"
content_adapter = "src/lib/content"
forbidden_state = [".next/**", ".velite/**", ".contentlayer/**"]
future_astro_key = "preserve-astro"

[[waivers]]
rule = "RS-CARGO-12"
file = "Cargo.toml"
selector = "clippy:redundant_pub_crate"
reason = "temporary allow during migration"
reviewer = "guardrail-team"
"#,
    );

    assertions::assert_version(&cfg, Some("1"));
    assertions::assert_profile(&cfg, Some(crate::types::RustProfile::Library));
    assertions::assert_string_list(
        &cfg.excluded_paths,
        &["legacy", "tests/fixtures"],
        "excluded_paths",
    );
    assertions::assert_string_list(&cfg.allowed_deps, &["serde", "toml"], "allowed_deps");
    assertions::assert_fmt_check(cfg.checks.as_ref(), Some(true));
    assertions::assert_clippy_check(cfg.checks.as_ref(), Some(true));
    assertions::assert_check_extra_bool(cfg.checks.as_ref(), "hexarch", false);
    assertions::assert_check_extra_string(cfg.checks.as_ref(), "future_check", "preserve-me");
    assertions::assert_ts_extra_string(cfg.ts.as_ref(), "future_ts_key", "preserve-ts");
    let astro = assertions::assert_ts_astro_policy(cfg.ts.as_ref());
    assertions::assert_ts_astro_profile(astro, Some("strict-local-content"));
    assertions::assert_string_list(
        &astro.content_routes,
        &["src/pages/**/*.astro"],
        "ts.astro.content_routes",
    );
    assertions::assert_string_list(
        &astro.non_content_routes,
        &["src/pages/404.astro"],
        "ts.astro.non_content_routes",
    );
    assertions::assert_string_list(
        &astro.endpoints,
        &["src/pages/**/*.ts"],
        "ts.astro.endpoints",
    );
    assert_eq!(
        astro.content_root.as_deref(),
        Some("src/content"),
        "ts.astro.content_root mismatch"
    );
    assert_eq!(
        astro.content_adapter.as_deref(),
        Some("src/lib/content"),
        "ts.astro.content_adapter mismatch"
    );
    assertions::assert_string_list(
        &astro.forbidden_state,
        &[".next/**", ".velite/**", ".contentlayer/**"],
        "ts.astro.forbidden_state",
    );
    assertions::assert_ts_astro_extra_string(astro, "future_astro_key", "preserve-astro");
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
fn absent_ts_astro_policy_stays_absent() {
    let cfg = parse_fixture(
        r"
[checks]
fmt = true
",
    );

    assert_eq!(cfg.ts, None, "ts policy should be absent");
}

#[test]
fn present_ts_without_astro_policy_stays_absent() {
    let cfg = parse_fixture(
        r#"
[ts]
future_ts_key = "preserve-ts"
"#,
    );

    let ts = cfg.ts.as_ref().expect("ts policy should be present");
    assert_eq!(ts.astro, None, "ts.astro policy should be absent");
    assertions::assert_ts_extra_string(cfg.ts.as_ref(), "future_ts_key", "preserve-ts");
}

#[test]
fn empty_ts_astro_policy_defaults_lists_to_empty() {
    let cfg = parse_fixture(
        r#"
[ts.astro]
profile = "strict-local-content"
"#,
    );

    let astro = assertions::assert_ts_astro_policy(cfg.ts.as_ref());
    assertions::assert_ts_astro_profile(astro, Some("strict-local-content"));
    assertions::assert_string_list(&astro.content_routes, &[], "ts.astro.content_routes");
    assertions::assert_string_list(
        &astro.non_content_routes,
        &[],
        "ts.astro.non_content_routes",
    );
    assertions::assert_string_list(&astro.endpoints, &[], "ts.astro.endpoints");
    assert_eq!(
        astro.content_root, None,
        "ts.astro.content_root should be None"
    );
    assert_eq!(
        astro.content_adapter, None,
        "ts.astro.content_adapter should be None"
    );
    assertions::assert_string_list(&astro.forbidden_state, &[], "ts.astro.forbidden_state");
}

#[test]
fn removed_hexarch_key_is_treated_as_extra() {
    let cfg = parse_fixture(
        r"
[checks]
hexarch = true
",
    );

    assertions::assert_check_extra_bool(cfg.checks.as_ref(), "hexarch", true);
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

    assertions::assert_profile(&cfg, Some(crate::types::RustProfile::Service));
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

    assertions::assert_profile(&cfg, Some(crate::types::RustProfile::Service));
}

#[test]
fn realistic_workspace_fixture_file_parses() {
    let cfg = parse_fixture_file("workspace_service.toml");

    assertions::assert_version(&cfg, Some("1"));
    assertions::assert_profile(&cfg, Some(crate::types::RustProfile::Service));
    assertions::assert_string_list(
        &cfg.excluded_paths,
        &["legacy", "tests/fixtures"],
        "excluded_paths",
    );
    assertions::assert_string_list(&cfg.allowed_deps, &["serde", "toml"], "allowed_deps");
    assertions::assert_fmt_check(cfg.checks.as_ref(), Some(true));
    assertions::assert_hooks_rs_check(cfg.checks.as_ref(), Some(true));
    assert_eq!(cfg.waivers.len(), 3, "fixture should contain three waivers");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = parse_error("this is not [[[valid toml");
    assertions::assert_parse_error(err);
}

#[test]
fn parse_error_on_wrong_ts_astro_field_type() {
    let err = parse_error(
        r#"
[ts.astro]
content_routes = "src/pages/**/*.astro"
"#,
    );

    assertions::assert_parse_error(err);
}

#[test]
fn parse_error_on_wrong_waiver_field_type() {
    let err = parse_error(
        r#"
[[waivers]]
rule = 5
file = "Cargo.toml"
selector = "x"
reason = "temporary"
"#,
    );

    assertions::assert_parse_error(err);
}
