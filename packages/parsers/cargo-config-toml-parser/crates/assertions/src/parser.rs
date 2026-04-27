#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use std::collections::BTreeMap;

use cargo_config_toml_parser_runtime::types::{
    CargoConfigToml, CommandValue, EnvValue, HttpSslVersion, TargetSelector, Value,
};

pub fn assert_core_fields_empty(cfg: &CargoConfigToml) {
    assert!(
        cfg.paths.is_empty(),
        "paths should be empty for empty input"
    );
    assert!(
        cfg.include.is_empty(),
        "include should be empty for empty input"
    );
    assert!(
        cfg.alias.is_empty(),
        "alias should be empty for empty input"
    );
    assert_eq!(cfg.build, None, "build should be None for empty input");
    assert!(cfg.env.is_empty(), "env should be empty for empty input");
    assert!(
        cfg.target.is_empty(),
        "target should be empty for empty input"
    );
}

pub fn assert_extra_empty(cfg: &CargoConfigToml) {
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

pub fn assert_simple_env_value(env: Option<&EnvValue>, expected: &str, key: &str) {
    assert!(env.is_some(), "{key} should exist");
    assert!(
        matches!(env, Some(EnvValue::Simple(_))),
        "{key} should be a simple env value",
    );
    let Some(EnvValue::Simple(actual)) = env else {
        return;
    };
    assert_eq!(actual, expected, "{key} mismatch");
}

pub fn assert_detailed_env_value(
    env: Option<&EnvValue>,
    expected_value: &str,
    expected_force: Option<bool>,
    expected_relative: Option<bool>,
    key: &str,
) {
    assert!(env.is_some(), "{key} should exist");
    assert!(
        matches!(env, Some(EnvValue::Detailed(_))),
        "{key} should be a detailed env value",
    );
    let Some(EnvValue::Detailed(detail)) = env else {
        return;
    };
    assert_eq!(detail.value, expected_value, "{key}.value mismatch");
    assert_eq!(detail.force, expected_force, "{key}.force mismatch");
    assert_eq!(
        detail.relative, expected_relative,
        "{key}.relative mismatch"
    );
    assert!(detail.extra.is_empty(), "{key}.extra should be empty");
}

pub fn assert_detailed_env_extra_table(env: Option<&EnvValue>, key: &str, extra_key: &str) {
    assert!(env.is_some(), "{key} should exist");
    assert!(
        matches!(env, Some(EnvValue::Detailed(_))),
        "{key} should be a detailed env value",
    );
    let Some(EnvValue::Detailed(detail)) = env else {
        return;
    };
    assert_nested_extra_table(&detail.extra, extra_key);
}

pub fn assert_command_list(actual: Option<&CommandValue>, expected: &[&str], field_name: &str) {
    let expected_values = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert!(actual.is_some(), "{field_name} should exist");
    assert!(
        matches!(actual, Some(CommandValue::List(_))),
        "{field_name} should be a list command",
    );
    let Some(CommandValue::List(value)) = actual else {
        return;
    };
    assert_eq!(value, &expected_values, "{field_name} mismatch");
}

pub fn assert_target_selector_list(
    actual: Option<&TargetSelector>,
    expected: &[&str],
    field_name: &str,
) {
    let expected_values = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert!(actual.is_some(), "{field_name} should exist");
    assert!(
        matches!(actual, Some(TargetSelector::List(_))),
        "{field_name} should be a list selector",
    );
    let Some(TargetSelector::List(value)) = actual else {
        return;
    };
    assert_eq!(value, &expected_values, "{field_name} mismatch");
}

pub fn assert_tls_range(
    actual: Option<&HttpSslVersion>,
    expected_min: Option<&str>,
    expected_max: Option<&str>,
) {
    assert!(actual.is_some(), "http.ssl-version should exist");
    assert!(
        matches!(actual, Some(HttpSslVersion::Range(_))),
        "http.ssl-version should be a range",
    );
    let Some(HttpSslVersion::Range(range)) = actual else {
        return;
    };
    assert_eq!(
        range.min.as_deref(),
        expected_min,
        "http.ssl-version.min mismatch"
    );
    assert_eq!(
        range.max.as_deref(),
        expected_max,
        "http.ssl-version.max mismatch"
    );
    assert!(
        range.extra.is_empty(),
        "http.ssl-version.extra should be empty"
    );
}

pub fn assert_string_list(actual: &[String], expected: &[&str], field_name: &str) {
    let expected_values = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(actual, expected_values, "{field_name} mismatch");
}

pub fn assert_top_level_string_extra(cfg: &CargoConfigToml, key: &str, expected: &str) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "top-level extra key should be preserved",
    );
}

pub fn assert_top_level_integer_extra(cfg: &CargoConfigToml, key: &str, expected: i64) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_integer),
        Some(expected),
        "top-level extra key should be preserved",
    );
}

pub fn assert_nested_extra_table(extra: &BTreeMap<String, Value>, key: &str) {
    assert!(
        extra.get(key).is_some_and(Value::is_table),
        "{key} should be preserved as an extra table",
    );
}

pub fn assert_tomls_equal(left: &CargoConfigToml, right: &CargoConfigToml) {
    assert_eq!(left, right, "roundtrip should produce identical config");
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid cargo config"),
        "expected error message prefix, got: {msg}",
    );
}

pub fn assert_include_path_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("expected a config include path ending with `.toml`"),
        "expected include path error, got: {msg}",
    );
}
