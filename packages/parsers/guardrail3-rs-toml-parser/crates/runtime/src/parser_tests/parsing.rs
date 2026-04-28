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
profile = "strict-static-content"
future_astro_key = "preserve-astro"

[ts.astro.routes]
content = ["src/pages/**/*.astro"]
non_content = ["src/pages/404.astro"]
endpoints = ["src/pages/**/*.ts"]
future_routes_key = "preserve-routes"

[ts.astro.content]
root = "src/content"
adapters = ["src/lib/content", "src/lib/secondary-content"]
required_collections = ["landing", "blog"]
future_content_key = "preserve-content"

[ts.astro.content.collection_fields]
landing = ["title", "description", "sections"]
blog = ["title", "description", "status", "publishedAt"]

[ts.astro.mdx]
component_maps = ["src/components/mdx"]
future_mdx_key = "preserve-mdx"

[ts.astro.seo]
metadata_helpers = ["src/lib/metadata"]
json_ld_helpers = ["src/lib/json-ld"]
strict_ai_readable = true
llms_required_sections = ["Docs", "Policies"]
llms_required_links = ["https://example.com/docs/"]
future_seo_key = "preserve-seo"

[ts.astro.state]
forbidden = [".next/**", ".velite/**", ".contentlayer/**"]
future_state_key = "preserve-state"

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
    assertions::assert_ts_astro_profile(astro, Some("strict-static-content"));
    assertions::assert_string_list(
        &astro.routes.content,
        &["src/pages/**/*.astro"],
        "ts.astro.routes.content",
    );
    assertions::assert_string_list(
        &astro.routes.non_content,
        &["src/pages/404.astro"],
        "ts.astro.routes.non_content",
    );
    assertions::assert_string_list(
        &astro.routes.endpoints,
        &["src/pages/**/*.ts"],
        "ts.astro.routes.endpoints",
    );
    assert_eq!(
        astro.content.root.as_deref(),
        Some("src/content"),
        "ts.astro.content.root mismatch"
    );
    assertions::assert_string_list(
        &astro.content.adapters,
        &["src/lib/content", "src/lib/secondary-content"],
        "ts.astro.content.adapters",
    );
    assertions::assert_string_list(
        &astro.content.required_collections,
        &["landing", "blog"],
        "ts.astro.content.required_collections",
    );
    assertions::assert_string_list(
        astro
            .content
            .collection_fields
            .get("landing")
            .expect("landing collection fields should parse"),
        &["title", "description", "sections"],
        "ts.astro.content.collection_fields.landing",
    );
    assertions::assert_string_list(
        astro
            .content
            .collection_fields
            .get("blog")
            .expect("blog collection fields should parse"),
        &["title", "description", "status", "publishedAt"],
        "ts.astro.content.collection_fields.blog",
    );
    assertions::assert_ts_astro_content_extra_string(
        astro,
        "future_content_key",
        "preserve-content",
    );
    assertions::assert_ts_astro_routes_extra_string(astro, "future_routes_key", "preserve-routes");
    assert_eq!(
        astro.content.adapters.first().map(String::as_str),
        Some("src/lib/content"),
        "ts.astro.content.adapters first value mismatch"
    );
    assertions::assert_string_list(
        &astro.mdx.component_maps,
        &["src/components/mdx"],
        "ts.astro.mdx.component_maps",
    );
    assertions::assert_ts_astro_mdx_extra_string(astro, "future_mdx_key", "preserve-mdx");
    assertions::assert_string_list(
        &astro.seo.metadata_helpers,
        &["src/lib/metadata"],
        "ts.astro.seo.metadata_helpers",
    );
    assertions::assert_string_list(
        &astro.seo.json_ld_helpers,
        &["src/lib/json-ld"],
        "ts.astro.seo.json_ld_helpers",
    );
    assertions::assert_ts_astro_seo_strict_ai_readable(astro, true);
    assertions::assert_ts_astro_seo_llms_required_sections(astro, &["Docs", "Policies"]);
    assertions::assert_ts_astro_seo_llms_required_links(astro, &["https://example.com/docs/"]);
    assertions::assert_ts_astro_seo_extra_string(astro, "future_seo_key", "preserve-seo");
    assertions::assert_string_list(
        &astro.state.forbidden,
        &[".next/**", ".velite/**", ".contentlayer/**"],
        "ts.astro.state.forbidden",
    );
    assertions::assert_ts_astro_state_extra_string(astro, "future_state_key", "preserve-state");
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
profile = "strict-static-content"
"#,
    );

    let astro = assertions::assert_ts_astro_policy(cfg.ts.as_ref());
    assertions::assert_ts_astro_profile(astro, Some("strict-static-content"));
    assertions::assert_string_list(&astro.routes.content, &[], "ts.astro.routes.content");
    assertions::assert_string_list(
        &astro.routes.non_content,
        &[],
        "ts.astro.routes.non_content",
    );
    assertions::assert_string_list(&astro.routes.endpoints, &[], "ts.astro.routes.endpoints");
    assert_eq!(
        astro.content.root, None,
        "ts.astro.content.root should be None"
    );
    assertions::assert_string_list(&astro.content.adapters, &[], "ts.astro.content.adapters");
    assertions::assert_string_list(
        &astro.content.required_collections,
        &[],
        "ts.astro.content.required_collections",
    );
    assert!(
        astro.content.collection_fields.is_empty(),
        "ts.astro.content.collection_fields should be empty"
    );
    assertions::assert_string_list(
        &astro.mdx.component_maps,
        &[],
        "ts.astro.mdx.component_maps",
    );
    assertions::assert_string_list(
        &astro.seo.metadata_helpers,
        &[],
        "ts.astro.seo.metadata_helpers",
    );
    assertions::assert_string_list(
        &astro.seo.json_ld_helpers,
        &[],
        "ts.astro.seo.json_ld_helpers",
    );
    assertions::assert_ts_astro_seo_strict_ai_readable(astro, false);
    assertions::assert_ts_astro_seo_llms_required_sections(astro, &[]);
    assertions::assert_ts_astro_seo_llms_required_links(astro, &[]);
    assertions::assert_string_list(&astro.state.forbidden, &[], "ts.astro.state.forbidden");
}

#[test]
fn old_flat_ts_astro_policy_fields_are_ignored_extras() {
    let cfg = parse_fixture(
        r#"
[ts.astro]
profile = "strict-static-content"
content_routes = ["src/pages/**/*.astro"]
non_content_routes = ["src/pages/404.astro"]
endpoints = ["src/pages/**/*.ts"]
content_root = "src/content"
content_adapter = "src/lib/content"
mdx_component_maps = ["src/components/mdx"]
metadata_helpers = ["src/lib/metadata"]
json_ld_helpers = ["src/lib/json-ld"]
forbidden_state = [".next/**", ".velite/**", ".contentlayer/**"]
"#,
    );

    let astro = assertions::assert_ts_astro_policy(cfg.ts.as_ref());
    assertions::assert_ts_astro_profile(astro, Some("strict-static-content"));
    assertions::assert_string_list(&astro.routes.content, &[], "ts.astro.routes.content");
    assertions::assert_string_list(
        &astro.routes.non_content,
        &[],
        "ts.astro.routes.non_content",
    );
    assertions::assert_string_list(&astro.routes.endpoints, &[], "ts.astro.routes.endpoints");
    assert_eq!(
        astro.content.root, None,
        "old flat content_root must not populate ts.astro.content.root"
    );
    assertions::assert_string_list(&astro.content.adapters, &[], "ts.astro.content.adapters");
    assertions::assert_string_list(
        &astro.mdx.component_maps,
        &[],
        "ts.astro.mdx.component_maps",
    );
    assertions::assert_string_list(
        &astro.seo.metadata_helpers,
        &[],
        "ts.astro.seo.metadata_helpers",
    );
    assertions::assert_string_list(
        &astro.seo.json_ld_helpers,
        &[],
        "ts.astro.seo.json_ld_helpers",
    );
    assertions::assert_string_list(&astro.state.forbidden, &[], "ts.astro.state.forbidden");
    assert!(
        astro.extra.contains_key("content_routes")
            && astro.extra.contains_key("content_adapter")
            && astro.extra.contains_key("forbidden_state"),
        "old flat ts.astro fields should be preserved only as extras"
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
[ts.astro.routes]
content = "src/pages/**/*.astro"
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
