use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::Error;

/// Parsed representation of a `.config/nextest.toml` configuration file.
///
/// All known nextest configuration keys are mapped to typed fields.
/// Profile-specific settings live under [`profile`](Self::profile).
/// Unknown top-level keys are captured in [`extra`](Self::extra) for forward
/// compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NextestConfig {
    /// Named test profiles (e.g., `default`, `ci`).
    #[serde(default)]
    pub profile: BTreeMap<String, NextestProfile>,

    /// Unknown top-level keys, preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Configuration for a single nextest profile.
///
/// Profiles define test execution behaviour — timeouts, threading, retries,
/// and failure modes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NextestProfile {
    /// Time before a test is considered slow.
    slow_timeout: Option<TimeoutConfig>,

    /// Time to wait for leaked handles after a test completes.
    leak_timeout: Option<TimeoutConfig>,

    /// Number of test threads (integer or string like `"num-cpus"`).
    test_threads: Option<Value>,

    /// Retry count (integer or structured retry config).
    retries: Option<Value>,

    /// Whether to stop on first failure.
    fail_fast: Option<bool>,

    /// Unknown profile keys, preserved for forward compatibility.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl NextestProfile {
    /// Time before a test is considered slow, if configured.
    #[must_use]
    pub const fn slow_timeout(&self) -> Option<&TimeoutConfig> {
        self.slow_timeout.as_ref()
    }

    /// Time to wait for leaked handles after a test completes, if configured.
    #[must_use]
    pub const fn leak_timeout(&self) -> Option<&TimeoutConfig> {
        self.leak_timeout.as_ref()
    }

    /// Number of test threads (integer or string like `"num-cpus"`), if configured.
    #[must_use]
    pub const fn test_threads(&self) -> Option<&Value> {
        self.test_threads.as_ref()
    }

    /// Retry count (integer or structured retry config), if configured.
    #[must_use]
    pub const fn retries(&self) -> Option<&Value> {
        self.retries.as_ref()
    }

    /// Whether to stop on first failure, if configured.
    #[must_use]
    pub const fn fail_fast(&self) -> Option<bool> {
        self.fail_fast
    }

    /// Unknown profile keys, preserved for forward compatibility.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

/// Timeout configuration supporting both simple and detailed forms.
///
/// Simple: `"60s"` — a bare duration string.
/// Detailed: `{ period = "60s", terminate-after = 2 }` — duration with
/// termination multiplier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TimeoutConfig {
    /// Bare duration string (e.g., `"60s"`).
    Simple(String),
    /// Table with period and optional terminate-after.
    Detailed(TimeoutDetail),
}

/// Detailed timeout configuration with period and termination multiplier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TimeoutDetail {
    /// Duration string (e.g., `"60s"`, `"2m"`).
    period: String,
    /// Number of periods after which to send SIGTERM/SIGKILL.
    terminate_after: Option<u32>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl TimeoutDetail {
    /// Duration string (e.g., `"60s"`, `"2m"`).
    #[must_use]
    pub fn period(&self) -> &str {
        &self.period
    }

    /// Number of periods after which to send SIGTERM/SIGKILL, if configured.
    #[must_use]
    pub const fn terminate_after(&self) -> Option<u32> {
        self.terminate_after
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

impl NextestConfig {
    /// Read and parse a nextest.toml file from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`] on read failure, [`Error::Toml`] on parse failure.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let content = crate::fs::read_to_string(path)?;
        content.parse()
    }
}

impl std::str::FromStr for NextestConfig {
    type Err = Error;

    #[allow(clippy::disallowed_methods)] // reason: this crate IS the centralized nextest.toml parser — toml::from_str is its core purpose
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to parse a string into `NextestConfig`, panicking on failure.
    fn parse(input: &str) -> NextestConfig {
        input.parse::<NextestConfig>().expect("should parse valid nextest.toml")
    }

    #[test]
    fn empty_string_yields_empty_config() {
        let cfg = parse("");

        assert!(cfg.profile.is_empty(), "profile map should be empty");
        assert!(cfg.extra.is_empty(), "extra should be empty");
    }

    #[test]
    fn single_profile_with_simple_timeouts() {
        let cfg = parse(r#"
[profile.default]
slow-timeout = "60s"
leak-timeout = "100ms"
"#);

        assert_eq!(cfg.profile.len(), 1, "should have 1 profile");
        let default = cfg.profile.get("default").expect("should have 'default' profile");

        match default.slow_timeout() {
            Some(TimeoutConfig::Simple(s)) => assert_eq!(s, "60s", "slow_timeout value"),
            other => panic!("expected Simple timeout, got: {other:?}"),
        }
        match default.leak_timeout() {
            Some(TimeoutConfig::Simple(s)) => assert_eq!(s, "100ms", "leak_timeout value"),
            other => panic!("expected Simple timeout, got: {other:?}"),
        }
    }

    #[test]
    fn detailed_timeout_with_terminate_after() {
        let cfg = parse(r#"
[profile.default]
slow-timeout = { period = "60s", terminate-after = 2 }
"#);

        let default = cfg.profile.get("default").expect("should have 'default' profile");

        match default.slow_timeout() {
            Some(TimeoutConfig::Detailed(detail)) => {
                assert_eq!(detail.period(), "60s", "period value");
                assert_eq!(detail.terminate_after(), Some(2), "terminate_after value");
                assert!(detail.extra().is_empty(), "no extra fields expected");
            }
            other => panic!("expected Detailed timeout, got: {other:?}"),
        }
    }

    #[test]
    fn profile_with_all_known_fields() {
        let cfg = parse(r#"
[profile.ci]
slow-timeout = { period = "120s", terminate-after = 3 }
leak-timeout = "500ms"
test-threads = 4
retries = 2
fail-fast = false
"#);

        let ci = cfg.profile.get("ci").expect("should have 'ci' profile");

        assert!(ci.slow_timeout().is_some(), "slow_timeout should be present");
        assert!(ci.leak_timeout().is_some(), "leak_timeout should be present");
        assert!(ci.test_threads().is_some(), "test_threads should be present");
        assert!(ci.retries().is_some(), "retries should be present");
        assert_eq!(ci.fail_fast(), Some(false), "fail_fast should be false");
        assert!(ci.extra().is_empty(), "all keys should be known");
    }

    #[test]
    fn multiple_profiles() {
        let cfg = parse(r#"
[profile.default]
slow-timeout = "60s"

[profile.ci]
slow-timeout = "120s"
fail-fast = true
"#);

        assert_eq!(cfg.profile.len(), 2, "should have 2 profiles");
        assert!(cfg.profile.contains_key("default"), "should have 'default'");
        assert!(cfg.profile.contains_key("ci"), "should have 'ci'");
    }

    #[test]
    fn unknown_top_level_keys_land_in_extra() {
        let cfg = parse(r#"
some-future-option = "yes"

[profile.default]
slow-timeout = "60s"
"#);

        assert_eq!(cfg.extra.len(), 1, "should capture 1 unknown top-level key");
        assert_eq!(
            cfg.extra.get("some-future-option").and_then(Value::as_str),
            Some("yes"),
            "unknown key value should be captured",
        );
    }

    #[test]
    fn unknown_profile_keys_land_in_profile_extra() {
        let cfg = parse(r#"
[profile.default]
slow-timeout = "60s"
some-new-nextest-option = true
"#);

        let default = cfg.profile.get("default").expect("should have 'default' profile");
        assert_eq!(default.extra().len(), 1, "should capture 1 unknown profile key");
        assert_eq!(
            default.extra().get("some-new-nextest-option").and_then(Value::as_bool),
            Some(true),
            "unknown profile key should be captured",
        );
    }

    #[test]
    fn test_threads_as_string() {
        let cfg = parse(r#"
[profile.default]
test-threads = "num-cpus"
"#);

        let default = cfg.profile.get("default").expect("should have 'default' profile");
        assert!(default.test_threads().is_some(), "test_threads should be present");
        assert_eq!(
            default.test_threads().and_then(Value::as_str),
            Some("num-cpus"),
            "test_threads string value",
        );
    }

    #[test]
    fn real_nextest_config_parses() {
        let cfg = parse(r#"
[profile.default]
slow-timeout = { period = "60s", terminate-after = 2 }
leak-timeout = "100ms"
test-threads = "num-cpus"
retries = 0
fail-fast = true

[profile.ci]
slow-timeout = { period = "120s", terminate-after = 3 }
leak-timeout = "500ms"
retries = 2
fail-fast = false
"#);

        assert_eq!(cfg.profile.len(), 2, "should have 2 profiles");

        let default = cfg.profile.get("default").expect("should have 'default' profile");
        assert!(default.slow_timeout().is_some(), "default slow_timeout");
        assert!(default.leak_timeout().is_some(), "default leak_timeout");
        assert_eq!(default.fail_fast(), Some(true), "default fail_fast");
        assert!(default.extra().is_empty(), "default: all keys should be known");

        let ci = cfg.profile.get("ci").expect("should have 'ci' profile");
        assert!(ci.slow_timeout().is_some(), "ci slow_timeout");
        assert_eq!(ci.fail_fast(), Some(false), "ci fail_fast");
        assert!(ci.extra().is_empty(), "ci: all keys should be known");
    }

    #[test]
    fn from_str_error_on_invalid_toml() {
        let bad = "this is not [[[valid toml";
        let err = bad.parse::<NextestConfig>();
        assert!(err.is_err(), "invalid TOML should produce an error");

        let msg = err.expect_err("should be an error").to_string();
        assert!(
            msg.contains("invalid nextest.toml"),
            "expected error message prefix, got: {msg}",
        );
    }
}
