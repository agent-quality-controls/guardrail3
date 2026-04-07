#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use nextest_toml_parser_runtime::{
    NextestProfile, NextestToml, TestThreads, ThreadsRequired, TimeoutConfig, Value,
};

pub fn assert_empty_toml(cfg: &NextestToml) {
    assert!(cfg.profile.is_empty(), "profile map should be empty");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

pub fn assert_profile_len(cfg: &NextestToml, expected: usize) {
    assert_eq!(cfg.profile.len(), expected, "profile map length mismatch");
}

pub fn assert_profile_extra_empty(profile: &NextestProfile) {
    assert!(profile.extra.is_empty(), "profile extra should be empty");
}

pub fn assert_top_level_extra_string(cfg: &NextestToml, key: &str, expected: &str) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_str),
        Some(expected),
        "top-level extra key should be preserved",
    );
}

pub fn assert_profile_extra_bool(profile: &NextestProfile, key: &str, expected: bool) {
    assert_eq!(
        profile.extra.get(key).and_then(Value::as_bool),
        Some(expected),
        "profile extra key should be preserved",
    );
}

pub fn assert_test_threads(actual: Option<TestThreads>, expected: TestThreads) {
    assert_eq!(actual, Some(expected), "test-threads mismatch");
}

pub fn assert_threads_required(actual: Option<ThreadsRequired>, expected: ThreadsRequired) {
    assert_eq!(actual, Some(expected), "threads-required mismatch");
}

pub fn assert_simple_timeout(actual: Option<&TimeoutConfig>, expected: &str, field_name: &str) {
    assert!(
        matches!(actual, Some(TimeoutConfig::Simple(s)) if s == expected),
        "expected Simple timeout for {field_name}, got: {actual:?}",
    );
}

pub fn assert_detailed_timeout(
    actual: Option<&TimeoutConfig>,
    period: &str,
    terminate_after: Option<u32>,
) {
    assert!(
        matches!(actual, Some(TimeoutConfig::Detailed(detail))
            if detail.period == period
                && detail.terminate_after == terminate_after
                && detail.extra.is_empty()),
        "expected Detailed timeout, got: {actual:?}",
    );
}

pub fn assert_parse_error_message(message: &str) {
    assert!(
        message.contains("invalid nextest.toml"),
        "expected error message prefix, got: {message}",
    );
}

/// Parse nextest TOML content through the runtime parser.
///
/// # Errors
///
/// Returns the parser error when the input is not valid nextest TOML.
pub fn parse_error(input: &str) -> Result<NextestToml, nextest_toml_parser_runtime::Error> {
    nextest_toml_parser_runtime::parse(input)
}
