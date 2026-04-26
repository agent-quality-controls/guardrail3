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
    reason = "single parser test proves all ts.astro fields map to typed output"
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
authored_content_globs = ["content/**/*.mdx"]
content_route_globs = ["src/pages/**/*.astro"]
chrome_route_globs = ["src/pages/_chrome/**/*.astro"]
utility_route_globs = ["src/pages/health.astro"]
generated_route_globs = ["src/pages/generated/**/*.astro"]
report_shell_route_globs = ["src/pages/reports/**/*.astro"]
endpoint_globs = ["src/pages/**/*.ts"]
content_data_module_globs = ["src/**/*.data.ts"]
query_adapter_globs = ["src/content/query/**/*.ts"]
adapter_barrel_globs = ["src/content/index.ts"]
adapter_helper_globs = ["src/content/helpers/**/*.ts"]
route_registry_globs = ["src/content/routes.ts"]
content_component_globs = ["src/content/components/**/*.tsx"]
content_config_globs = ["src/content.config.ts"]
mdx_content_globs = ["src/content/**/*.mdx"]
approved_mdx_component_globs = ["src/content/components/**/*.tsx"]
approved_generated_artifact_globs = ["src/generated/content/**/*.json"]
astro_content_type_import_globs = ["src/content/types.ts"]
contentlayer_config_globs = ["contentlayer.config.*"]
contentlayer_generated_globs = [".contentlayer/**"]
forbidden_generated_state_globs = [".next/**", ".velite/**", ".contentlayer/**"]
build_output_globs = ["dist/**"]
blog_index_route_globs = ["src/pages/blog.astro"]
blog_article_route_globs = ["src/pages/blog/[slug].astro"]
metadata_helper_globs = ["src/content/metadata.ts"]
json_ld_helper_globs = ["src/content/json-ld.ts"]
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
        &astro.authored_content_globs,
        &["content/**/*.mdx"],
        "ts.astro.authored_content_globs",
    );
    assertions::assert_string_list(
        &astro.content_route_globs,
        &["src/pages/**/*.astro"],
        "ts.astro.content_route_globs",
    );
    assertions::assert_string_list(
        &astro.chrome_route_globs,
        &["src/pages/_chrome/**/*.astro"],
        "ts.astro.chrome_route_globs",
    );
    assertions::assert_string_list(
        &astro.utility_route_globs,
        &["src/pages/health.astro"],
        "ts.astro.utility_route_globs",
    );
    assertions::assert_string_list(
        &astro.generated_route_globs,
        &["src/pages/generated/**/*.astro"],
        "ts.astro.generated_route_globs",
    );
    assertions::assert_string_list(
        &astro.report_shell_route_globs,
        &["src/pages/reports/**/*.astro"],
        "ts.astro.report_shell_route_globs",
    );
    assertions::assert_string_list(
        &astro.endpoint_globs,
        &["src/pages/**/*.ts"],
        "ts.astro.endpoint_globs",
    );
    assertions::assert_string_list(
        &astro.content_data_module_globs,
        &["src/**/*.data.ts"],
        "ts.astro.content_data_module_globs",
    );
    assertions::assert_string_list(
        &astro.query_adapter_globs,
        &["src/content/query/**/*.ts"],
        "ts.astro.query_adapter_globs",
    );
    assertions::assert_string_list(
        &astro.adapter_barrel_globs,
        &["src/content/index.ts"],
        "ts.astro.adapter_barrel_globs",
    );
    assertions::assert_string_list(
        &astro.adapter_helper_globs,
        &["src/content/helpers/**/*.ts"],
        "ts.astro.adapter_helper_globs",
    );
    assertions::assert_string_list(
        &astro.route_registry_globs,
        &["src/content/routes.ts"],
        "ts.astro.route_registry_globs",
    );
    assertions::assert_string_list(
        &astro.content_component_globs,
        &["src/content/components/**/*.tsx"],
        "ts.astro.content_component_globs",
    );
    assertions::assert_string_list(
        &astro.content_config_globs,
        &["src/content.config.ts"],
        "ts.astro.content_config_globs",
    );
    assertions::assert_string_list(
        &astro.mdx_content_globs,
        &["src/content/**/*.mdx"],
        "ts.astro.mdx_content_globs",
    );
    assertions::assert_string_list(
        &astro.approved_mdx_component_globs,
        &["src/content/components/**/*.tsx"],
        "ts.astro.approved_mdx_component_globs",
    );
    assertions::assert_string_list(
        &astro.approved_generated_artifact_globs,
        &["src/generated/content/**/*.json"],
        "ts.astro.approved_generated_artifact_globs",
    );
    assertions::assert_string_list(
        &astro.astro_content_type_import_globs,
        &["src/content/types.ts"],
        "ts.astro.astro_content_type_import_globs",
    );
    assertions::assert_string_list(
        &astro.contentlayer_config_globs,
        &["contentlayer.config.*"],
        "ts.astro.contentlayer_config_globs",
    );
    assertions::assert_string_list(
        &astro.contentlayer_generated_globs,
        &[".contentlayer/**"],
        "ts.astro.contentlayer_generated_globs",
    );
    assertions::assert_string_list(
        &astro.forbidden_generated_state_globs,
        &[".next/**", ".velite/**", ".contentlayer/**"],
        "ts.astro.forbidden_generated_state_globs",
    );
    assertions::assert_string_list(
        &astro.build_output_globs,
        &["dist/**"],
        "ts.astro.build_output_globs",
    );
    assertions::assert_string_list(
        &astro.blog_index_route_globs,
        &["src/pages/blog.astro"],
        "ts.astro.blog_index_route_globs",
    );
    assertions::assert_string_list(
        &astro.blog_article_route_globs,
        &["src/pages/blog/[slug].astro"],
        "ts.astro.blog_article_route_globs",
    );
    assertions::assert_string_list(
        &astro.metadata_helper_globs,
        &["src/content/metadata.ts"],
        "ts.astro.metadata_helper_globs",
    );
    assertions::assert_string_list(
        &astro.json_ld_helper_globs,
        &["src/content/json-ld.ts"],
        "ts.astro.json_ld_helper_globs",
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
    assertions::assert_string_list(
        &astro.authored_content_globs,
        &[],
        "ts.astro.authored_content_globs",
    );
    assertions::assert_string_list(
        &astro.content_route_globs,
        &[],
        "ts.astro.content_route_globs",
    );
    assertions::assert_string_list(
        &astro.content_data_module_globs,
        &[],
        "ts.astro.content_data_module_globs",
    );
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
content_route_globs = "src/pages/**/*.astro"
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
