#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::panic,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use guardrail3_rs_toml_parser_runtime::types::{
    Guardrail3RsToml, RustChecksConfig, RustProfile, TsAstroPolicyConfig, TsPolicyConfig, Value,
    WaiverConfig,
};

pub fn assert_core_fields_empty(cfg: &Guardrail3RsToml) {
    assert_eq!(cfg.version, None, "version should be None for empty input");
    assert_eq!(cfg.profile, None, "profile should be None for empty input");
    assert!(
        cfg.excluded_paths.is_empty(),
        "excluded_paths should be empty"
    );
    assert!(cfg.allowed_deps.is_empty(), "allowed_deps should be empty");
    assert_eq!(cfg.checks, None, "checks should be None for empty input");
    assert_eq!(cfg.ts, None, "ts should be None for empty input");
    assert!(cfg.waivers.is_empty(), "waivers should be empty");
}

pub fn assert_extra_empty(cfg: &Guardrail3RsToml) {
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

pub fn assert_version(cfg: &Guardrail3RsToml, expected: Option<&str>) {
    assert_eq!(cfg.version.as_deref(), expected, "version mismatch");
}

pub fn assert_profile(cfg: &Guardrail3RsToml, expected: Option<RustProfile>) {
    assert_eq!(cfg.profile, expected, "profile mismatch");
}

pub fn assert_string_list(actual: &[String], expected: &[&str], field_name: &str) {
    let expected_values = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(actual, expected_values, "{field_name} mismatch");
}

pub fn assert_fmt_check(checks: Option<&RustChecksConfig>, expected: Option<bool>) {
    let actual = checks.and_then(|checks| checks.fmt);
    assert_eq!(actual, expected, "fmt mismatch");
}

pub fn assert_clippy_check(checks: Option<&RustChecksConfig>, expected: Option<bool>) {
    let actual = checks.and_then(|checks| checks.clippy);
    assert_eq!(actual, expected, "clippy mismatch");
}

pub fn assert_garde_check(checks: Option<&RustChecksConfig>, expected: Option<bool>) {
    let actual = checks.and_then(|checks| checks.garde);
    assert_eq!(actual, expected, "garde mismatch");
}

pub fn assert_hooks_rs_check(checks: Option<&RustChecksConfig>, expected: Option<bool>) {
    let actual = checks.and_then(|checks| checks.hooks_rs);
    assert_eq!(actual, expected, "hooks_rs mismatch");
}

pub fn assert_check_extra_string(checks: Option<&RustChecksConfig>, key: &str, expected: &str) {
    assert_eq!(
        checks
            .and_then(|checks| checks.extra.get(key))
            .and_then(Value::as_str),
        Some(expected),
        "check extra value mismatch",
    );
}

pub fn assert_check_extra_bool(checks: Option<&RustChecksConfig>, key: &str, expected: bool) {
    assert_eq!(
        checks
            .and_then(|checks| checks.extra.get(key))
            .and_then(Value::as_bool),
        Some(expected),
        "check extra bool mismatch",
    );
}

pub fn assert_check_extra_table(checks: Option<&RustChecksConfig>, key: &str) {
    assert!(
        checks
            .and_then(|checks| checks.extra.get(key))
            .is_some_and(Value::is_table),
        "{key} should be preserved as an extra table",
    );
}

#[must_use]
pub fn assert_ts_astro_policy(ts: Option<&TsPolicyConfig>) -> &TsAstroPolicyConfig {
    let ts = ts.expect("ts policy should exist");
    ts.astro.as_ref().expect("ts.astro policy should exist")
}

pub fn assert_ts_extra_string(ts: Option<&TsPolicyConfig>, key: &str, expected: &str) {
    assert_eq!(
        ts.and_then(|ts| ts.extra.get(key)).and_then(Value::as_str),
        Some(expected),
        "ts extra value mismatch",
    );
}

pub fn assert_ts_astro_profile(astro: &TsAstroPolicyConfig, expected: Option<&str>) {
    assert_eq!(
        astro.profile.as_deref(),
        expected,
        "ts.astro profile mismatch"
    );
}

pub fn assert_ts_astro_extra_string(astro: &TsAstroPolicyConfig, key: &str, expected: &str) {
    assert_eq!(
        astro.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "ts.astro extra value mismatch",
    );
}

pub fn assert_ts_astro_routes_extra_string(astro: &TsAstroPolicyConfig, key: &str, expected: &str) {
    assert_eq!(
        astro.routes.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "ts.astro.routes extra value mismatch",
    );
}

pub fn assert_ts_astro_content_extra_string(
    astro: &TsAstroPolicyConfig,
    key: &str,
    expected: &str,
) {
    assert_eq!(
        astro.content.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "ts.astro.content extra value mismatch",
    );
}

pub fn assert_ts_astro_mdx_extra_string(astro: &TsAstroPolicyConfig, key: &str, expected: &str) {
    assert_eq!(
        astro.mdx.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "ts.astro.mdx extra value mismatch",
    );
}

pub fn assert_ts_astro_seo_extra_string(astro: &TsAstroPolicyConfig, key: &str, expected: &str) {
    assert_eq!(
        astro.seo.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "ts.astro.seo extra value mismatch",
    );
}

pub fn assert_ts_astro_state_extra_string(astro: &TsAstroPolicyConfig, key: &str, expected: &str) {
    assert_eq!(
        astro.state.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "ts.astro.state extra value mismatch",
    );
}

pub fn assert_waiver(
    waiver: Option<&WaiverConfig>,
    expected_rule: &str,
    expected_file: &str,
    expected_selector: &str,
    expected_reason: &str,
) {
    let waiver = waiver.expect("waiver should exist");
    assert_eq!(waiver.rule, expected_rule, "waiver rule mismatch");
    assert_eq!(waiver.file, expected_file, "waiver file mismatch");
    assert_eq!(
        waiver.selector, expected_selector,
        "waiver selector mismatch"
    );
    assert_eq!(waiver.reason, expected_reason, "waiver reason mismatch");
}

pub fn assert_waiver_extra_string(waiver: Option<&WaiverConfig>, key: &str, expected: &str) {
    assert_eq!(
        waiver
            .and_then(|waiver| waiver.extra.get(key))
            .and_then(Value::as_str),
        Some(expected),
        "waiver extra value mismatch",
    );
}

pub fn assert_top_level_string_extra(cfg: &Guardrail3RsToml, key: &str, expected: &str) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "top-level extra key should be preserved",
    );
}

pub fn assert_tomls_equal(left: &Guardrail3RsToml, right: &Guardrail3RsToml) {
    assert_eq!(left, right, "roundtrip should produce identical config");
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid guardrail3-rs.toml"),
        "expected error message prefix, got: {msg}",
    );
}
