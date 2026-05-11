use g3ts_toml_parser_runtime_assertions::parser as assertions;
use helpers::{from_path_missing, parse_error, parse_fixture, parse_from_tempfile};
use toml::Value;

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
#[allow(
    clippy::too_many_lines,
    reason = "single parser test proves the TS policy schema maps to typed output"
)]
fn style_and_astro_policy_parse() {
    let cfg = parse_fixture(
        r#"
[style]
source_globs = ["src/**/*.{ts,tsx,astro}"]
stylelint_css_globs = ["src/**/*.css"]
future_style_key = "preserve-style"

[astro]
profile = "strict-static-content"
future_astro_key = "preserve-astro"

[astro.routes]
content = ["src/pages/**/*.astro"]
non_content = ["src/pages/404.astro"]
endpoints = ["src/pages/**/*.ts"]
future_routes_key = "preserve-routes"

[astro.content]
root = "src/content"
adapters = ["src/lib/content", "src/lib/secondary-content"]
required_collections = ["landing", "blog"]
future_content_key = "preserve-content"

[astro.content.collection_fields]
landing = ["title", "description", "sections"]
blog = ["title", "description", "status", "publishedAt"]

[astro.mdx]
component_maps = ["src/components/mdx"]
future_mdx_key = "preserve-mdx"

[astro.seo]
metadata_helpers = ["src/lib/metadata"]
json_ld_helpers = ["src/lib/json-ld"]
strict_ai_readable = true
llms_required_sections = ["Docs", "Policies"]
llms_required_links = ["https://example.com/docs/"]
future_seo_key = "preserve-seo"

[astro.state]
forbidden = [".next/**", ".velite/**", ".contentlayer/**"]
future_state_key = "preserve-state"

[astro.i18n]
locales = ["en", "es"]
default_locale = "en"
require_locale_prefix_for_content_routes = true
allowed_unprefixed_routes = ["/"]
content_route_prefixes = ["/blog"]
checked_internal_link_helpers = ["src/lib/i18n-links"]
approved_internal_link_helpers = ["src/lib/i18n-links"]
approved_localized_link_components = ["LocalizedLink"]
approved_date_format_helpers = ["formatDate"]
approved_number_format_helpers = ["formatNumber"]
public_source_globs = ["src/pages/**/*"]
helper_source_globs = ["src/lib/i18n/**/*"]

[astro.media]
favicon = "/favicon.ico"
app_icons = ["/apple-touch-icon.png"]
default_social_image = "/og/default.png"
allow_svg_icons = false
public_source_globs = ["src/pages/**/*"]
media_helper_modules = ["src/lib/media"]
approved_media_helpers = ["mediaAsset"]
content_image_components = ["ContentImage"]
content_image_key_props = ["imageKey"]
banned_image_source_props = ["src"]
banned_image_alt_props = ["alt"]
allowed_public_image_paths = ["/favicon.ico"]
checked_image_extensions = [".png", ".jpg"]
metadata_image_property_names = ["image"]
"#,
    );

    let style = assertions::assert_style_policy(cfg.style.as_ref());
    assertions::assert_string_list(
        &style.source_globs,
        &["src/**/*.{ts,tsx,astro}"],
        "style.source_globs",
    );
    assertions::assert_string_list(
        &style.stylelint_css_globs,
        &["src/**/*.css"],
        "style.stylelint_css_globs",
    );
    assert_eq!(
        style.extra.get("future_style_key").and_then(Value::as_str),
        Some("preserve-style"),
        "style extra mismatch"
    );

    let astro = assertions::assert_astro_policy(cfg.astro.as_ref());
    assert_eq!(
        astro.profile.as_deref(),
        Some("strict-static-content"),
        "astro profile mismatch"
    );
    assertions::assert_string_list(
        &astro.routes.content,
        &["src/pages/**/*.astro"],
        "astro.routes.content",
    );
    assertions::assert_string_list(
        &astro.content.adapters,
        &["src/lib/content", "src/lib/secondary-content"],
        "astro.content.adapters",
    );
    assertions::assert_string_list(
        astro
            .content
            .collection_fields
            .get("landing")
            .expect("landing collection fields should parse"),
        &["title", "description", "sections"],
        "astro.content.collection_fields.landing",
    );
    assertions::assert_string_list(
        &astro.mdx.component_maps,
        &["src/components/mdx"],
        "astro.mdx.component_maps",
    );
    assertions::assert_string_list(
        &astro.seo.metadata_helpers,
        &["src/lib/metadata"],
        "astro.seo.metadata_helpers",
    );
    assert!(
        astro.seo.strict_ai_readable,
        "astro.seo.strict_ai_readable should parse"
    );
    assertions::assert_string_list(
        &astro.state.forbidden,
        &[".next/**", ".velite/**", ".contentlayer/**"],
        "astro.state.forbidden",
    );
    let i18n = astro.i18n.as_ref().expect("i18n should parse");
    assertions::assert_string_list(&i18n.locales, &["en", "es"], "astro.i18n.locales");
    let media = astro.media.as_ref().expect("media should parse");
    assert_eq!(media.favicon, "/favicon.ico", "media favicon mismatch");
    assertions::assert_string_list(
        &media.app_icons,
        &["/apple-touch-icon.png"],
        "astro.media.app_icons",
    );
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
