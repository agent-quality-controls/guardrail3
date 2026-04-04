#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::panic,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use guardrail3_rs_toml_parser_runtime::{
    Guardrail3RsToml, RustChecksConfig, RustProfile, Value, WaiverConfig,
};

pub fn assert_core_fields_empty(cfg: &Guardrail3RsToml) {
    assert_eq!(cfg.version, None, "version should be None for empty input");
    assert_eq!(cfg.profile, None, "profile should be None for empty input");
    assert!(cfg.excluded_paths.is_empty(), "excluded_paths should be empty");
    assert!(cfg.allowed_deps.is_empty(), "allowed_deps should be empty");
    assert_eq!(cfg.checks, None, "checks should be None for empty input");
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

pub fn assert_check_value(checks: Option<&RustChecksConfig>, key: &str, expected: Option<bool>) {
    let actual = checks.and_then(|checks| match key {
        "topology" => checks.topology,
        "arch" => checks.arch,
        "fmt" => checks.fmt,
        "toolchain" => checks.toolchain,
        "clippy" => checks.clippy,
        "deny" => checks.deny,
        "cargo" => checks.cargo,
        "code" => checks.code,
        "hexarch" => checks.hexarch,
        "deps" => checks.deps,
        "garde" => checks.garde,
        "test" => checks.test,
        "release" => checks.release,
        "hooks_shared" => checks.hooks_shared,
        "hooks_rs" => checks.hooks_rs,
        _ => panic!("unexpected check key: {key}"),
    });
    assert_eq!(actual, expected, "{key} mismatch");
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

pub fn assert_check_extra_table(checks: Option<&RustChecksConfig>, key: &str) {
    assert!(
        checks
            .and_then(|checks| checks.extra.get(key))
            .is_some_and(Value::is_table),
        "{key} should be preserved as an extra table",
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
    assert_eq!(waiver.selector, expected_selector, "waiver selector mismatch");
    assert_eq!(waiver.reason, expected_reason, "waiver reason mismatch");
}

pub fn assert_waiver_extra_string(
    waiver: Option<&WaiverConfig>,
    key: &str,
    expected: &str,
) {
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
