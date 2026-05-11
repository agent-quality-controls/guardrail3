#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::panic,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use g3ts_toml_parser_runtime::types::{
    Guardrail3TsToml, TsAstroPolicyConfig, TsChecksConfig, TsStylePolicyConfig, Value,
};

pub fn assert_core_fields_empty(cfg: &Guardrail3TsToml) {
    assert_eq!(cfg.version, None, "version should be None for empty input");
    assert_eq!(cfg.checks, None, "checks should be None for empty input");
    assert_eq!(cfg.astro, None, "astro should be None for empty input");
    assert_eq!(cfg.style, None, "style should be None for empty input");
}

pub fn assert_extra_empty(cfg: &Guardrail3TsToml) {
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

/// Assert a single `Option<bool>` check-field matches `expected`, labeled by `label` on failure.
fn assert_check_field(label: &str, actual: Option<bool>, expected: Option<bool>) {
    assert_eq!(actual, expected, "{label} mismatch");
}

pub fn assert_eslint_check(checks: Option<&TsChecksConfig>, expected: Option<bool>) {
    assert_check_field("eslint", checks.and_then(|c| c.eslint), expected);
}

pub fn assert_style_check(checks: Option<&TsChecksConfig>, expected: Option<bool>) {
    assert_check_field("style", checks.and_then(|c| c.style), expected);
}

pub fn assert_check_extra_string(checks: Option<&TsChecksConfig>, key: &str, expected: &str) {
    assert_eq!(
        checks
            .and_then(|c| c.extra.get(key))
            .and_then(Value::as_str),
        Some(expected),
        "check extra value mismatch",
    );
}

pub fn assert_string_list(actual: &[String], expected: &[&str], field_name: &str) {
    let expected_values = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(actual, expected_values, "{field_name} mismatch");
}

#[must_use]
pub fn assert_style_policy(style: Option<&TsStylePolicyConfig>) -> &TsStylePolicyConfig {
    style.expect("style policy should exist")
}

#[must_use]
pub fn assert_astro_policy(astro: Option<&TsAstroPolicyConfig>) -> &TsAstroPolicyConfig {
    astro.expect("astro policy should exist")
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid guardrail3-ts.toml"),
        "expected error message prefix, got: {msg}",
    );
}
