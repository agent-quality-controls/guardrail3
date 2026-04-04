use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Parsed representation of a `.config/nextest.toml` configuration file.
///
/// All known nextest configuration keys are mapped to typed fields.
/// Profile-specific settings live under [`profile`](Self::profile).
/// Unknown top-level keys are captured in [`extra`](Self::extra) for forward
/// compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct NextestToml {
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
